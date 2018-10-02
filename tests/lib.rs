extern crate butlerd;
use butlerd::Butler;
use butlerd::Responses::*;
use std::env;
use std::fs;
#[cfg(target_os = "macos")]
static OS_STR: &str = "macos";
#[cfg(target_os = "linux")]
static OS_STR: &str = "linux";
#[cfg(target_os = "windows")]
static OS_STR: &str = "windows";
thread_local!(static b : Butler = Butler::new().unwrap());
const SSN_ID: i32 = 248620;
const SOL_H: i32 = 283483;
const LOCALH : i32 = 167215;
#[test]
fn fetchall() {
    b.with(|but| {
        let games = but.fetchall();
        println!("{:?}", games);
    });
}
#[test]
fn fetch_game() {
    b.with(|but| {
        let game = but.fetch_game(248620).unwrap();
        assert_eq!("Subserial Network", &game.title);
        assert_eq!(248620, game.id);
    });
}
#[test]
fn fetch_cave() {
    b.with(|but| {
        let cave = but.fetch_cave("e97cd944-386d-4c6c-b1e9-76a3175f4ca9").unwrap();
        assert_eq!("LOCALHOST", &cave.game.title);
        assert_eq!(LOCALH, cave.game.id);
    });
}
#[test]
#[ignore]
fn launch_game() {
    b.with(|but| {
        but.launch_game("e97cd944-386d-4c6c-b1e9-76a3175f4ca9");
    });
}
#[test]
#[ignore]
fn login_api_key() {
    b.with(|but| {
        let key = env::var_os("ITCH_API");
        if key.is_some() {
            but.login_api_key(&key.unwrap().into_string().unwrap());
        }
    });
}
#[test]
fn check_sale_none() {
    b.with(|but| {
        let sale = but.fetch_sale(SSN_ID);
        println!("{:?}", sale);
    });
}
#[test]
fn get_install_locations() {
    b.with(|but| {
        let locations = but.get_install_locations().unwrap();
        assert!(locations.len() > 0);
    });
}
#[test]
fn fetch_uploads() {
    b.with(|but| {
        let uploads = but.fetch_uploads(SOL_H, true).unwrap();
        println!("{:?}", uploads);
        assert!(uploads.len() > 0);
    });

}
#[test]
fn fetch_version() {
    b.with(|but| {
        let v = but.get_version();
        println!("{:?}", v);
    });
}
#[test]
#[ignore]
fn install() {
    let but = Butler::new().unwrap();
    let game = but.fetch_game(SOL_H).unwrap();
    let install_id = &but.get_install_locations().unwrap()[0];
    let mut uploads = but.fetch_uploads(SOL_H, true).unwrap();
    uploads = uploads
        .into_iter()
        .filter(|x| x.supports(OS_STR))
        .collect::<Vec<Upload>>();
    but.install_game(game, &install_id.id.to_string(), uploads.pop().unwrap());
}
#[test]
fn profile_list () {
    b.with(|but| {
        let profiles = but.profile_list();
        println!("{:?}", profiles);
    });
}
#[test]
#[ignore]
fn test_login_fetch_keys () {
    b.with(|but| {
        let username = env::var_os("ITCH_USERNAME").unwrap().into_string().unwrap();
        let password = env::var_os("ITCH_PASSWORD").unwrap().into_string().unwrap();
        let profile = but.login_password(&username, &password).unwrap();
        let keys = but.fetch_profile_keys(profile.profile.id, true);
        println!("{:?}", keys);
    });
}
#[test]
fn commons() {
    b.with(|but| {
        let commons = but.fetch_commons();
        println!("{:?}", commons);
    });
}
#[test]
fn pin() {
    b.with(|but| {
        let caves = but.fetchall().unwrap();
        but.pin_cave(&caves[0].id.clone(), true);
        but.pin_cave(&caves[0].id.clone(), false);
    });
}
#[test]
#[ignore]
fn install_location_add() {
    b.with(|but| {
        if fs::metadata("/tmp/butlertest").is_err() {
            fs::create_dir("/tmp/butlertest").expect("Couldn't create test dir");
        }
         but.install_location_add("/tmp/butlertest");
    });
}
#[test]
fn snooze_cave() {
    b.with(|but| {
        but.snooze_cave("e97cd944-386d-4c6c-b1e9-76a3175f4ca9");
    });
}
#[test]
fn check_updates() {
    b.with(|but| {
        but.check_update(vec!["e97cd944-386d-4c6c-b1e9-76a3175f4ca9".to_string()]);
    });
}
#[test]
fn expireall() {
    b.with(|but| {
        but.expireall();
    });
}
#[test]
fn set_offline() {
    b.with(|but| {
        but.set_offline(true);
        but.set_offline(false);
    });
}
#[test]
fn set_throttle() {
    b.with(|but| {
        but.set_throttle(true, 10);
        but.set_throttle(false, 0);
    });
}
