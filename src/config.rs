use std::{fs::{self, File}, io::{BufRead, BufReader}};

use crate::structures::structures::Configuration;


pub const CONFIG_FILE_PATH : &str = "config.rs";
// src/config.rs
pub fn load_config(config_file_name : &str) -> Configuration{
    let file = File::open(config_file_name).expect("Failed to open config file");
    let reader = BufReader::new(file); 
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
