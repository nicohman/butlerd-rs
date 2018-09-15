extern crate reqwest;
use std::process::Command;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use std::io::Read;
use std::env;
use serde_json::value::Map;
use reqwest::Method;
pub mod responses;
use std::fs;
use responses::*;
#[cfg(target_os = "macos")]
static DB_PATH: &str = "";
#[cfg(target_os = "linux")]
static DB_PATH: &str = "~/.config/itch/db/butler.db";
#[cfg(target_os = "windows")]
static DB_PATH: &str = "";
pub struct Butler {
    pub secret: String,
    pub address: String,
    pub client: reqwest::Client,
}
impl Butler {
    pub fn new() -> Butler {
        let ch = Command::new("sh")
            .arg("-c")
            .arg(
                "butler daemon --json --dbpath=".to_string() +
                    &DB_PATH.replace("~", &get_home()) + " --destiny-pid=" +
                    &::std::process::id().to_string() + " > /tmp/butlerdrs.log",
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
        let pmeta: BStart =
            serde_json::from_str(&bd.trim()).expect("Couldn't deserialze butler start");
        Butler {
            secret: pmeta.secret.to_string(),
            address: pmeta.http[&"address".to_string()].to_string().replace(
                "\"",
                "",
            ),
            client: reqwest::Client::new(),
        }
    }
    fn request(&self, method: Method, path: String, params: String) -> Result<String, String> {
        let mut headers = reqwest::header::Headers::new();
        headers.set_raw("X-Secret", self.secret.as_bytes());
        headers.set_raw("X-ID", "0");
        let url = "http://".to_string() + &self.address.clone() + &path;
        let mut res = self.client
            .request(method, &url)
            .headers(headers)
            .body(params)
            .send()
            .unwrap();
        if res.status().is_success() {
            Ok(res.text().unwrap())
        } else {
            Err("No".to_string())
        }
    }
    pub fn fetchall(&self) -> Vec<Cave> {
        let cvs = self.request(
            Method::Post,
            "/call/Fetch.Caves".to_string(),
            "{}".to_string(),
        ).unwrap();
        let mut cavesR: ResponseRes = serde_json::from_str(&cvs).unwrap();
        let mut caves = cavesR.result["items"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let mut new: Cave = serde_json::from_str(&x.to_string()).unwrap();
                return new;
            })
            .collect::<Vec<Cave>>();
        caves
    }
    pub fn fetch_game(&self, id: i32) -> Game {
        let gvs = self.request(
            Method::Post,
            "/call/Fetch.Game".to_string(),
            "{\"gameId\":".to_string() + &id.to_string() + "}",
        ).unwrap();
        let mut gameR: ResponseRes = serde_json::from_str(&gvs).unwrap();
        let mut game: Game = serde_json::from_str(&gameR.result["game"].to_string()).unwrap();
        game
    }
}
fn get_home() -> String {
    return String::from(env::home_dir().unwrap().to_str().unwrap());
}
