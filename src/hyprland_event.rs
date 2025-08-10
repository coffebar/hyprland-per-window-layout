// logging

// options struct
use crate::options::Options;

// std lib
use std::fmt;

// system cmd
use std::process::Command;

// global hashmap with Mutex
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
lazy_static! {
    // hashmap to store windows and thier layouts
    static ref HASHMAP: Mutex<HashMap<String, u16>> = Mutex::new(HashMap::new());
    // vec to store layouts (long names)
    pub static ref LAYOUTS: Mutex<Vec<String>> =  Mutex::new(Vec::new());
    // vec to store keyboard names
    pub static ref KEYBOARDS: Mutex<Vec<String>> =  Mutex::new(Vec::new());
    // last active window address
    static ref ACTIVE_WINDOW: Mutex<String> = Mutex::new(String::new());
    // last active window class
    static ref ACTIVE_CLASS: Mutex<String> = Mutex::new(String::new());
    // current active layout index
    static ref ACTIVE_LAYOUT: Mutex<u16> = Mutex::new(0);
}

// work with messages from hyprland socket
pub fn event(name: &str, data: &str, options: &Options) {
    log::debug!("E:'{}' D:'{}'", name, data);

    if name == "activewindow" {
        // save only all before first comma
        let data = data.split(",").next().unwrap_or("").to_string();
        if let Ok(mut active_class) = ACTIVE_CLASS.lock() {
            *active_class = data;
        }
        return;
    }

    if name == "activewindowv2" {
        let addr = format!("0x{data}");
        if let Ok(mut active_window) = ACTIVE_WINDOW.lock() {
            *active_window = addr.clone();
        }
        let map = match HASHMAP.lock() {
            Ok(map) => map,
            Err(_) => return,
        };
        match map.get(&addr) {
            Some(index) => {
                log::debug!("{}: {}", addr, index);
                // set layout to saved one
                change_layout(*index);
            }
            None => {
                drop(map);
                log::debug!("added addr: {}", addr);
                // check if we have default layout for this window
                let default_layouts = &options.default_layouts;

                for (index, window_classes) in default_layouts.iter() {
                    for window_class in window_classes.iter() {
                        if let Ok(active_class) = ACTIVE_CLASS.lock() {
                            for window_active_class in active_class.split(",") {
                                if window_active_class.eq(window_class) {
                                    log::debug!(
                                        "Found default layout {} for window {}",
                                        index,
                                        window_active_class
                                    );
                                    // Drop active_class before acquiring new mutex
                                    std::mem::drop(active_class);
                                    if let Ok(mut map) = HASHMAP.lock() {
                                        map.insert(addr.clone(), *index);
                                        // map will be dropped automatically
                                    }
                                    change_layout(*index);
                                    return;
                                }
                            }
                        }
                    }
                }
                // set layout to default one (index 0)
                if let Ok(mut map) = HASHMAP.lock() {
                    map.insert(addr, 0);
                    // map will be dropped automatically
                }
                change_layout(0);
            }
        }
        return;
    }

    if name == "closewindow" {
        let addr = format!("0x{data}");
        if let Ok(mut map) = HASHMAP.lock() {
            map.remove(&addr);
        }
        return;
    }

    if name == "activelayout" {
        // params ex: keychron-keychron-k2,English (US)
        // params ex with variant: at-translated-set-2-keyboard,English (US, intl., with dead keys)
        if let Some((param_keyboard, param_layout)) = data.split_once(',') {
            if param_keyboard.contains("wlr_virtual_keyboard_v") {
                log::debug!("Skip virtual keyboard {}", param_keyboard);
                return;
            }
            log::debug!(
                "Catch layout changed event on {} with {}",
                param_keyboard,
                param_layout
            );
            fullfill_keyboards_list(param_keyboard.to_string());
            fullfill_layouts_list(param_layout.to_string());

            let layout_vec = match LAYOUTS.lock() {
                Ok(vec) => vec,
                Err(_) => return,
            };
            for (index, layout) in layout_vec.iter().enumerate() {
                let index = index as u16;
                if param_layout.eq(layout) {
                    let active_layout: u16 = match ACTIVE_LAYOUT.lock() {
                        Ok(layout) => *layout,
                        Err(_) => return,
                    };
                    if active_layout == index {
                        log::debug!("Layout {} is current", layout);
                        return;
                    }
                    if let Ok(mut active_layout_ref) = ACTIVE_LAYOUT.lock() {
                        *active_layout_ref = index;
                    }
                    let addr = match ACTIVE_WINDOW.lock() {
                        Ok(window) => window.clone(),
                        Err(_) => return,
                    };

                    if let Ok(mut map) = HASHMAP.lock() {
                        map.insert(addr.clone(), index);
                        log::debug!(
                            "Saved layout {} with index {} on addr {}",
                            layout,
                            index,
                            addr
                        );
                    }

                    return;
                }
            }
        } else {
            log::warn!("Bad 'activelayout' format: {}", data)
        }
    }
}
#[derive(Debug)]
pub struct CommandFailed {}
impl fmt::Display for CommandFailed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command returned error")
    }
}

