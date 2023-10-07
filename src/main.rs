//use whoami;
use std::env::consts;
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use serde_derive::Deserialize;
use std::process::exit;
use toml;

const DEFAULT_CONFIG_FILE: &str = "src\\Config.toml";


#[derive(Deserialize)]
struct Config { 
    vanilla: String,
    curse: String,
    mmc: String,
    rinth: String,
}

#[derive(Deserialize)]
struct Data {
    config: Config,
}

fn create_default_config(){
    let config_file = &DEFAULT_CONFIG_FILE;

    let config = match fs::read_to_string(config_file) {

        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", config_file);
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&config) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", config);
            exit(1);
        }
    };


    let OS: &str = consts::OS;
    let USER: String = whoami::username();

    println!("hewwo from obsidian");
    println!("{}", OS);
    println!("{}", USER);
    println!("\ninstance locations found:");
    println!("{}", data.config.vanilla);
    println!("{}", data.config.curse);
    println!("{}", data.config.mmc);
    println!("{}\n", data.config.rinth);


    let linux = format!("/home/{}/.config/obsidian", USER);
    let win = format!(r"C:\Users\{}\AppData\Roaming\argo\obsidian", USER);
    let mac = format!("/Users/{}/Library/Application Support/com.argo.obsidian", USER);
/*
    match OS {
        linux => fs::create_dir(linux),
        windows => fs::create_dir(win),
        macos => fs::create_dir(mac),
    };
*/
        //fs::create_dir(proj_dirs.config_dir())?;
        //let config_location:String = String::from("{}\\{}", proj_dirs.config_dir, DEFAULT_CONFIG_FILE);
        //fs::copy(DEFAULT_CONFIG_FILE, &config_location )?;
        // Linux:   /home/alice/.config/obsidian
        // Windows: C:\Users\Alice\AppData\Roaming\argo\obsidian
        // macOS:   /Users/Alice/Library/Application Support/com.argo.obsidian
}
    


fn main() {
    create_default_config();
    //let config: Config = {
      //  let config_text = fs::read_to_string(&DEFAULT_CONFIG_FILE).expect("Error reading file");
        //toml::from_str(&config_text).expect("Error reading stream")
    //};
    //println_f!("@DEBUG\nvanilla:\t{config.vanilla}\ncurse:\t{config.curse}\nmmc:\t{config.mmc}\n" );

}
