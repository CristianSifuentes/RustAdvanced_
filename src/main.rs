mod advanced;

use advanced::app::run_demo;

fn main() {
    if let Err(error) = run_demo() {
        eprintln!("Application error: {error}");
        std::process::exit(1);
    }
}
