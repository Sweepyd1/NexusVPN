
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use std::process::Command;
#[derive(Serialize, Deserialize)]
struct VlessConfig {
    name: String,
    vless_data: String,
}

#[tauri::command]
async fn create_nexus_dir() -> Result<String, String> {

    let home_dir = dirs::home_dir()
        .ok_or("Не удалось найти домашнюю директорию".to_string())?;

    let nexus_path = home_dir.join(".nexus");
    

    std::fs::create_dir_all(&nexus_path)
        .map_err(|e| format!("Ошибка создания папки: {}", e))?;
    

    Ok(nexus_path.to_string_lossy().into_owned())
}

#[tauri::command]
async fn save_vless_config(name: String, vless_data: String) -> Result<(), String> {

    if name.trim().is_empty() {
        return Err("Имя конфигурации не может быть пустым".into());
    }
    if vless_data.trim().is_empty() {
        return Err("VLESS данные не могут быть пустыми".into());
    }

    let home_dir = dirs::home_dir()
        .ok_or("Не удалось найти домашнюю директорию")?;
    
   
    let nexus_dir = home_dir.join(".nexus");
    

    fs::create_dir_all(&nexus_dir)
        .map_err(|e| format!("Ошибка создания директории: {}", e))?;

    
    let config = VlessConfig {
        name: name.clone(),
        vless_data,
    };

 
    let json_data = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Ошибка сериализации: {}", e))?;

  
    let file_name = format!("{}.json", sanitize_filename::sanitize(&name));
    let config_path = nexus_dir.join(file_name);

  
    fs::write(&config_path, json_data)
        .map_err(|e| format!("Ошибка записи файла: {}", e))?;

    Ok(())
}


#[tauri::command]
async fn start_vless(config_name: String) -> Result<String, String> {
    let home_dir = dirs::home_dir()
        .ok_or("Не найдена домашняя директория")?;
    
    let config_path = home_dir
        .join(".nexus")
        .join(format!("{}.json", sanitize_filename::sanitize(&config_name)));
    
 
    if !config_path.exists() {
        return Err("Конфиг не найден".into());
    }

  
    let output = Command::new("xray")
        .arg("run")
        .arg("-config")
        .arg(config_path)
        .output()
        .map_err(|e| format!("Ошибка запуска: {}", e))?;

    if output.status.success() {
        Ok("Соединение установлено".into())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}


#[tauri::command]
async fn get_configs() -> Result<Vec<VlessConfig>, String> {
    let nexus_dir = dirs::home_dir()
        .ok_or("Не удалось найти домашнюю директорию")?
        .join(".nexus");

    let mut configs = Vec::new();

    
    let entries = match fs::read_dir(&nexus_dir) {
        Ok(e) => e,
        Err(_) => return Ok(vec![]), 
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue, 
        };

        let path = entry.path();
        
      
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

    
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

      
        match serde_json::from_str::<VlessConfig>(&content) {
            Ok(config) => configs.push(config),
            Err(e) => {
                eprintln!("Ошибка парсинга файла {}: {}", path.display(), e);
                continue;
            }
        };
    }

 
    configs.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(configs)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_nexus_dir,
            save_vless_config,
            start_vless,
            get_configs
        ])
        .run(tauri::generate_context!())
        .expect("Ошибка запуска приложения");
}