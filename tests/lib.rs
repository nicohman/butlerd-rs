extern crate butlerd;
use butlerd::Butler;
use butlerd::Responses::*;
use std::env;
#[cfg(target_os = "macos")]
static OS_STR: &str = "macos";
#[cfg(target_os = "linux")]
static OS_STR: &str = "linux";
#[cfg(target_os = "windows")]
static OS_STR: &str = "windows";
thread_local!(static b : Butler = Butler::new());
#[test]
fn fetchall() {
    b.with(|but| {
        let games = but.fetchall();
        /*for game in games {
            println!("{}", game.game.title);
        }*/
    });
}
#[test]
fn fetch_game() {
    b.with(|but| {
        let game = but.fetch_game(248620);
        assert_eq!("Subserial Network", &game.title);
        assert_eq!(248620, game.id);
    });
}
#[test]
fn fetch_cave() {
    b.with(|but| {
        let cave = but.fetch_cave("e97cd944-386d-4c6c-b1e9-76a3175f4ca9".to_string());
        assert_eq!("LOCALHOST", &cave.game.title);
        assert_eq!(167215, cave.game.id);
    });
}
#[test]
#[ignore]
fn launch_game() {
    b.with(|but| {
        but.launch_game("e97cd944-386d-4c6c-b1e9-76a3175f4ca9".to_string());
    });
}
#[test]
#[ignore]
fn login_api_key() {
    b.with(|but| {
        let key = env::var_os("ITCH_API");
        if key.is_some() {
            but.login_api_key(key.unwrap().into_string().unwrap());
        }
    });
}
#[test]
fn check_sale_none() {
    b.with(|but| {
        let sale = but.fetch_sale(248620);
        assert!(sale.is_none());
    });
}
#[test]
fn get_install_locations() {
    b.with(|but| {
        let locations = but.get_install_locations();
        assert!(locations.len() > 0);
    });
}
#[test]
fn fetch_uploads() {
    b.with(|but| {
        let uploads = but.fetch_uploads(283483, true);
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
    let but = Butler::new();
    let game = but.fetch_game(283483);
    let install_id = &but.get_install_locations()[0];
    let mut uploads = but.fetch_uploads(283483, true);
    uploads = uploads
        .into_iter()
        .filter(|x| x.supports(OS_STR))
        .collect::<Vec<Upload>>();
    but.install_game(game, install_id.id.to_string(), uploads.pop().unwrap());
}
#[test]
fn profile_list () {
    b.with(|but| {
        let profiles = but.profile_list();
        println!("{:?}", profiles);
    });
}
#[test]
fn test_login_fetch_keys () {
    b.with(|but| {
        let username = env::var_os("ITCH_USERNAME");
        let password = env::var_os("ITCH_PASSWORD");
        let profile = but.login_password(username, password);
        let keys = fetch_profile_keys(profile.id, true);
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
