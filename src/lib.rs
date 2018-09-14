extern crate reqwest;
use std::process::Command;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use serde_json::value::Map;
use reqwest::Method;
pub mod responses;
use responses::*;
#[cfg(target_os = "macos")]
static DB_PATH: &str = "";
#[cfg(target_os = "linux")]
static DB_PATH: &str = "~/.config/itch/db/butler.db";
#[cfg(target_os = "windows")]
static DB_PATH: &str = "";
pub struct Butler {
   secret:String,
   address:String,
   client: reqwest::Client
}
impl Butler {
    pub fn new () -> Butler {
         let meta = Command::new("butler daemon --json --dbpath ".to_string()+DB_PATH).output().expect("Couldn't start butler adaemon");
        let pmeta : BStart = serde_json::from_str(&String::from_utf8_lossy(&meta.stdout)).expect("Couldn't deserialze butler start");
         Butler {
            secret: pmeta.secret.to_string(),
            address:pmeta.http[&"address".to_string()].to_string(),
            client: reqwest::Client::new()
         }
    }
    fn request(&self, method:Method, path:String) -> Result<String, String> {
        let mut headers = reqwest::header::Headers::new();
        headers.set_raw("X-Secret", self.secret.as_bytes());
        headers.set_raw("X-ID", "0");
        let mut res = self.client.request(method, &(self.address.clone()+&path)).headers(headers).send().unwrap();
        if res.status().is_success() {
            Ok(res.text().unwrap())
        } else {
            Err("No".to_string())
        }
    }
    pub fn fetchall(&self)  {
        
    } 
}
