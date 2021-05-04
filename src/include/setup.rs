use crate::include;

pub fn setup() {
    let configuration: Vec<String> = Vec::new();

    println!(
        "{} v{} | {} & contributors\n",
        include::vars::NAME,
        include::vars::VERSION,
        include::vars::AUTHOR
    );

    let answer = include::readln(
        "Would you like to setup {}? [Y/n]: "
            .to_string()
            .replace("{}", include::vars::NAME),
    );
    
    if answer == "Y".to_string() || answer == "y".to_string() || answer == "".to_string()
    {
        /* add code here to check if config exists */

        /* ask to curl config */
        println!("Hello?");
    } else {
        eprintln!("exiting.");
        return;
    }
}
