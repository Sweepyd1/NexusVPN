
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn create_nexus_dir() -> Result<String, String> {

    let home_dir = dirs::home_dir()
        .ok_or("Не удалось найти домашнюю директорию".to_string())?;

    let nexus_path = home_dir.join(".nexus");
    

    std::fs::create_dir_all(&nexus_path)
        .map_err(|e| format!("Ошибка создания папки: {}", e))?;
    

    Ok(nexus_path.to_string_lossy().into_owned())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_nexus_dir])
        .run(tauri::generate_context!())
        .expect("Ошибка запуска приложения");
}