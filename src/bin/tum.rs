use std::env;
use std::process;
use std::sync::Arc;
use tum::Tum;

fn main() {
    let tum_app = match Tum::build(env::args()) {
        Ok(tum) => Arc::new(tum),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    };

    // TODO: make OS specific signal handling, handle for Windows OS, currently unix systems are only supported
    let tum_app_handler = Arc::clone(&tum_app);
    if let Err(e) = ctrlc::set_handler(move || match tum_app_handler.halt() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }) {
        eprintln!("ERR failed to register interrupt signal handler: {}", e);
        process::exit(1)
    }

    if let Err(e) = tum_app.run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
