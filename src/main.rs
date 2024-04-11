pub mod compile;
pub mod error;
pub mod types;
pub mod watch;

use libloading::Symbol;
use notify_debouncer_mini::DebounceEventResult;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use watch::start_watch;

use crate::types::init_library;

fn main() {
    let dir_path: String = String::from(".");
    let src_path: String = format!("{}/test_shared.rs", dir_path);
    let object_path: String = String::from("./libtest_shared.so");

    let debouncer = start_watch(
        "./test_shared.rs",
        move |res: DebounceEventResult| match res {
            // TODO: Watch a dir, and compile any path
            Ok(_) => {
                println!("COMPILING");
                compile::compile(&src_path, &dir_path);

                println!("LOADING");

                let lib = unsafe {
                    init_library(
                        PathBuf::from_str(&src_path).ok(),
                        PathBuf::from(&object_path),
                    )
                }
                .unwrap();
                println!("DONE");

                // In a real program you want to cache the symbol and not do it every time if your
                // application is performance critical
                let fun: Symbol<extern "C" fn() -> ()> =
                    unsafe { lib.as_ref().lib.get(b"hello\0").unwrap() };
                fun();
            }
            Err(_) => println!("CANNOT LOAD TEST"),
        },
    );
    let dur = Duration::from_secs(1);
    loop {
        sleep(dur);
        println!("SLEEPING");
    }
}
