const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSLES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
