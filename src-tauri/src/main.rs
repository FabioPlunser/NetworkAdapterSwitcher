// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use powershell_script;
use std::process::Command;

#[tauri::command]
async fn get_all_network_adapters() -> String {
    let response = powershell_script::run(
        "Get-NetAdapter | Select-Object -Property Name,MacAddress,LinkSpeed,Status |  ConvertTo-Json",
    );
    match response {
        Ok(output) => output.to_string(),
        Err(error) => error.to_string(),
    }
}

#[tauri::command]
fn enable_adapter(adapter_name: String) {
    println!("enable adapter {:?}", adapter_name);
    println!("Enable-NetAdapter -Name \"{}\" -Confirm:$false",
    adapter_name);

    let script = format!(
        "Enable-NetAdapter -Name \"{}\" -Confirm:$false",
        adapter_name
    );

    let _status = Command::new("powershell")
        .arg("-Command")
        .arg(format!("Start-Process PowerShell -Verb RunAs -ArgumentList '-NoProfile -Command \"{}\"'", script))
        .status()
        .expect("failed to execute process");
      
    
}

#[tauri::command]
fn disable_adapter(adapter_name: String) {
    println!("disable adapter {:?}", adapter_name);
    println!("Disable-NetAdapter -Name \"{}\" -Confirm:$false",
    adapter_name);

    let script = format!(
      "Disable-NetAdapter -Name \"{}\" -Confirm:$false",
      adapter_name
    );

    let _status = Command::new("powershell")
    .arg("-Command")
    .arg(format!("Start-Process PowerShell -Verb RunAs -ArgumentList '-NoProfile -Command \"{}\"'", script))
    .status()
    .expect("failed to execute process");
  
    
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_all_network_adapters,
            enable_adapter,
            disable_adapter
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
