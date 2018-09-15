extern crate butlerd;
use butlerd::Butler;
#[test]
fn fetchall () {
    let b = Butler::new();
    println!("{}{}",b.secret,b.address);
    b.fetchall();
}