// run cli command 'hyprctl' with given args
pub fn hyprctl(argv: Vec<&str>) -> Result<String, CommandFailed> {
    let output = Command::new("hyprctl")
        .args(argv)
        .output()
        .expect("failed to execute process");
    match output.status.code() {
        Some(code) => {
            log::debug!("Status code is {}", code);
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        None => Err(CommandFailed {}),
    }
}

// updates layout on all active keyboards
// Note: you need to manualy change layout on keyboard to add it into this list
fn change_layout(index: u16) {
    let mut keyboards = match KEYBOARDS.lock() {
        Ok(kb) => kb,
        Err(_) => return,
    };
    if keyboards.is_empty() {
        log::debug!("layout change interrupt: no keyboard added");
        return;
    }
    log::debug!("layout change {}", index);
    if let Ok(mut active_layout) = ACTIVE_LAYOUT.lock() {
        *active_layout = index;
    }
    let mut kb_index = 0;
    let mut trash: Vec<usize> = Vec::new();
    for kb in keyboards.iter() {
        if kb.contains("yubikey") {
            // skip yubikey
            kb_index += 1;
            continue;
        }
        let new_index = &index.to_string();
        let e = hyprctl(["switchxkblayout", "--", kb.as_str(), new_index].to_vec());
        match e {
            Ok(code) => {
                log::debug!(
                    "Layout changed kb:{} index:{} exit_code:{}",
                    kb,
                    new_index,
                    code
                );
            }
            Err(_e) => {
                log::warn!("Keyboard removed from list: {}", kb);
                trash.push(kb_index);
            }
        }
        kb_index += 1;
    }
    // Remove elements in reverse order to avoid index misalignment
    for kb_index in trash.iter().rev() {
        keyboards.remove(*kb_index);
    }
}

// we have to fill this layouts list on go
pub fn fullfill_layouts_list(long_name: String) {
    // add kb long name to LAYOUTS if not there
    let mut layout_vec = match LAYOUTS.lock() {
        Ok(vec) => vec,
        Err(_) => return,
    };

    // skip blacklisted layouts
    let blacklisted_layouts = ["wvkbd"];
    for layout in blacklisted_layouts.iter() {
        if layout.eq(&long_name) {
            log::debug!("Layout blacklisted: {}", long_name);
            return;
        }
    }

    if !layout_vec.contains(&long_name) {
        layout_vec.push(long_name.clone());
        log::debug!("Layout stored: {}", long_name);
    }
}

pub fn fullfill_keyboards_list(name: String) {
    if let Ok(mut keyboards) = KEYBOARDS.lock() {
        if !keyboards.contains(&name) {
            keyboards.push(name);
        }
    }
}
