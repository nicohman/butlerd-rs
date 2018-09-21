extern crate reqwest;
use std::process::Command;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate hyper;
extern crate serde;
use reqwest::Method;
use serde::de::DeserializeOwned;
use std::env;
use std::io::Read;
mod Packaged;
pub mod Responses;
use std::collections::HashMap;
use std::fs;
use Packaged::*;
use Responses::*;
#[cfg(target_os = "macos")]
static DB_PATH: &str = "";
#[cfg(target_os = "linux")]
static DB_PATH: &str = "~/.config/itch/db/butler.db";
#[cfg(target_os = "windows")]
static DB_PATH: &str = "";
#[cfg(target_os = "macos")]
static LOG_PATH: &str = "";
#[cfg(target_os = "linux")]
static LOG_PATH: &str = "/tmp/butlerdrs.log";
#[cfg(target_os = "windows")]
static LOG_PATH: &str = "";
#[cfg(target_os = "macos")]
static PRE_PATH: &str = "";
#[cfg(target_os = "linux")]
static PRE_PATH: &str = "~/.config/itch/prereqs";
#[cfg(target_os = "windows")]
static PRE_PATH: &str = "";
const POST: Method = Method::POST;
/// Represents a connection to a butlerd instance
pub struct Butler {
    secret: String,
    address: String,
    client: reqwest::Client,
    pre_dir: String,
    client_launch: reqwest::Client,
}
impl Butler {
    /// Initializes a new butlerd instance. It will close when your program does.
    pub fn new() -> Butler {
        let mut file: fs::File;
        if fs::metadata(LOG_PATH).is_ok() {
            if fs::remove_file(LOG_PATH).is_err() {
                println!("Failed to remove previous log at {}. May crash.", LOG_PATH);
                file = fs::File::open(LOG_PATH).unwrap();
            } else {
                file = fs::File::create(LOG_PATH).unwrap();
            }
        } else {
            file = fs::File::create(LOG_PATH).unwrap();
        }
        Command::new("sh")
            .arg("-c")
            .arg(
                "butler daemon --json --dbpath=".to_string() + &DB_PATH.replace("~", &get_home()) +
                    " --destiny-pid=" + &::std::process::id().to_string(),
            )
            .stdout(file)
            .spawn()
            .expect("Couldn't start butler daemon");
        //TODO: REPLACE
        ::std::thread::sleep_ms(750);
        let mut bd = String::new();
        fs::File::open(LOG_PATH)
            .unwrap()
            .read_to_string(&mut bd)
            .unwrap();
        bd = bd.replace("\\\"", "");
        bd = bd.lines().next().unwrap().to_string();
        let pmeta: BStart =
            serde_json::from_str(&bd.trim()).expect("Couldn't deserialze butler start");
        let secret = pmeta.secret.to_string();
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
            address: pmeta.http[&"address".to_string()].to_string().replace(
                "\"",
                "",
            ),
            client: built,
            pre_dir: PRE_PATH.to_string().replace("~", &get_home()),
            client_launch: builtl,
        }
    }
    ///Shuts down butler daemon.
    pub fn close(&self) {
        self.request(POST, "/Meta.Shutdown", "{}".to_string())
            .expect("Couldn't shut down butler daemon");;
    }
    fn make_request(
        &self,
        method: Method,
        path: &str,
        params: String,
        client: String,
    ) -> Result<String, String> {
        let res: Result<reqwest::Response, reqwest::Error>;
        let url = "http://".to_string() + &self.address.clone() + path;
        if &client == "launch" {
            res = self.client_launch.request(method, &url).body(params).send();
        } else {
            res = self.client.request(method, &url).body(params).send();
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
    fn request(&self, method: Method, path: &str, params: String) -> Result<String, String> {
        self.make_request(method, path, params, "default".to_string())
    }
    fn request_l(&self, method: Method, path: &str, params: String) -> Result<String, String> {
        self.make_request(method, path, params, "launch".to_string())
    }
    /// Fetchs all installed caves
    pub fn fetchall(&self) -> Vec<Cave> {
        let caves: FetchCaves = self.res_req("/call/Fetch.Caves", vec![]).unwrap();
        caves.items
    }
    ///Fetches specific game by id
    pub fn fetch_game(&self, game_id: i32) -> Game {
        let gvs = self.request(
            POST,
            "/call/Fetch.Game",
            json!({
                "gameId": game_id
            }).to_string(),
        ).expect("Couldn't fetch game by id");
        let game: FetchGame = pres(gvs).unwrap();
        game.game
    }
    ///Fetches specific cave by id
    pub fn fetch_cave(&self, cave_id: String) -> Cave {
        let cave: FetchCave = self.res_req("/call/Fetch.Cave", vec![("caveId", &cave_id)])
            .unwrap();
        cave.cave
    }
    /// Makes a cave 'pinned' or not depending on pinned
    pub fn pin_cave(&self, cave_id: String, pinned: bool) {
        self.request(
            POST,
            "/call/Caves.SetPinned",
            json!({
            "caveId":cave_id,
            "pinned":pinned
        }).to_string(),
        ).expect("Couldn't pin cave");
    }
    /// Launches game by CaveID. Note that this will not complete until the game is closed.
    pub fn launch_game(&self, cave_id: String) {
        self.request_l(
            POST,
            "/call/Launch",
            json!({
                "caveId": cave_id,
                "prereqsDir": self.pre_dir.clone()
            }).to_string(),
        ).expect("Couldn't launch game");
    }
    /// Lists saved profiles
    pub fn profile_list(&self) -> Vec<Profile> {
        let profiles: FetchProfiles = self.res_req("/call/Profile.List", vec![]).expect(
            "Couldn't list saved profiles",
        );
        profiles.profiles
    }
    /// Removes a profile's saved info. Also removes it from profile_list. Returns true if
    /// successful.
    pub fn profile_forget(&self, profile_id: i32) -> bool {
        let sis = self.request(
            POST,
            "/call/Profile.Forget",
            json!({ "profileId": profile_id }).to_string(),
        ).expect("Couldn't forget profile");
        let suc: Success = pres(sis).unwrap();
        suc.success
    }
    /// Logs into a profile using saved credentials
    pub fn login_saved(&self, profile_id: i32) -> Profile {
        let pis = self.request(
            POST,
            "/call/Profile.UseSavedLogin",
            json!({ "profileId": profile_id }).to_string(),
        ).expect("Couldn't login using saved credentials");
        let profile: FetchProfile = pres(pis).unwrap();
        profile.profile
    }
    /// Given an API key, logs into a profile and returns profile.
    pub fn login_api_key(&self, api_key: String) -> Profile {
        let profile: FetchProfile =
            self.res_req("/call/Profile.LoginWithAPIKey", vec![("apiKey", &api_key)])
                .unwrap();
        profile.profile
    }
    /// Given an username and password, logs into a profile and returns profile and cookie.
    pub fn login_password(&self, username: String, password: String) -> PassLogRes {
        let profile: PassLogRes = self.res_req(
            "/call/Profile.LoginWithPassword",
            vec![("username", &username), ("password", &password)],
        ).unwrap();
        profile
    }
    /// Fetches all common/cached items and returns summaries
    pub fn fetch_commons(&self) -> Commons {
        let comm: Commons = self.res_req("/call/Fetch.Commons", vec![]).expect(
            "Couldn't fetch commons",
        );
        comm
    }
    /// Fetches a vec of games owned by a specific profile id
    pub fn fetch_profile_games(&self, profile_id: i32) -> Vec<ProfileGame> {
        let pvs = self.request(
            POST,
            "/call/Fetch.ProfileGames",
            json!({
                "profileId": profile_id,
            }).to_string(),
        ).expect("Couldn't fetch profile games");
        let games: FetchPGames = pres(pvs).unwrap();
        games.items
    }
    /// Fetches owned download keys for a profile. Pass fresh as true to force butler to refresh
    /// cache
    pub fn fetch_profile_keys(&self, profile_id: i32, fresh: bool) -> Vec<DownloadKey> {
        let dis = self.request(
            POST,
            "/call/Fetch.ProfileOwnedKeys",
            json!({
            "profileId": profile_id,
            "fresh":fresh
        }).to_string(),
        ).expect("Couldn't fetch profile keys");
        let keys: ProfileKeys = pres(dis).unwrap();
        keys.items
    }
    /// Fetches the best available sale for a game(if such a sale exists)
    pub fn fetch_sale(&self, game_id: i32) -> Option<Sale> {
        let sls = self.request(
            POST,
            "/call/Fetch.Sale",
            json!({
                "gameId": game_id,
            }).to_string(),
        ).expect("Couldn't fetch sale");
        let sale: FetchSale = pres(sls).unwrap();
        sale.sale
    }
    /// Gets all configured butler install locations in a vec
    pub fn get_install_locations(&self) -> Vec<InstallLocationSummary> {
        let idirs: FetchIDirs = self.res_req("/call/Install.Locations.List", vec![])
            .unwrap();
        idirs.installLocations
    }
    /// Checks if an update is available for a vec of Caves. If you pass an empty vec, all caves
    /// will be checked.
    pub fn check_update(&self, cave_ids: Vec<String>) -> CheckUpdate {
        let cuis = self.request(
            POST,
            "/call/CheckUpdate",
            json!({
            "caveIds":cave_ids,
            "verbose":false
        }).to_string(),
        ).expect("Couldn't check updates");
        let cu: CheckUpdate = pres(cuis).unwrap();
        cu
    }
    /// Cancels an install. Needs an id
    pub fn install_cancel(&self, id: &str) -> bool {
        let mut d : DidCancel = self.res_req("/call/Install.Cancel", vec![("id", id)]).unwrap();
        d.didCancel
    }
    /// Queues up a game installation
    pub fn install_queue(
        &self,
        game: Game,
        install_location_id: String,
        upload: Upload,
        reason: DownloadReason,
    ) -> QueueResponse {
        let mut req = InstallQueueReq {
            install_location_id: install_location_id,
            reason: dr_str(reason),
            game: game,
            upload: upload,
        };
        let rstr = serde_json::to_string(&req).unwrap();
        let qis = self.request(POST, "/call/Install.Queue", rstr).expect(
            "Couldn't queue game for download",
        );
        let queue: QueueResponse = pres(qis).unwrap();
        return queue;
    }
    /// Performs an Install. Download must be completed beforehand
    pub fn install_perform(&self, queue_id: String, staging_folder: String) {
        self.request(
            POST,
            "/call/Install.Perform",
            json!({
                "id":queue_id,
                "stagingFolder": staging_folder}).to_string(),
        ).expect("Couldn't perform install");
    }
    /// Fetches all uploads for a game
    pub fn fetch_uploads(&self, game_id: i32, compatible: bool) -> Vec<Upload> {
        let uis = self.request(
            POST,
            "/call/Fetch.GameUploads",
            json!({
                "gameId": game_id,
                "compatible": compatible,
                "fresh": true
            }).to_string(),
        ).expect("Couldn't fetch game uploads");
        let uploads: FetchUploads = pres(uis).unwrap();
        uploads.uploads
    }
    /// Queues a download to later be downloaded by downloads_drive
    pub fn download_queue(&self, i_queue: QueueResponse) {
        self.request(
            POST,
            "/call/Downloads.Queue",
            json!({
                "item": serde_json::to_string(&i_queue).unwrap()
            }).to_string(),
        ).expect("Couldn't queue download");
    }
    /// Downloads all games in the queue. Completes when they are all done
    pub fn downloads_drive(&self) {
        let mut hclient = hyper::Client::new();
        let uri = "http://".to_string() + &self.address + "/call/Downloads.Drive";
        let mut builder = hyper::Request::builder();
        builder.method("POST");
        builder.header("X-Secret", self.secret.as_str());
        builder.header("X-ID", "0");
        builder.uri(uri);
        let mut request = builder.body(hyper::Body::empty()).unwrap();
        hclient.request(request);
        let mut done = false;
        while !done {
            ::std::thread::sleep_ms(1000);
            self.clear_completed();
            let mut ds = self.downloads_list();
            if ds.is_none() {
                done = true;
            }
        }
    }
    /// Cancels driving downloads. Returns bool indicating success.
    pub fn cancel_download_drive(&self) -> bool {
        let done: DidCancel = self.res_req("/call/Downloads.Drive.Cancel", vec![])
            .expect("Couldn't cancel downloads driving");
        done.didCancel
    }
    /// Discards one download
    pub fn discard_download(&self, download_id: String) {
        self.request(
            POST,
            "/call/Downloads.Discard",
            json!({ "downloadId": download_id }).to_string(),
        ).expect("Couldn't discard download");
    }
    /// Prioritizes by download id
    pub fn prioritize_download(&self, download_id: String) {
        self.request(
            POST,
            "/call/Downloads.Prioritize",
            json!({ "downloadId": download_id }).to_string(),
        ).expect("Couldn't prioritize download");
    }
    /// Retries an errored download id
    pub fn download_retry(&self, download_id: String) {
        self.request(
            POST,
            "/call/Downloads.Retry",
            json!({ "downloadId": download_id }).to_string(),
        ).expect("Couldn't retry download");
    }
    /// Gets butler version strings
    pub fn get_version(&self) -> VersionInfo {
        let version: VersionInfo = self.res_req("/call/Version.Get", vec![]).expect(
            "Couldn't get version",
        );
        version
    }
    /// Clears all completed downloads from the queue
    pub fn clear_completed(&self) {
        self.request(POST, "/call/Downloads.ClearFinished", "{}".to_string())
            .expect("Couldn't clear completed donwloads");
    }
    /// A helper function that performs all of the game installation/download steps for you.
    /// Recommended over doing installation yourself.
    pub fn install_game(&self, game: Game, install_location_id: String, upload: Upload) {
        let inf = self.install_queue(game, install_location_id, upload, DownloadReason::Install);
        let id = inf.id.clone();
        let stf = inf.staging_folder.clone();
        self.download_queue(inf);
        self.downloads_drive();
        println!("Downloads drive successful");
        self.install_perform(id, stf);
        println!("Install perform successful");
    }
    /// Fetches a vec of Downloads from the queue, returning None if none are available
    pub fn downloads_list(&self) -> Option<Vec<Download>> {
        let down: DownList = self.res_req("/call/Downloads.List", vec![]).unwrap();
        down.downloads
    }
    /// Searches games for string. Requires profileid.
    pub fn search_games(&self, profile_id: i32, query: String) -> Option<Vec<Game>> {
        let gis = self.request(
            POST,
            "/call/Seach.Games",
            json!({
            "profileId":profile_id,
            "query":query
        }).to_string(),
        ).unwrap();
        let games: GamesSearchRes = pres(gis).unwrap();
        return games.games;
    }
    /// Uninstalls a cave
    pub fn uninstall(&self, cave_id: String) {
        self.request(
            POST,
            "/call/Uninstall.Perform",
            json!({ "caveId": cave_id }).to_string(),
        ).expect("Couldn't uninstall cave");
    }
    fn res_req<T>(&self, url: &str, body: Vec<(&str, &str)>) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let mut b = String::new();
        if body.len() < 1 {
            b = "{}".to_string();
        } else {
            b = serde_json::to_string(&mp(body)).unwrap();
        }
        let ris = self.request(POST, url, b).unwrap();
        let res = pres(ris);
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
    }.to_string()
}
/// A helper function to interpet a common result response from butler. Took far too long to write.
fn pres<T>(st: String) -> Option<T>
where
    T: DeserializeOwned,
{
    let res: ResponseRes = serde_json::from_str(&st).unwrap();
    return Some(
        serde_json::from_str(&serde_json::to_string(&res.result).unwrap()).unwrap(),
    );
}
/// A helper function to create a map easily for use with res_req
fn mp(data: Vec<(&str, &str)>) -> HashMap<String, String> {
    data.into_iter()
        .map(|x| (x.0.to_string(), x.1.to_string()))
        .collect::<HashMap<String, _>>()
}
