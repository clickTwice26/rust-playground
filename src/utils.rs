use std::env;
pub fn greet(){
    println!("Hello I am from utils file");
}

pub fn get_cwd()-> Result<String, i32>{
    match env::current_dir(){
        Ok(path) => Ok(path.to_string_lossy().into_owned()),
        Err(_) => Err(30),
    }
}

