mod include;

use include::{args, file, vars, setup};

fn main() {

    // custom args
    let clap_args = args::run(true);

    if clap_args[2] == "true".to_string() {
        setup::setup();
    }

    return;
}
