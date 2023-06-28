const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);

    let (mut another_missiles, _another_ready) : (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    another_missiles = another_missiles - _another_ready;
    println!("{} another missiles left", another_missiles);
}
