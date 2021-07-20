#![cfg_attr(
   all(not(debug_assertions), target_os = "windows"),
   windows_subsystem = "windows"
)]

#[tauri::command]
fn say_hello(name: String) -> String {
   let response = format!("Hello, {}!", name);
   println!("{:?}", response);
   response.into()
}

fn main() {
   println!("** Tauri app about to start");
   tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![say_hello,])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
