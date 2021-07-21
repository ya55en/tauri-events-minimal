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

use tauri::Manager;

#[derive(Clone, serde::Serialize)]
struct Payload {
   message: String,
}

fn main() {
   println!("** Tauri app about to start");
   tauri::Builder::default()
      .setup(|app| {
         // listen to the `event-name` (emitted on any window)
         let id = app.listen_global("user-click", |event| {
            println!("got event with payload {:?}", event.payload());
         });
         // unlisten to the event using the `id` returned on the `listen_global` function
         // an `once_global` API is also exposed on the `App` struct
         app.unlisten(id);

         // emit the `event-name` event to all webview windows on the frontend
         app.emit_all(
            "user-click",
            Payload {
               message: "Tauri is awesome! (coming from back-end)".into(),
            },
         )
         .unwrap();
         Ok(())
      })
      .invoke_handler(tauri::generate_handler![say_hello,])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
