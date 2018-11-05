//! # butlerd
//! Interfaces with itch.io's [butlerd](https://github.com/itchio/butler).
//! It provides methods for almost every API call that can be made.
//! Right now, replying to events from butler isn't implemented yet. This means that you can't log
//! in using an username/password if a captcha gets triggered or if a 2factor token is required.
//! Currently working on fixing this, but I'm still not very certain as to how this should be
//! implemented. Open to suggestions.
extern crate reqwest;
use std::process::Command;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate hyper;
extern crate rand;
extern crate regex;
extern crate serde;
use regex::Regex;
use reqwest::Method;
use serde::de::DeserializeOwned;
use std::env;
use std::io::Read;
use std::result::Result::*;
pub mod Responses;
use hyper::Client;
use serde_json::value::Map;
use std::collections::HashMap;
use std::fs;
use Responses::*;
#[cfg(target_os = "macos")]
static DB_PATH: &str = "~/Library/Application Support/itch/db/butler.db";
#[cfg(target_os = "linux")]
static DB_PATH: &str = "~/.config/itch/db/butler.db";
#[cfg(target_os = "windows")]
static DB_PATH: &str = "%APPDATA%/itch/db/butler.db";
#[cfg(target_os = "macos")]
static LOG_PATH_PRE: &str = "/tmp/butlerdrs";
#[cfg(target_os = "linux")]
static LOG_PATH_PRE: &str = "/tmp/butlerdrs";
#[cfg(target_os = "windows")]
static LOG_PATH_PRE: &str = "%TEMP%/butlerdrs";
#[cfg(target_os = "macos")]
static PRE_PATH: &str = "~/Library/Application Support/itch/prereqs";
#[cfg(target_os = "linux")]
static PRE_PATH: &str = "~/.config/itch/prereqs";
#[cfg(target_os = "windows")]
static PRE_PATH: &str = "%APPDATA%/itch/db/butler.db";
const POST: Method = Method::POST;
/// Represents a connection to a butlerd instance
pub struct Butler {
    secret: String,
    address: String,
    client: reqwest::Client,
    pre_dir: String,
    client_launch: reqwest::Client,
    hclient: Client<hyper::client::HttpConnector, hyper::Body>,
}
impl Butler {
    /// Initializes a new butlerd instance. It will close when your program does.
    pub fn new() -> Result<Butler, String> {
        let mut db_proc = DB_PATH.replace("~", &get_home());
        let appdata = env::var("APPDATA");
        if appdata.is_ok() {
            db_proc = db_proc.replace("%APPDATA%", &appdata.unwrap().to_string());
        }
        let mut log_path_proc = LOG_PATH_PRE.to_string();
        let temp = env::var("TEMP");
        if temp.is_ok() {
            log_path_proc = log_path_proc.replace("%TEMP%", &temp.unwrap().to_string());
        }
        let log_path = &(log_path_proc + &rand::random::<f64>().to_string() + ".log");
        let mut file: fs::File;
        if fs::remove_file(log_path).is_ok() {
            file = fs::File::create(log_path).unwrap();
        }
        if fs::metadata(log_path).is_ok() {
            if fs::remove_file(log_path).is_err() {
                file = fs::File::create(log_path).unwrap();
            } else {
                file = fs::File::open(log_path).unwrap();
            }
        } else {
            file = fs::File::create(log_path).unwrap();
        }
        Command::new("sh")
            .arg("-c")
            .arg(
                "butler daemon --json --dbpath=".to_string()
                    + &db_proc
                    + " --destiny-pid="
                    + &::std::process::id().to_string(),
            )
            .stdout(file)
            .spawn()
            .expect("Couldn't start butler daemon");
        //TODO: REPLACE
        let mut finish = false;
        let mut bd: String = String::new();
        let reg = Regex::new(r"\{(?:.|\s)+\}").unwrap();
        while !finish {
            bd = String::new();
            fs::File::open(log_path)
                .unwrap()
                .read_to_string(&mut bd)
                .unwrap();
            let res = reg.find(&bd);
            if res.is_some() {
                finish = true;
            } else {
                ::std::thread::sleep_ms(250);
            }
        }
        bd = reg.find(&bd).unwrap().as_str().to_string();
        bd = bd.replace("\\\"", "");
        let mut lines = bd.lines();
        let mut done = false;
        let mut pmeta = BStart {
            http: Map::new(),
            https: Map::new(),
            secret: String::new(),
        };
        while !done {
            let ltry = lines.next();
            if ltry.is_some() {
                let try = ltry.unwrap().to_string();
                let td = serde_json::from_str(&try.trim());
                if td.is_ok() {
                    pmeta = td.unwrap();
                    done = true;
                }
            } else {
                fs::remove_file(log_path).expect("Couldn't remove log file early");
                return Err("Couldn't get butler startup".to_string());
                // break;
            }
        }
        if done {
            let but = Butler::from_start(pmeta);
            fs::remove_file(log_path).expect("Couldn't remove log file");
            Ok(but)
        } else {
            Err("Couldn't start butler".to_string())
        }
    }
    /// Builds a Butler from a startup message. Useful if you want to start the daemon yourself or
    /// maintain your own daemon with multiple connections.
    pub fn from_start(pmeta: BStart) -> Butler {
        let secret = pmeta.secret.to_string();
        let mut pre_path_proc = PRE_PATH.to_string();
        let appdata = env::var("APPDATA");
        if appdata.is_ok() {
            pre_path_proc = pre_path_proc.replace("%APPDATA%", &appdata.unwrap().to_string());
        }
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-Secret", secret.parse().unwrap());
        headers.insert("X-ID", "0".parse().unwrap());
        let mut client = reqwest::Client::builder();
        let mut client_launch = reqwest::Client::builder();
        client_launch = client_launch.default_headers(headers.clone());
        client = client.default_headers(headers);
        client_launch = client_launch.timeout(None);
        let built = client.build().unwrap();
        let builtl = client_launch.build().unwrap();
        Butler {
            secret: secret,
            address: pmeta.http[&"address".to_string()]
                .to_string()
                .replace("\"", ""),
            client: built,
            pre_dir: pre_path_proc.replace("~", &get_home()),
            client_launch: builtl,
            hclient: Client::new(),
        }
    }
    ///Shuts down butler daemon.
    pub fn close(&self) {
        self.request("/Meta.Shutdown", "{}".to_string())
            .expect("Couldn't shut down butler daemon");;
    }
    fn make_request<N>(
        &self,
        method: Method,
        path: N,
        params: N,
        client: &str,
    ) -> Result<String, String>
    where
        N: Into<String>,
    {
        let res: Result<reqwest::Response, reqwest::Error>;
        let url = "http://".to_string() + &self.address.clone() + &path.into();
        if client == "launch" {
            res = self
                .client_launch
                .request(method.into(), &url)
                .body(params.into())
                .send();
        } else {
            res = self
                .client
                .request(method.into(), &url)
                .body(params.into())
                .send();
        }
        if res.is_ok() {
            let mut res = res.unwrap();
            if res.status().is_success() {
                Ok(res.text().unwrap())
            } else {
                Err("No".to_string())
            }
        } else {
            Err("Timed out".to_string())
        }
    }
    fn request<S, N>(&self, path: S, params: N) -> Result<String, String>
    where
        S: Into<String>,
        N: Into<String>,
    {
        self.make_request(POST, path.into(), params.into(), "default")
    }
    fn request_l<S, N>(&self, path: S, params: N) -> Result<String, String>
    where
        S: Into<String>,
        N: Into<String>,
    {
        self.make_request(POST, path.into(), params.into(), "launch")
    }
    /// Fetchs all installed caves
    pub fn fetchall(&self) -> Result<Vec<Cave>, BError> {
        self.res_preq("/call/Fetch.Caves", vec![], "items")
    }
    ///Fetches specific game by id
    pub fn fetch_game(&self, game_id: i32) -> Result<Game, BError> {
        let gvs = self
            .request("/call/Fetch.Game", json!({ "gameId": game_id }).to_string())
            .expect("Couldn't fetch game by id");
        parse_r(gvs, "game")
    }
    ///Fetches specific cave by id
    pub fn fetch_cave<N>(&self, cave_id: N) -> Result<Cave, BError>
    where
        N: Into<String>,
    {
        self.res_preq(
            "/call/Fetch.Cave",
            vec![("caveId", &cave_id.into())],
            "cave",
        )
    }
    /// Makes a cave 'pinned' or not depending on pinned
    pub fn pin_cave<N>(&self, cave_id: N, pinned: bool)
    where
        N: Into<String>,
    {
        self.request(
            "/call/Caves.SetPinned",
            json!({
            "caveId":cave_id.into(),
            "pinned":pinned
        })
            .to_string(),
        )
        .expect("Couldn't pin cave");
    }
    /// Launches game by CaveID. Note that this will not complete until the game is closed.
    pub fn launch_game<N>(&self, cave_id: N)
    where
        N: Into<String>,
    {
        self.request_l(
            "/call/Launch",
            json!({
                "caveId": cave_id.into(),
                "prereqsDir": self.pre_dir.clone()
            })
            .to_string(),
        )
        .expect("Couldn't launch game");
    }
    /// Lists saved profiles
    pub fn profile_list(&self) -> Result<Vec<Profile>, BError> {
        self.res_preq("/call/Profile.List", vec![], "profiles")
    }
    /// Sets a specific profile info value
    pub fn profile_put<N>(&self, profile_id: i32, key: N, value: N)
    where
        N: Into<String>,
    {
        self.request(
            "/call/Profile.Data.Put",
            json!({
                "profileId":profile_id,
                "key":key.into(),
                "value":value.into()
            })
            .to_string(),
        )
        .expect("Couldn't put profile data");
    }
    /// Searches for folders possible to clean
    pub fn clean_search(
        &self,
        roots: Vec<String>,
        whitelist: Vec<String>,
    ) -> Result<Vec<CleanDownloadsEntry>, BError> {
        let cis = self
            .request(
                "/call/CleanDownloads.Search",
                json!({
            "roots":roots,
            "whitelist":whitelist
        })
                .to_string(),
            )
            .expect("Couldn't search for folders to clean");
        parse_r(cis, "entries")
    }
    /// Cleans specified CleanDownloadsEntries
    pub fn clean_apply(&self, entries: Vec<CleanDownloadsEntry>) {
        self.request(
            "/call/CleanDownloads.Apply",
            json!({ "entries": entries }).to_string(),
        )
        .expect("Couldn't apply downloads clean");
    }
    /// Gets a specific profile info value
    pub fn profile_get<N>(&self, profile_id: i32, key: N) -> Result<String, BError>
    where
        N: Into<String>,
    {
        let gis = self
            .request(
                "/call/Profile.Data.Get",
                json!({
            "profileId":profile_id,
            "key":key.into()
        })
                .to_string(),
            )
            .expect("Couldn't get profile data value");
        parse_r(gis, "value")
    }
    /// Removes a profile's saved info. Also removes it from profile_list. Returns true if
    /// successful.
    pub fn profile_forget(&self, profile_id: i32) -> Result<bool, BError> {
        let sis = self
            .request(
                "/call/Profile.Forget",
                json!({ "profileId": profile_id }).to_string(),
            )
            .expect("Couldn't forget profile");
        parse_r(sis, "success")
    }
    /// Disables updates for a cave
    pub fn snooze_cave<N>(&self, cave_id: N) where N : Into<String> {
        self.request(
            "/call/Snooze.Cave",
            json!({ "caveId": cave_id.into() }).to_string(),
        )
        .expect("Couldn't snooze cave");
    }
    /// Logs into a profile using saved credentials
    pub fn login_saved(&self, profile_id: i32) -> Result<Profile, BError> {
        let pis = self
            .request(
                "/call/Profile.UseSavedLogin",
                json!({ "profileId": profile_id }).to_string(),
            )
            .expect("Couldn't login using saved credentials");
        parse_r(pis, "profile")
    }
    /// Given an API key, logs into a profile and returns profile.
    pub fn login_api_key<N>(&self, api_key: N) -> Result<Profile, BError>
    where
        N: Into<String>,
    {
        self.res_preq(
            "/call/Profile.LoginWithAPIKey",
            vec![("apiKey", &api_key.into())],
            "profile",
        )
    }
    /// Given an username and password, logs into a profile and returns profile and cookie. May
    /// fail if a captcha or 2factor is required. Working on fix.
    pub fn login_password<N>(&self, username: N, password: N) -> Result<PassLogRes, BError>
    where
        N: Into<String>,
    {
        self.res_req(
            "/call/Profile.LoginWithPassword",
            vec![
                ("username", &username.into()),
                ("password", &password.into()),
            ],
        )
        .unwrap()
    }
    /// Fetches all common/cached items and returns summaries
    pub fn fetch_commons(&self) -> Result<Commons, BError> {
        self.res_req("/call/Fetch.Commons", vec![])
    }
    /// Fetches a vec of games owned by a specific profile id
    pub fn fetch_profile_games(&self, profile_id: i32) -> Result<Vec<ProfileGame>, BError> {
        let pvs = self
            .request(
                "/call/Fetch.ProfileGames",
                json!({
                "profileId": profile_id,
            })
                .to_string(),
            )
            .expect("Couldn't fetch profile games");
        parse_r(pvs, "items")
    }
    /// Fetches download key
    pub fn fetch_download_key(
        &self,
        profile_id: i32,
        download_key_id: i32,
        fresh: bool,
    ) -> Result<DownloadKey, BError> {
        let dis = self
            .request(
                "/call/Fetch.DownloadKey",
                json!({
            "profileId": profile_id,
            "downloadKeyId": download_key_id,
            "fresh":fresh
        })
                .to_string(),
            )
            .expect("Couldn't fetch download key");
        parse_r(dis, "downloadKey")
    }
    /// Fetches collection info. Does not include games
    pub fn fetch_collection(
        &self,
        profile_id: i32,
        collection_id: i32,
        fresh: bool,
    ) -> Result<Collection, BError> {
        let cis = self
            .request(
                "/call/Fetch.Collection",
                json!({
            "profileId":profile_id,
            "collectionId":collection_id,
            "fresh":fresh
        })
                .to_string(),
            )
            .expect("Couldn't fetch collection");
        parse_r(cis, "collection")
    }
    /// Fetches all collections for a profile. Does not include games
    pub fn fetch_profile_collections(
        &self,
        profile_id: i32,
        fresh: bool,
    ) -> Result<Vec<Collection>, BError> {
        let cis = self
            .request(
                "/call/Fetch.Collection",
                json!({
            "profileId": profile_id,
            "fresh": fresh
        })
                .to_string(),
            )
            .expect("Couldn't fetch profile collections");
        parse_r(cis, "items")
    }
    /// Fetches games in a collection
    pub fn fetch_collection_games(
        &self,
        profile_id: i32,
        collection_id: i32,
        fresh: bool,
    ) -> Result<Vec<CollectionGame>, BError> {
        let cis = self
            .request(
                "/call/Fetch.Collection.Games",
                json!({
            "profileId": profile_id,
            "collectionId": collection_id,
            "fresh":fresh
        })
                .to_string(),
            )
            .expect("Couldn't fetch collection games");
        parse_r(cis, "items")
    }
    /// Fetches owned download keys for a profile. Pass fresh as true to force butler to refresh
    /// cache
    pub fn fetch_profile_keys(
        &self,
        profile_id: i32,
        fresh: bool,
    ) -> Result<Vec<DownloadKey>, BError> {
        let dis = self
            .request(
                "/call/Fetch.ProfileOwnedKeys",
                json!({
            "profileId": profile_id,
            "fresh":fresh
        })
                .to_string(),
            )
            .expect("Couldn't fetch profile keys");
        parse_r(dis, "items")
    }
    /// Marks all local data as 'stale' and outdated
    pub fn expireall(&self) {
        self.req_h("Fetch.ExpireAll");
    }
    /// Searches users
    pub fn search_users<N>(&self, profile_id: i32, query: N) -> Result<Vec<User>, BError>
    where
        N: Into<String>,
    {
        let uis = self
            .request(
                "/call/Search.Users",
                json!({
            "profileId":profile_id,
            "query":query.into()
        })
                .to_string(),
            )
            .expect("Couldn't search users");
        parse_r(uis, "users")
    }
    fn req_h<N>(&self, path: N)
    where
        N: Into<String>,
    {
        let uri = "http://".to_string() + &self.address + "/call/" + &path.into();
        let mut builder = hyper::Request::builder();
        builder.method("POST");
        builder.header("X-Secret", self.secret.as_str());
        builder.header("X-ID", "0");
        builder.uri(uri);
        let request = builder.body(hyper::Body::empty()).unwrap();
        let _res = self.hclient.request(request);
    }
    /// Sets a throttle for how much bandwith butler can use. If enabled is false, disables any
    /// previous set throttles. Rate is measured in kbps
    pub fn set_throttle(&self, enabled: bool, rate: i64) {
        self.request(
            "/call/Network.SetBandwidthThrottle",
            json!({
            "enabled":enabled,
            "rate":rate
        })
            .to_string(),
        )
        .expect("Couldn't set throttle");
    }
    /// Fetches the best available sale for a game(if such a sale exists)
    pub fn fetch_sale(&self, game_id: i32) -> Result<Sale, BError> {
        let sls = self
            .request(
                "/call/Fetch.Sale",
                json!({
                "gameId": game_id,
            })
                .to_string(),
            )
            .expect("Couldn't fetch sale");
        parse_r(sls, "sale")
    }
    /// Gets all configured butler install locations in a vec
    pub fn get_install_locations(&self) -> Result<Vec<InstallLocationSummary>, BError> {
        self.res_preq("/call/Install.Locations.List", vec![], "installLocations")
    }
    /// Gets info on a filesystem
    pub fn statfs<N>(&self, path: N) -> Result<FsInfo, BError>
    where
        N: Into<String>,
    {
        self.res_req("/call/System.StatFS", vec![("path", &path.into())])
    }
    /// Checks if an update is available for a vec of Caves. If you pass an empty vec, all caves
    /// will be checked.
    pub fn check_update(&self, cave_ids: Vec<String>) -> Result<CheckUpdate, BError> {
        let cuis = self
            .request(
                "/call/CheckUpdate",
                json!({
            "caveIds":cave_ids,
            "verbose":false
        })
                .to_string(),
            )
            .expect("Couldn't check updates");
        pres(cuis)
    }
    /// Cancels an install. Needs an install id. Result is true if cancel succeeded
    pub fn install_cancel<N>(&self, id: N) -> Result<bool, BError>
    where
        N: Into<String>,
    {
        self.res_preq(
            "/call/Install.Cancel",
            vec![("id", &id.into())],
            "didCancel",
        )
    }
    /// Queues up a game installation
    pub fn install_queue<N>(
        &self,
        game: Game,
        install_location_id: N,
        upload: Upload,
        reason: DownloadReason,
    ) -> Result<QueueResponse, BError>
    where
        N: Into<String>,
    {
        let req = InstallQueueReq {
            install_location_id: install_location_id.into(),
            reason: dr_str(reason),
            game: game,
            upload: upload,
        };
        let rstr = serde_json::to_string(&req).unwrap();
        let qis = self
            .request("/call/Install.Queue", rstr)
            .expect("Couldn't queue game for download");
        pres(qis)
    }
    /// Performs an Install. Download must be completed beforehand
    pub fn install_perform<N>(&self, queue_id: N, staging_folder: N)
    where
        N: Into<String>,
    {
        self.request(
            "/call/Install.Perform",
            json!({
                "id":queue_id.into(),
                "stagingFolder": staging_folder.into()})
            .to_string(),
        )
        .expect("Couldn't perform install");
    }
    /// Fetches all uploads for a game
    pub fn fetch_uploads(&self, game_id: i32, compatible: bool) -> Result<Vec<Upload>, BError> {
        let uis = self
            .request(
                "/call/Fetch.GameUploads",
                json!({
                "gameId": game_id,
                "compatible": compatible,
                "fresh": true
            })
                .to_string(),
            )
            .expect("Couldn't fetch game uploads");
        parse_r(uis, "uploads")
    }
    /// Queues a download to later be downloaded by downloads_drive
    pub fn download_queue(&self, i_queue: QueueResponse) {
        self.request(
            "/call/Downloads.Queue",
            json!({
                "item": serde_json::to_string(&i_queue).unwrap()
            })
            .to_string(),
        )
        .expect("Couldn't queue download");
    }
    /// Downloads all games in the queue. Completes when they are all done
    pub fn downloads_drive(&self) {
        let uri = "http://".to_string() + &self.address + "/call/Downloads.Drive";
        let mut builder = hyper::Request::builder();
        builder.method("POST");
        builder.header("X-Secret", self.secret.as_str());
        builder.header("X-ID", "0");
        builder.uri(uri);
        let request = builder.body(hyper::Body::empty()).unwrap();
        self.hclient.request(request);
        let mut done = false;
        while !done {
            ::std::thread::sleep_ms(1000);
            self.clear_completed();
            let ds = self.downloads_list();
            if ds.is_err() {
                done = true;
            }
        }
    }
    /// Cancels driving downloads. Returns bool indicating success.
    pub fn cancel_download_drive(&self) -> Result<bool, BError> {
        self.res_preq("/call/Downloads.Drive.Cancel", vec![], "didCancel")
    }
    /// Forces butler's online/offline state. True is offline, False is online
    pub fn set_offline(&self, online: bool) {
        self.request(
            "/call/Network.SetSimulateOffline",
            json!({ "enabled": online }).to_string(),
        )
        .expect("Couldn't change butler's network status");
    }
    /// Discards one download
    pub fn discard_download<N>(&self, download_id: N)
    where
        N: Into<String>,
    {
        self.request(
            "/call/Downloads.Discard",
            json!({ "downloadId": download_id.into() }).to_string(),
        )
        .expect("Couldn't discard download");
    }
    /// Prioritizes by download id
    pub fn prioritize_download<N>(&self, download_id: N)
    where
        N: Into<String>,
    {
        self.request(
            "/call/Downloads.Prioritize",
            json!({ "downloadId": download_id.into() }).to_string(),
        )
        .expect("Couldn't prioritize download");
    }
    /// Retries an errored download id
    pub fn download_retry<N>(&self, download_id: N)
    where
        N: Into<String>,
    {
        self.request(
            "/call/Downloads.Retry",
            json!({ "downloadId": download_id.into() }).to_string(),
        )
        .expect("Couldn't retry download");
    }
    /// Gets butler version strings
    pub fn get_version(&self) -> Result<VersionInfo, BError> {
        self.res_req("/call/Version.Get", vec![])
    }
    /// Clears all completed downloads from the queue
    pub fn clear_completed(&self) {
        self.request("/call/Downloads.ClearFinished", "{}".to_string())
            .expect("Couldn't clear completed donwloads");
    }
    /// A helper function that performs all of the game installation/download steps for you.
    /// Recommended over doing installation yourself.
    pub fn install_game<N>(&self, game: Game, install_location_id: N, upload: Upload)
    where
        N: Into<String>,
    {
        let inf = self
            .install_queue(game, install_location_id, upload, DownloadReason::Install)
            .unwrap();
        let id = inf.id.clone();
        let stf = inf.staging_folder.clone();
        self.download_queue(inf);
        self.downloads_drive();
        self.install_perform(id, stf);
    }
    /// Fetches a vec of Downloads from the queue, returning a BError if none are available
    pub fn downloads_list(&self) -> Result<Vec<Download>, BError> {
        self.res_preq("/call/Downloads.List", vec![], "downloads")
    }
    /// Searches games for string. Requires profileid.
    pub fn search_games<N>(&self, profile_id: i32, query: N) -> Result<Vec<Game>, BError>
    where
        N: Into<String>,
    {
        let gis = self
            .request(
                "/call/Seach.Games",
                json!({
            "profileId":profile_id,
            "query":query.into()
        })
                .to_string(),
            )
            .unwrap();
        parse_r(gis, "games")
    }
    /// Adds a new install location
    pub fn install_location_add<N>(&self, path: N)
    where
        N: Into<String>,
    {
        self.request(
            "/call/Install.Locations.Add",
            json!({ "path": path.into() }).to_string(),
        )
        .expect("Couldn't add new install location");
    }
    /// Removes an install location
    pub fn install_location_remove<N>(&self, id: N)
    where
        N: Into<String>,
    {
        self.request(
            "/call/Install.Locations.Remove",
            json!({ "id": id.into() }).to_string(),
        )
        .expect("Couldn't remove install location");
    }
    /// Gets an install location from a previously fetched id
    pub fn install_location_get_by_id<N>(&self, id: N) -> Result<InstallLocationSummary, BError>
    where
        N: Into<String>,
    {
        self.res_req("/call/Install.Locations.GetByID", vec![("id", &id.into())])
    }
    /// Uninstalls a cave
    pub fn uninstall<N>(&self, cave_id: N)
    where
        N: Into<String>,
    {
        self.request(
            "/call/Uninstall.Perform",
            json!({ "caveId": &cave_id.into() }).to_string(),
        )
        .expect("Couldn't uninstall cave");
    }
    fn res_req<T, N>(&self, url: N, body: Vec<(&str, &str)>) -> Result<T, BError>
    where
        T: DeserializeOwned,
        N: Into<String>,
    {
        let mut b = String::new();
        if body.len() < 1 {
            b = "{}".to_string();
        } else {
            b = serde_json::to_string(&mp(body)).unwrap();
        }
        let ris = self.request(url.into(), b).unwrap();
        let res = pres(ris);
        res
    }
    fn res_preq<T, N>(&self, url: N, body: Vec<(&str, &str)>, field: N) -> Result<T, BError>
    where
        T: DeserializeOwned,
        N: Into<String>,
    {
        let mut b = String::new();
        if body.len() < 1 {
            b = "{}".to_string();
        } else {
            b = serde_json::to_string(&mp(body)).unwrap();
        }
        let ris = self.request(url, b).unwrap();
        let res = parse_r(ris, field);
        res
    }
}
fn get_home() -> String {
    return String::from(env::home_dir().unwrap().to_str().unwrap());
}
/// Translates a DownloadReason into a string to be used by the butler API
fn dr_str(r: DownloadReason) -> String {
    match r {
        DownloadReason::Install => "install",
        DownloadReason::Reinstall => "reinstall",
        DownloadReason::Update => "update",
        DownloadReason::VersionSwitch => "version-switch",
    }
    .to_string()
}
/// A helper function to interpet a common result response from butler. Took far too long to write.
fn pres<T>(st: String) -> Result<T, BError>
where
    T: DeserializeOwned,
{
    let res: Result<ResponseRes, serde_json::Error> = serde_json::from_str(&st);
    if res.is_ok() {
        return Ok(
            serde_json::from_str(&serde_json::to_string(&res.unwrap().result).unwrap()).unwrap(),
        );
    } else {
        let err: Result<ResponseErr, serde_json::Error> = serde_json::from_str(&st);
        if err.is_ok() {
            return Err(err.unwrap().error);
        } else {
            return Err(BError {
                message: st,
                code: -1,
            });
        }
    }
}
fn parse_r<T, N>(st: String, prop: N) -> Result<T, BError>
where
    T: DeserializeOwned,
    N: Into<String>,
{
    let prop = prop.into();
    let response: Result<ResponseRes, serde_json::Error> = serde_json::from_str(&st);
    if response.is_ok() {
        let result = response.unwrap().result;
        if result.contains_key(&prop) {
            let finish =
                serde_json::from_str(&serde_json::to_string(&result[&prop]).unwrap()).unwrap();
            return Ok(finish);
        } else {
            return Err(BError {
                message: "Not Found".to_string(),
                code: -2,
            });
        }
    } else {
        let err: Result<ResponseErr, serde_json::Error> = serde_json::from_str(&st);
        if err.is_ok() {
            return Err(err.unwrap().error);
        } else {
            return Err(BError {
                message: st,
                code: -1,
            });
        }
    }
}
/// A helper function to create a map easily for use with res_req
fn mp(data: Vec<(&str, &str)>) -> HashMap<String, String> {
    data.into_iter()
        .map(|x| (x.0.to_string(), x.1.to_string()))
        .collect::<HashMap<String, _>>()
}
