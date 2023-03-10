use std::env; // read env variables
use std::io::BufRead; // read unix socket
use std::io::BufReader; // read unix socket
use std::os::unix::net::UnixStream;

mod hyprland_event; // work with message from socket
use hyprland_event::{event, fullfill_layouts_list, hyprctl};

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
    loop {
        // read message from socket
        let mut buf: Vec<u8> = vec![];
        reader.read_until(b'\n', &mut buf).unwrap();
        let data = String::from_utf8_lossy(&buf);
        let data_parts: Vec<&str> = data.trim().split(">>").collect();
        if data_parts.len() > 1 {
            event(data_parts[0], data_parts[1])
        }
    }
}

// get keyboards count listed in hyprland conf file (input section)
fn get_kb_layouts_count() -> u16 {
    // get layouts list from hyprctl cli call
    match hyprctl(["getoption", "input:kb_layout", "-j"].to_vec()) {
        Ok(output) => {
            log::debug!("input:kb_layout: {}", output);
            // parse the string from stdin into serde_json::Value
            let json: Value = serde_json::from_str(&output).unwrap();
            let kb_layout = str::replace(&json["str"].to_string().trim(), "\"", "");

            if kb_layout.len() > 0 {
                let items: Vec<&str> = kb_layout.split(",").collect();
                return items.len() as u16;
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

// get default layout from cli command "hyprctl devices -j"
// value of ['keyboards'][0]['active_keymap']
fn get_default_layout_name() {
    fullfill_layouts_list("English (US)".to_string());
    match hyprctl(["devices", "-j"].to_vec()) {
        Ok(output) => {
            log::debug!("input:kb_layout: {}", output);
            // parse the string from stdin into serde_json::Value
            let json: Value = serde_json::from_str(&output).unwrap();
            let kb_layout = str::replace(
                &json["keyboards"][0]["active_keymap"].to_string().trim(),
                "\"",
                "",
            );
            if kb_layout.len() > 0 {
                fullfill_layouts_list(kb_layout.to_string());
            } else {
                log::warn!("Keyboard layouts not found")
            }
        }
        Err(_e) => {
            println!("Failed to get devices from hyprctl");
        }
    }
}

// read env variables and listen Hyprland unix socket
fn main() {
    // to see logs in output: add env RUST_LOG='debug'
    env_logger::init();
    // this program make sense if you have 2+ layouts
    let layouts_found = get_kb_layouts_count();

    if layouts_found < 2 {
        println!("Fatal error: You need to configure layouts on Hyprland");
        println!("Add kb_layout option to input group in your hyprland.conf");
        println!("You don't need this program if you have only 1 keyboard layout");
        std::process::exit(1);
    }
    get_default_layout_name();

    match env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(val) => {
            let socket = format!("/tmp/hypr/{}/.socket2.sock", val);
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
