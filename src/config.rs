use std::fs;
use std::io;
use std::io::BufRead;
use crate::config;
use crate::structures::structures::Configuration;
use crate::utils::get_cwd;


pub const CONFIG_FILE_PATH : &str = "config.sha";
// src/config.rs
pub fn load_config(config_file_name : &str) -> Configuration{
    let mut project_path = get_cwd().expect("Failed to retrieve project path");
    project_path.push_str("/src/");
    project_path.push_str(config_file_name);
    println!("{}", project_path);

    let file = fs::File::open(project_path).expect("Failed to open config file");
    let reader = io::BufReader::new(file); 
    for line in reader.lines(){
        if let Err(content) = &line{
            println!("{}", content);
            
        }
        if let Ok(content) = &line {
            println!("{}", content);
        }   
    }
    Configuration{
        debug: true,
        logging: true,
    }
}
