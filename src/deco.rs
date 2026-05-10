pub fn _log(level : &str, message : &str){

    let color_code = match level.to_lowercase().as_str(){
        "error" => "\x1b[31m",   // Red
        "warning" => "\x1b[33m", // Yellow
        "info" => "\x1b[34m",    // Blue
        "debug" => "\x1b[32m",   // Green
        _ => "\x1b[0m",     
    };

    let reset = "\x1b[0m";

    println!("{}{} : {}{}", color_code, level.to_lowercase().as_str(), reset, message);
}