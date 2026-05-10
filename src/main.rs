mod deco;
mod utils;
fn main() {
    println!("Hello, world!");
    utils::greet();
    let _path: Result<String, i32> = utils::get_cwd();
    match utils::get_cwd(){
        Ok(_path) => deco::_log("debug", "get_cwd()_success"),
        Err(_) => deco::_log("error", "get_cwd()_failed"),
    }
}