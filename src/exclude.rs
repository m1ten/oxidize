/* 
let struct_data = MainConfig {
    information: Some(Information {
        name: Some("oxymake".to_string()),
        path: None,
        version: Some("0.1.1".to_string()),
        author: Some(VectorOrString::String("miten".to_string())),
        description: Some("one of the many build systems".to_string()),
        readme: Some("README.md".to_string()),
        repository: Some("https://github.com/m1ten/oxymake".to_string()),
        license: Some("MIT".to_string()),
    }),
    language: Some(vec![
        Language {
            name: Some("python".to_string()),
            alias: Some(VectorOrString::String("py".to_string())),
            version: Some(VectorOrString::Vector(vec![
                "3.9.4".to_string(),
                "2.7.16".to_string(),
            ])),
            interpreter: Some(VectorOrString::Vector(vec![
                "python3".to_string(),
                "python2".to_string(),
            ])),
            compiler: None,
            path: None,
            extension: Some(VectorOrString::String(".py".to_string())),
            command: Some(vec![Command {
                name: "interpret".to_string(),
                command: "{interpreter.0} {file}".to_string(),
            }]),
        },
        Language {
            name: Some("rust".to_string()),
            alias: None,
            version: Some(VectorOrString::Vector(vec![
                "1.51".to_string(),
                "1.53".to_string(),
            ])),
            interpreter: None,
            compiler: Some(VectorOrString::Vector(vec![
                "rustc".to_string(),
                "cargo".to_string(),
            ])),
            path: None,
            extension: Some(VectorOrString::String(".rs".to_string())),
            command: Some(vec![
                Command {
                    name: "debug".to_string(),
                    command: "{compiler.1} build".to_string(),
                },
                Command {
                    name: "release".to_string(),
                    command: "{compiler.1} build --release".to_string(),
                },
            ]),
        },
    ]),
}; 
*/

/*
use curl::easy::Easy;
use std::env;
use std::io::Write;
use toml::map::Map;
use toml::Value;
*/

   // main configuration file name and path
 /*   let mut main_config: Vec<String> = Vec::new();
    main_config.push(".toml".to_string()); // 0 type or extension
    main_config.push(format!("oxymake{}", main_config[0])); // 1 name
    main_config.push(
        format!("{:?}{1}", env::current_exe().ok().unwrap(), main_config[0]).replace("\"", ""),
    ); // 2 path

    // path of execution or current path
    let current_path = env::current_dir().ok().unwrap();

    if cfg!(debug_assertions) {
        dbg!(&main_config);
    }

    file::set_path(&main_config[2].replace(&main_config[1], ""));

    // check if config file already exists
    if !std::path::Path::new(&main_config[1]).exists() {
        print!(
            "main config: {} does not exist, cURL it? [y/n] ",
            &main_config[2]
        );
        // y or n input
        std::io::stdout().flush().unwrap();

        if include::readln().contains("y") {
            // cURL config file
            curl_main_config(main_config[1].to_owned());
        } else {
            println!("exiting.");
            return;
        } 
    }

    main_config.push(file::read_file(main_config[2].to_owned())); // 3 main config data

    check_main_config(
        main_config[1].to_owned(),
        main_config[0].to_owned(),
        main_config[3].to_owned(),
    );
    // set path of execution
    file::set_path(current_path.as_os_str().to_str().unwrap());

    if clap_args[0] != "" {
        //println!("file = {}", clap_args[0]);
        let extension = format!(".{}", clap_args[0].split(".").collect::<Vec<&str>>()[1]);
        let mut lang_index = Value::String("None".to_string());

        if main_config[0] == ".toml" {
            let parsed_mcd = main_config[3]
                .parse::<Value>()
                .ok()
                .and_then(|r| match r {
                    Value::Table(table) => Some(table),
                    _ => None,
                })
                .unwrap_or(Map::new());
            let mut number = 0;
            let status = true;
            while status {
                match parsed_mcd.get("lang") {
                    Some(i) => match i.get(number.to_string()) {
                        Some(j) => match j.get("file_ext") {
                            Some(k) => {
                                if k.as_array()
                                    .unwrap()
                                    .contains(&toml::Value::String(extension.to_string()))
                                {
                                    //println!("tru, {} does!", extension);
                                    break;
                                } else {
                                    println!("fals, {} doesn't!", extension);
                                }
                            }
                            _ => (),
                        },
                        _ => panic!("please check the main config, lang.index not found"),
                    },
                    _ => panic!("please check the main config, \"lang\" not found"),
                }
                number += 1;
            }
            lang_index = parsed_mcd["lang"][number.to_string()].to_owned();
        }
    }
}

fn curl_main_config(config_file: String) {
    let mut easy = Easy::new();
    easy.url(
        format!(
            "https://raw.githubusercontent.com/m1ten/{0}/templates/{1}",
            vars::NAME,
            &config_file
        )
        .as_str(),
    )
    .unwrap();
    easy.write_function(move |data| {
        file::write_file(
            String::from(&config_file),
            String::from_utf8(data.to_vec()).unwrap(),
        );
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
}

fn check_main_config(main_config_file: String, main_config_type: String, main_config_data: String) {
    // check (and write) if config file has text
    match main_config_data.as_str() {
        "" | "404: Not Found" => {
            print!(
                "main config: {} is empty (or there was an error), cURL template? [y/n] ",
                main_config_file
            );
            // y or n input
            std::io::stdout().flush().unwrap();
            if include::readln().contains("y") {
                // cURL config file
                curl_main_config(main_config_file.to_owned());
                check_main_config(
                    main_config_file.to_owned(),
                    main_config_type.to_owned(),
                    main_config_data.to_owned(),
                );
            } else {
                println!("exiting.");
                return;
            }
        }
        _ => {
            /* check config */
            if main_config_type == ".toml" {
                match main_config_data.parse::<Value>() {
                    Ok(o) => {
                        if cfg!(debug_assertions) {
                            dbg!(o);
                        }
                    }
                    Err(e) => panic!("Error parsing {}. Err: {}", &main_config_file, e),
                }
            } else {
                panic!(
                    "Sorry the config type, {}, is currently not supported",
                    main_config_type
                );
            }
        }
    } */