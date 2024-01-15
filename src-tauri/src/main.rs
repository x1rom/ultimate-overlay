// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::thread;
use actix_cors::Cors;
use actix_web::get;
use actix_web::{middleware, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use tauri::{State, Manager};
use tauri::AppHandle;

struct TauriAppState {
    app: Mutex<AppHandle>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct OverlayData {
    name_l: String,
    name_r: String,
    score_l: i8,
    score_r: i8,
    tournament_name: String,
    round_name: String,
    casters: Vec<String>,
}
struct Data(Mutex<OverlayData>);

fn main() {
    tauri::Builder::default()
        .manage(Data(Default::default()))
        .invoke_handler(tauri::generate_handler![set_data, get_data])
        .setup(|app| {

            let handle = app.handle();
            let boxed_handle = Box::new(handle);
    
            thread::spawn(move || {
                init(*boxed_handle).unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[actix_web::main]
async fn init(app: AppHandle)-> std::io::Result<()> {
    let tauri_app = web::Data::new(TauriAppState {
        app: Mutex::new(app),
    });

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_origin();
        App::new()
            .app_data(tauri_app.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(handler)
    })
    .bind(("127.0.0.1", 4875))?
    .run()
    .await
}

#[get("/get_data")]
async fn handler(app_handle: web::Data<TauriAppState>) -> actix_web::Result<String> {
    let app = app_handle.app.lock().unwrap();
    let mutex: State<Data> = app.state();
    let guard = mutex.0.lock().unwrap();
    let data = guard.clone();
    return Ok(serde_json::to_string(&data).unwrap());
}

#[tauri::command(rename_all = "snake_case")]
async fn set_data(data: OverlayData, state: State<'_, Data>) -> Result<(), ()> {
    let mut dt = state.0.lock().unwrap();
    *dt = data;
    return Ok(());
}

#[tauri::command(rename_all = "snake_case")]
async fn get_data(state: State<'_, Data>) -> Result<OverlayData, ()> {
    let data = state.0.lock().unwrap();
    return Ok(data.clone());
}
