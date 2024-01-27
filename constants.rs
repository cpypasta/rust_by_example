// Constants are set at compile time vs runtime
// Constants can be declared in any scope including global
const MAX_POINTS: u32 = 100_000;
// Static is similar to a constant, but the memory address is fixed and is used as a reference
static MAX_HEALTH: u32 = 100;


fn main() {
    println!("MAX_POINTS: {}", MAX_POINTS);
    println!("MAX_HEALTH: {}", MAX_HEALTH);
}