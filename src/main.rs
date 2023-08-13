use rand::{thread_rng, Rng};

trait print {
    fn print() {
        println!("default!")
    }
}
struct default__;
impl print for default__ {}
struct other;
impl print for other {
    fn print() {
        println!("other")
    }
}
fn run_print<T: print>(some: T) {
print::print();
}
fn main() {
    run_print(default__ {});
    run_print(other {});
}
