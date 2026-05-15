use crate::structures::structures::Configuration;

mod deco;
mod utils;
mod structures;
mod config;
fn main() {
    utils::greet();
    let configuration : Configuration =  config::load_config(config::CONFIG_FILE_PATH);
    let _path: Result<String, i32> = utils::get_cwd();
    let working_dir : String = match utils::get_cwd(){
        Ok(path) => {
            // let path : String = _path;
            path
        },
        Err(_) => {
            deco::_log("error", "get_cwd()_failed");
            return;
        },
    };
    // deco::clear_console();
    utils::list_files(&working_dir);
}