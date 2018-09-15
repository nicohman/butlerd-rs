extern crate butlerd;
use butlerd::Butler;
thread_local!(static b : Butler = Butler::new());
#[test]
fn fetchall() {
    b.with(|but| {
        let games = but.fetchall();

        for game in games {
            println!("{}", game.game.title);
        }
    });
}
#[test]
fn fetch_game() {
    b.with(|but| {
        let game = but.fetch_game(248620);
        println!("{:?}", game);
    });
}
