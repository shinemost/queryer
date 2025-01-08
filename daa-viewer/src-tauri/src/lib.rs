// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn example_sql() -> String {
    queryer::example_sql()
}

#[tauri::command]
async fn query(sql: String) -> Result<String, String> {
    let mut data = queryer::query(&sql).await.map_err(|err| err.to_string())?;
    Ok(data.to_csv().map_err(|err| err.to_string())?)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![example_sql,query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// SELECT location name, total_cases, new_cases, total_deaths, new_deaths FROM https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 5