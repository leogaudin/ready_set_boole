mod ex00;
use ex00::adder;

fn main() {
    for n in 0..42 {
        println!(
            "{} + {} = {} (expected {})",
            n, n*2, adder(n, n*2), n + n*2
        );
    }
}
