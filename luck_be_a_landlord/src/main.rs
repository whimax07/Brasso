use std::time::Instant;

//noinspection RsModuleNaming
mod luck_be_a_landlord;


fn main() {
    println!("main.rs, main(): Start.");
    let now = Instant::now();
    luck_be_a_landlord::main();
    let elapsed = now.elapsed();
    println!("main.rs, main(), {}ms: End.", elapsed.as_millis());
}
