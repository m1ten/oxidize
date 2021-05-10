// static or const variables 

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MainConfig {
    information: Option<Information>,
    language: Option<Vec<Language>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Information {
    name: Option<String>,
    path: Option<String>,
    version: Option<String>,
    author: Option<VectorOrString>,
    description: Option<String>,
    readme: Option<String>,
    repository: Option<String>,
    license: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Language {
    name: Option<String>,
    alias: Option<VectorOrString>,
    version: Option<VectorOrString>,
    interpreter: Option<VectorOrString>,
    compiler: Option<VectorOrString>,
    path: Option<VectorOrString>,
    extension: Option<VectorOrString>,
    command: Option<Vec<Command>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    name: String,
    command: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum VectorOrString {
    Vector(Vec<String>),
    String(String),
}

pub const NAME: &str = "oxidize";
pub const VERSION: &str = "0.1.0";
pub const AUTHOR: &str = "miten"; 
// pub static mut CONFIG_FILE: &str = "";
// pub static mut COMPILE_FILE: &str = "";