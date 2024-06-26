fn main() {
    const _STARTING_MISSILES: i32 = 8;
    const _READY_AMOUNT: i32 = 2;
    
    let mut missiles = 8;
    let ready = 2;

    println!("Firing {} of my {} missiles...", ready, missiles);


    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
