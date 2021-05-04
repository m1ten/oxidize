mod include;

use include::{args, file, vars, setup};

fn main() {
    // default rust arguments
    // let arguments: Vec<String> = env::args().collect();

    // custom args
    let clap_args = args::run();

    if clap_args[2] == "true".to_string() {
        setup::setup();
    }

    return;
}
