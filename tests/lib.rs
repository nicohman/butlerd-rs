extern crate butlerd;
use butlerd::Butler;
#[test]
fn fetchall () {
    let b = Butler::new();
    let games =b.fetchall();
    for game in games {
        println!("{}", game.game.title);
    }
}
