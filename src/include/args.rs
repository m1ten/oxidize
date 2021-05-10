use crate::include::vars;
use clap::{App, Arg};

pub fn run(custom: bool) -> Vec<String> {
      if !custom {
            return std::env::args().collect()
      }

      // create custom args
      let matches = App::new(vars::NAME)
            .version(vars::VERSION)
            .author(vars::AUTHOR)
            .about("one of the many build systems")
            .arg(Arg::with_name("setup")
                  .short("s")
                  .long("setup")
                  .value_name("SETUP")
                  .help("setup oxymake")
                  .takes_value(false))
            .arg(Arg::with_name("file")
                  .short("f")
                  .long("file")
                  .value_name("FILE")
                  .help("file to compile")
                  .takes_value(true))
            .arg(Arg::with_name("option")
                  .short("o")
                  .long("option")
                  .value_name("OPTION")
                  .help("option to run")
                  .takes_value(true))
            .get_matches();

      // return custom args
      let mut matches_data: Vec<String> = Vec::new();
      /* 0 */ matches_data.push(String::from(matches.value_of("file").unwrap_or("")));
      /* 1 */ matches_data.push(String::from(matches.value_of("option").unwrap_or_default()));

      /* 2 */
      match matches.occurrences_of("setup") {
            1 => matches_data.push("true".to_string()),
            _ => matches_data.push("false".to_string())
      }

      matches_data

}
