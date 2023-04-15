#[macro_use]
extern crate fstrings;
extern crate directories;
use directories::ProjectDirs;
use std::fs;
use std::process::exit;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use tui::{backend::CrosstermBackend, Terminal};
use serde::{Serialize, Deserialize};
use toml;


const DEFAULT_CONFIG_FILE: &str = "src\\Config.toml";



#[derive(Deserialize)]
struct Config { 
    vanilla: String,
    curse: String,
    mmc: String,

}

fn create_default_config() -> std::io::Result<()> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "argo",  "obsidian") {
        fs::create_dir(proj_dirs.config_dir())?;
        let config_location = String::from("{}\\{}", proj_dirs.config_dir, DEFAULT_CONFIG_FILE);
        fs::copy(DEFAULT_CONFIG_FILE, &config_location )?;
        // Linux:   /home/alice/.config/obsidian
        // Windows: C:\Users\Alice\AppData\Roaming\argo\obsidian
        // macOS:   /Users/Alice/Library/Application Support/com.argo.obsidian
    }
    Ok(())
    
}

fn main() {
    create_default_config();
    let config: Config = {
        let config_text = fs::read_to_string(&DEFAULT_CONFIG_FILE).expect("Error reading file");
        toml::from_str(&config_text).expect("Error reading stream")
    };
    println_f!("@DEBUG\nvanilla:\t{config.vanilla}\ncurse:\t{config.curse}\nmmc:\t{config.mmc}\n" );

}
