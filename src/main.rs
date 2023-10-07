use std::env::consts;
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::path::Path;
use std::process::exit;
use serde_derive::Deserialize;
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
    
    let OS: &str = consts::OS;
    let USER: String = whoami::username();

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


    println!("hewwo from obsidian");
    println!("{}", OS);
    println!("{}", USER);
    println!("\ninstance locations found:");
    println!("{}", data.config.vanilla);
    println!("{}", data.config.curse);
    println!("{}", data.config.mmc);
    println!("{}\n", data.config.rinth);

    let nixpath = format!("/home/{}/.config/obsidian", &USER);
    let nix = Path::new(&nixpath);
    let winpath = format!(r"C:\Users\{}\AppData\Roaming\argo\obsidian", &USER);
    let win = Path::new(&winpath);
    let macpath = format!("/Users/{}/Library/Application Support/com.argo.obsidian", &USER);
    let mac = Path::new(&macpath);


    match OS {
        linux => fs::create_dir_all(nix),
        windows => fs::create_dir_all(win),
        macos => fs::create_dir_all(mac),
    };


}
    


fn main() {
    create_default_config();

}
