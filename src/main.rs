#[macro_use]
extern crate log;
extern crate time;
mod logging;
fn main() {

    match logging::init() {
        Err(e) => println!("Unable to initialize logging system: {}", e),
        _ => {}
    }
    
    info!("ready");
    
}

