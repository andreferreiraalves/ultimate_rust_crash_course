const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;

    println!("Firing {} of my {} misseles", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);

    let (missiles2, ready2) = (missiles, ready);

    println!("Firing2 {} of my {} misseles", ready2, missiles2);
}
