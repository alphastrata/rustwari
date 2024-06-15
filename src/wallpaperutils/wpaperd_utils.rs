use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct Monitor {
    id: u32,
    name: String,

    #[serde(flatten)]
    other: std::collections::HashMap<String, serde_json::Value>,
    // description: String,
    // make: String,
    // model: String,
    // serial: String,
    // width: u32,
    // height: u32,
    // refresh_rate: f64,
    // x: u32,
    // y: u32,
    // active_workspace: Workspace,
    // special_workspace: Workspace,
    // reserved: Vec<u32>,
    // scale: f64,
    // transform: u32,
    // focused: bool,
    // dpms_status: bool,
    // vrr: bool,
    // actively_tearing: bool,
    // disabled: bool,
    // current_format: String,
    // available_modes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Workspace {
    id: u32,
    name: String,
}

pub fn get_monitor_names() -> impl IntoIterator<Item = String> {
    get_monitors().unwrap().into_iter().map(|m| m.name)
}

fn get_monitors() -> Result<Vec<Monitor>, Box<dyn std::error::Error>> {
    let output = Command::new("hyprctl").args(&["monitors", "-j"]).output()?;
    if !output.status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to execute hyprctl",
        )));
    }
    // let output = String::from_utf8(output.stdout).unwrap();

    let monitors: Vec<Monitor> = serde_json::from_slice(&output.stdout)?;
    // let monitors: Vec<Monitor> = serde_json::from_str(&output)?;

    Ok(monitors)
}
