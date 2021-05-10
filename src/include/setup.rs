use crate::include;
use ansi_term::Color;

pub fn setup() {
    let configuration: Vec<String> = Vec::new();

    println!(
        "{} v{} | {} & contributors\n",
        include::vars::NAME,
        include::vars::VERSION,
        include::vars::AUTHOR
    );

    println!("Would you like to setup {0}? (see https://github.com/m1ten/{0}/wiki/setup)", include::vars::NAME);
    
    print!("[{}/{}] > ", Color::Green.paint("Y"), Color::Red.paint("n"));
    let answer = include::readln();

    if answer == "Y".to_string() || answer == "y".to_string() || answer == "".to_string() {
        /* add code here to check if config exists */

        print!("\nChoose a format from the following.\nTOML: 0 (default)\nJSON: 1\nYAML: 2\n[0/1/2] > ");

        let config_format = match &include::readln() as &str
        {
            "1" => "{}.json",
            "2" => "{}.yaml",
            _ => "{}.toml",
        };

        return;

        /* curl config */
        let mut easy = curl::easy::Easy::new();
        easy.url(
            format!(
                "https://raw.githubusercontent.com/m1ten/{0}/templates/{1}",
                include::vars::NAME,
                &config_format.replace("{}", include::vars::NAME)
            )
            .as_str(),
        )
        .unwrap();
        easy.write_function(move |data| {
            include::file::write_file(
                String::from(&config_format.replace("{}", include::vars::NAME)),
                String::from_utf8(data.to_vec()).unwrap(),
            );
            Ok(data.len())
        })
        .unwrap();
        easy.perform().unwrap();
    } else {
        eprintln!("{}", Color::Red.paint("exiting."));
        return;
    }
}
