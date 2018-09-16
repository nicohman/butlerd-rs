extern crate reqwest;
use std::process::Command;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use std::io::Read;
use std::env;
use reqwest::Method;
pub mod Responses;
use std::fs;
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
/// Represents a connection to a butlerd instance
pub struct Butler {
    pub secret: String,
    pub address: String,
    pub client: reqwest::Client,
    pub pre_dir: String,
    pub client_launch: reqwest::Client,
}
impl Butler {
    /// Initializes a new butlerd instance. It will close when your program does.
    pub fn new() -> Butler {
        if fs::metadata(LOG_PATH).is_ok() {
            if fs::remove_file(LOG_PATH).is_err() {
                println!("Failed to remove previous log at {}. May crash.", LOG_PATH);
            }
        }
        Command::new("sh")
            .arg("-c")
            .arg(
                "butler daemon --json --dbpath=".to_string() +
                    &DB_PATH.replace("~", &get_home()) + " --destiny-pid=" +
                    &::std::process::id().to_string() + " > " + LOG_PATH,
            )
            .spawn()
            .expect("Couldn't start butler daemon");
        ::std::thread::sleep_ms(500);
        let mut bd = String::new();
        fs::File::open("/tmp/butlerdrs.log")
            .unwrap()
            .read_to_string(&mut bd)
            .unwrap();
        bd = bd.replace("\\\"", "");
        bd = bd.lines().next().unwrap().to_string();
        let pmeta: BStart =
            serde_json::from_str(&bd.trim()).expect("Couldn't deserialze butler start");
        let secret = pmeta.secret.to_string();
        let mut headers = reqwest::header::Headers::new();
        headers.set_raw("X-Secret", secret.as_bytes());
        headers.set_raw("X-ID", "0");
        let mut client = reqwest::Client::builder();
        let mut client_launch = reqwest::Client::builder();
        client_launch.default_headers(headers.clone());
        client.default_headers(headers);
        client_launch.timeout(None);
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
        self.request(Method::Post, "/Meta.Shutdown".to_string(), "{}".to_string())
            .expect("Couldn't shut down butler daemon");;
    }
    fn make_request(
        &self,
        method: Method,
        path: String,
        params: String,
        client: String,
    ) -> Result<String, String> {
        let res: Result<reqwest::Response, reqwest::Error>;
        let url = "http://".to_string() + &self.address.clone() + &path;
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
    fn request(&self, method: Method, path: String, params: String) -> Result<String, String> {
        self.make_request(method, path, params, "default".to_string())
    }
    fn request_l(&self, method: Method, path: String, params: String) -> Result<String, String> {
        self.make_request(method, path, params, "launch".to_string())
    }
    /// Fetchs all installed caves
    pub fn fetchall(&self) -> Vec<Cave> {
        let cvs = self.request(
            Method::Post,
            "/call/Fetch.Caves".to_string(),
            "{}".to_string(),
        ).unwrap();
        let caves_r: ResponseRes = serde_json::from_str(&cvs).unwrap();
        let caves = caves_r.result["items"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let new: Cave = serde_json::from_str(&x.to_string()).unwrap();
                return new;
            })
            .collect::<Vec<Cave>>();
        caves
    }
    ///Fetches specific game by id
    pub fn fetch_game(&self, game_id: i32) -> Game {
        let gvs = self.request(
            Method::Post,
            "/call/Fetch.Game".to_string(),
            "{\"gameId\":".to_string() + &game_id.to_string() + "}",
        ).unwrap();
        let game_r: ResponseRes = serde_json::from_str(&gvs).unwrap();
        let game: Game = serde_json::from_str(&game_r.result["game"].to_string()).unwrap();
        game
    }
    ///Fetches specific cave by id
    pub fn fetch_cave(&self, cave_id: String) -> Cave {
        let cvs = self.request(
            Method::Post,
            "/call/Fetch.Cave".to_string(),
            "{\"caveId\":\"".to_string() + &cave_id + "\"}",
        ).expect("Couldn't fetch cave");
        let cave_r: ResponseRes = serde_json::from_str(&cvs).unwrap();
        let cave: Cave = serde_json::from_str(&cave_r.result["cave"].to_string()).unwrap();
        cave
    }
    /// Launches game by CaveID. Note that this will not complete until the game is closed.
    pub fn launch_game(&self, cave_id: String) {
        self.request_l(
            Method::Post,
            "/call/Launch".to_string(),
            "{\"caveId\":\"".to_string() + &cave_id + "\",\"prereqsDir\":\"" + &self.pre_dir +
                "\"}",
        ).expect("Couldn't launch game");
    }
    /// Given an API key, logs into a profile and returns profile.
    pub fn login_api_key(&self, api_key: String) -> Profile {
        let pvs = self.request(
            Method::Post,
            "/call/Profile.LoginWithAPIKey".to_string(),
            "{\"apiKey\":\"".to_string() + &api_key + "\"}",
        ).expect("Couldn't login with Api key");
        println!("{}", pvs);
        let prof_r: ResponseRes = serde_json::from_str(&pvs).unwrap();
        let profile: Profile = serde_json::from_str(&prof_r.result["profile"].to_string()).unwrap();
        profile
    }
    /// Fetches a vec of games owned by a specific profile id
    pub fn fetch_profile_games(&self, profile_id: i32) -> Vec<ProfileGame> {
        let pvs = self.request(
            Method::Post,
            "/call/Fetch.ProfileGames".to_string(),
            "{\"profileId\":\"".to_string() + &profile_id.to_string() + "\"}",
        ).expect("Couldn't fetch profile games");
        let prof_r: ResponseRes = serde_json::from_str(&pvs).unwrap();
        let games = prof_r.result["items"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let new: ProfileGame = serde_json::from_str(&x.to_string()).unwrap();
                return new;
            })
            .collect::<Vec<ProfileGame>>();
        games
    }
    /// Fetches the best available sale for a game(if such a sale exists)
    pub fn fetch_sale(&self, game_id: i32) -> Option<Sale> {
        let sls = self.request(
            Method::Post,
            "/call/Fetch.Sale".to_string(),
            "{\"gameId\":".to_string() + &game_id.to_string() + "}",
        ).expect("Couldn't fetch sale");
        let sale_r: ResponseRes = serde_json::from_str(&sls).unwrap();
        if sale_r.result.contains_key("sale") && !sale_r.result["sale"].is_null() {
            let sale: Sale = serde_json::from_str(&sale_r.result["sale"].to_string()).unwrap();
            return Some(sale);
        } else {
            return None;
        }
    }
}
fn get_home() -> String {
    return String::from(env::home_dir().unwrap().to_str().unwrap());
}
