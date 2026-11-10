//! Windows registry read helpers for Godot Mono.

pub const APP_KEY: &str = "Software\\Godot\\Godot Mono";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Godot Mono";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Godot Mono") },
        RegValue { name: "Version",     value: String::from("v1.5.14") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Godot Mono") },
        RegValue { name: "Publisher",       value: String::from("Godot") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Godot Mono\\uninstall.exe\"") },
    ]
}
