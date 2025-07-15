use std::env; // read env variables
use std::io::BufRead; // read unix socket
use std::io::BufReader; // read unix socket
use std::os::unix::net::UnixStream;

mod hyprland_event; // work with message from socket
use hyprland_event::{event, fullfill_keyboards_list, fullfill_layouts_list, hyprctl};

mod options; // read options.toml
use options::read_options;

mod single; // a struct representing one running instance
use single::SingleInstance;

use env_logger; // debug output with env RUST_LOG='debug'
use log;

use serde_json::Value; // json parsed

// listen Hyprland socket
fn listen(socket_addr: String) -> std::io::Result<()> {
    let stream = match UnixStream::connect(socket_addr) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Couldn't connect: {e:?}");
            return Err(e);
        }
    };
    let mut reader = BufReader::new(stream);
    let opt = read_options();
    if opt.keyboards.len() > 0 {
        for keyboard in opt.keyboards.iter() {
            fullfill_keyboards_list(keyboard.to_string());
            log::debug!("Keyboard added: {}", keyboard);
        }
    }
    loop {
        // read message from socket
        let mut buf: Vec<u8> = vec![];
        let readed = match reader.read_until(b'\n', &mut buf) {
            Ok(size) => size,
            Err(e) => {
                log::warn!("Error reading from socket: {}", e);
                break Err(e);
            }
        };
        if readed == 0 {
            break Ok(());
        }
        let data = String::from_utf8_lossy(&buf);
        let data_parts: Vec<&str> = data.trim().split(">>").collect();
        if data_parts.len() > 1 {
            event(data_parts[0], data_parts[1], &opt)
        }
    }
}

// get keyboards count listed in hyprland conf file (input section)
// return -1 if failed
fn get_kb_layouts_count() -> i16 {
    // get layouts list from hyprctl cli call
    match hyprctl(["getoption", "input:kb_layout", "-j"].to_vec()) {
        Ok(output) => {
            log::debug!("input:kb_layout: {}", output);
            // parse the string from stdin into serde_json::Value
            let json: Value = match serde_json::from_str(&output) {
                Ok(json) => json,
                Err(e) => {
                    log::warn!("Failed to parse JSON: {}", e);
                    return -1;
                }
            };
            if json.is_null() || json["str"].is_null() {
                return -1;
            }
            let kb_layout = str::replace(&json["str"].to_string().trim(), "\"", "");

            if kb_layout.len() > 0 {
                let items: Vec<&str> = kb_layout.split(",").collect();
                return items.len() as i16;
            } else {
                0
            }
        }
        Err(_e) => {
            println!("Failed to get option from hyprctl");
            0
        }
    }
}

// try to get kb layouts count 5 times with 1 sec delay
fn get_kb_layouts_count_retry() -> i16 {
    let mut count = 0;
    loop {
        let layouts_found = get_kb_layouts_count();
        if layouts_found > -1 {
            return layouts_found;
        }
        count += 1;
        if count > 5 {
            return -1;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

// check kb_file option is set in hyprland conf file
fn kb_file_isset() -> bool {
    // get layouts list from hyprctl cli call
    match hyprctl(["getoption", "input:kb_file", "-j"].to_vec()) {
        Ok(output) => {
            log::debug!("input:kb_file: {}", output);
            // parse the string from stdin into serde_json::Value
            let json: Value = match serde_json::from_str(&output) {
                Ok(json) => json,
                Err(e) => {
                    log::warn!("Failed to parse JSON: {}", e);
                    return false;
                }
            };
            if json["str"].is_null() {
                return false;
            }
            let value = str::replace(&json["str"].to_string().trim(), "\"", "");
            value != "[[EMPTY]]"
        }
        Err(_e) => {
            println!("Failed to get option from hyprctl");
            false
        }
    }
}

// get default layout from cli command "hyprctl devices -j"
// value of ['keyboards'][0]['active_keymap']
fn get_default_layout_name() -> bool {
    match hyprctl(["devices", "-j"].to_vec()) {
        Ok(output) => {
            // parse the string from stdin into serde_json::Value
            let json: Value = match serde_json::from_str(&output) {
                Ok(json) => json,
                Err(e) => {
                    log::warn!("Failed to parse JSON: {}", e);
                    return false;
                }
            };
            let keyboards = &json["keyboards"];
            log::debug!("keyboards: {}", keyboards);
            if keyboards.is_null() {
                log::warn!("No keyboards found");
                return false;
            }
            let keyboards_array = match keyboards.as_array() {
                Some(arr) => arr,
                None => {
                    log::warn!("Keyboards is not an array");
                    return false;
                }
            };
            if keyboards_array.len() < 1 {
                log::warn!("No keyboards found");
                return false;
            }
            let kb_layout = str::replace(
                &keyboards_array[0]["active_keymap"].to_string().trim(),
                "\"",
                "",
            );
            if kb_layout.len() > 0 {
                fullfill_layouts_list(kb_layout.to_string());
                return true;
            } else {
                log::warn!("Keyboard layouts not found");
                return false;
            }
        }
        Err(_e) => {
            println!("Failed to get devices from hyprctl");
            return false;
        }
    }
}

// read env variables and listen Hyprland unix socket
fn main() {
    // to see logs in output: add env RUST_LOG='debug'
    env_logger::init();
    let instance_sock = SingleInstance::new("hyprland-per-window-layout").unwrap();
    if !instance_sock.is_single() {
        println!("Another instance is running.");
        std::process::exit(1);
    }
    // this program make sense if you have 2+ layouts
    let layouts_found = get_kb_layouts_count_retry();

    if layouts_found < 2 && !kb_file_isset() {
        println!("Fatal error: You need to configure layouts on Hyprland");
        println!("Add kb_layout option to input group in your hyprland.conf");
        println!("You don't need this program if you have only 1 keyboard layout");
        std::process::exit(1);
    }
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 30; // 30 second timeout
    while !get_default_layout_name() {
        // repeat until success
        attempts += 1;
        if attempts >= MAX_ATTEMPTS {
            println!(
                "Timeout: Could not get default layout after {} seconds",
                MAX_ATTEMPTS
            );
            std::process::exit(1);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    match env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(hypr_inst) => {
            let default_socket = format!("/tmp/hypr/{}/.socket2.sock", hypr_inst); // for backawards compatibility
            let socket = match env::var("XDG_RUNTIME_DIR") {
                Ok(runtime_dir) => match std::fs::metadata(format!(
                    "{}/hypr/{}/.socket2.sock",
                    runtime_dir, hypr_inst
                )) {
                    Ok(_) => format!("{}/hypr/{}/.socket2.sock", runtime_dir, hypr_inst),
                    Err(..) => default_socket,
                },
                Err(..) => default_socket,
            };
            // listen Hyprland socket
            match listen(socket) {
                Ok(()) => {}
                Err(e) => log::warn!("Error {e}"),
            }
        }
        Err(e) => println!("Fatal Error: Hyprland is not run. {e}"),
    }
    std::process::exit(1);
}
