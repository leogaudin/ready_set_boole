mod ex00;
use ex00::adder;

mod ex01;
use ex01::multiplier;

fn main() {
    println!("\nEX00 - ADDER");
    for n in 0..42 {
        let a: u32 = n;
        let b: u32 = n * 2;
        println!(
            "{} + {} = {} (expected {})",
            a, b, adder(a, b), a + b
        );
    }
    println!("\nEX01 - MULTIPLIER");
    for n in 0..42 {
        let a: u32 = n;
        let b: u32 = n * 420000;
        println!(
            "{} * {} = {} (expected {})",
            a, b, multiplier(a, b), a * b
        );
    }
}
