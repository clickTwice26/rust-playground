use std::env;
use std::fs;
pub fn greet(){
    println!("CRUN Beta 2.0");
}

pub fn get_cwd()-> Result<String, i32>{
    match env::current_dir(){
        Ok(path) => Ok(path.to_string_lossy().into_owned()),
        Err(_) => Err(30),
    }
}

pub fn list_files(dir_path : &str){
    match fs::read_dir(dir_path){
        Ok(entries) =>{
            for entry in entries{
                let entry = entry.expect("Failed to get directory entry");
                // println!("{}", entry.path().display());
            }
        },
        Err(e) => eprintln!("Error reading directory : {}", e),
    }
}



