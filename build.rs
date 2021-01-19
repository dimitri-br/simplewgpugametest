use std::process::Command;
use std::env;
use std::path::Path;

fn main(){
    /*// Get current working dir
    let working_dir = env::current_dir().unwrap();
    // Go to shader dir
    env::set_current_dir(".\\src\\shaders\\").unwrap();
    // Compile
    let status = Command::new("cmd").args(&["/C compiler.bat"]).status().unwrap_or_else(|e|{
        panic!("Failed to compile shaders: {}", e);
    });
    // return to root dir
    env::set_current_dir(working_dir).unwrap();*/
}