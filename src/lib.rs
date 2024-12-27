use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use sysinfo::{ProcessExt, System, SystemExt};

pub fn close_on_url(browser: &str, target_url: &str) {
    let mut system = System::new_all();
    let browser = browser.to_lowercase();

    loop {
        system.refresh_all();
        for (_, process) in system.get_processes() {
            if process.name().to_lowercase().contains(&browser) {
                if let Some(cmd) = process.cmd().get(1) {
                    if cmd.to_lowercase().contains(&target_url.to_lowercase()) {
                        println!("Closing process with PID: {}", process.pid());
                        if let Err(e) = process.kill() {
                            println!("Failed to kill process {}: {}", process.pid(), e);
                        }
                    }
                }
            }
        }

        sleep(Duration::from_secs(5));
    }
}
