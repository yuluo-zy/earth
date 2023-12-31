
use tauri::async_runtime::Mutex;
use tauri::State;
use tracing::{debug, error, info};
use crate::services::storage::{Page, PageResult, Storage};
//
// use crate::services::{bing, pexels, AsyncProcessMessage, PhotoService};
// use crate::{cache, config, services};
//
// use tokio::sync::{mpsc, Mutex};
//
// pub struct AsyncProcInputTx {
//   pub sender: Mutex<mpsc::Sender<AsyncProcessMessage>>,
// }
//
// #[tauri::command]
// pub async fn set_as_desktop(url: &str, service: PhotoService) -> Result<String, String> {
//   println!("set as {:?}", url);
//
//   match service {
//     PhotoService::Bing => Ok(bing::Wallpaper::set_wallpaper(url).await.unwrap()),
//     PhotoService::Pexels => Ok(pexels::Pexels::set_wallpaper(url).await.unwrap()),
//     PhotoService::Unsplash => {
//       bing::Wallpaper::set_wallpaper(url).await.unwrap();
//       Ok(String::from("asdf"))
//     }
//   }
// }
//
// #[tauri::command]
// pub async fn download(url: &str, service: PhotoService) -> Result<String, String> {
//   match service {
//     PhotoService::Bing => Ok(bing::Wallpaper::save_wallpaper(url, None).await.unwrap()),
//     PhotoService::Pexels => Ok(pexels::Pexels::save_photo(url).await.unwrap()),
//     PhotoService::Unsplash => {
//       bing::Wallpaper::set_wallpaper(url).await.unwrap();
//       Ok(String::from("asdf"))
//     }
//   }
// }
//
// #[tauri::command]
// pub async fn get_bing_wallpaper_list(_page: u8) -> Vec<bing::Images> {
//   let mut cache = cache::CACHE.lock().await;
//   let res = cache.get_bing_list().await;
//
//   res
// }
//
// #[tauri::command]
// pub async fn get_bing_daily() -> bing::Images {
//   let mut bing_daily = cache::CACHE.lock().await;
//   let res = bing_daily.get_bing_daily().await;
//
//   res
// }
//
// #[tauri::command]
// pub async fn get_pexels_curated_photos(page: u8) -> Vec<pexels::Photo> {
//   if page == 1 {
//     println!("page: {:?}", page);
//     let mut cache = cache::CACHE.lock().await;
//     let res = cache.get_pexels_list().await;
//
//     res
//   } else {
//     let pexels_client =
//       pexels::Pexels::new("s9GlfCrhK5qzYQTQjMipbIQ25spgFJnThF9n3uW73g9dge6eFzMJ7aeY".to_string());
//     let res = pexels_client.get_photo_curated(30, page).await;
//
//     res.photos
//   }
// }
//
// #[tauri::command]
// pub async fn get_config() -> serde_json::Value {
//   let pavo_config = config::PavoConfig::get_config();
//
//   serde_json::to_value(pavo_config).unwrap()
// }
//
// #[tauri::command]
// #[allow(unused)]
// pub async fn set_auto_rotate(
//   rotate: bool,
//   state: tauri::State<'_, AsyncProcInputTx>,
// ) -> Result<(), ()> {
//   let pavo_config = config::PavoConfig::get_config();
//
//   pavo_config.set_auto_rotate(rotate);
//
//   let async_proc_input_tx = state.sender.lock().await;
//
//   if rotate {
//     async_proc_input_tx
//       .send(AsyncProcessMessage::StartRotate)
//       .await
//       .map_err(|e| e.to_string());
//   } else {
//     async_proc_input_tx
//       .send(AsyncProcessMessage::StopRotate)
//       .await
//       .map_err(|e| e.to_string());
//   }
//
//   Ok(())
// }
//
// #[tauri::command]
// pub async fn set_interval(interval: u64) {
//   let pavo_config = config::PavoConfig::get_config();
//
//   println!("{:?}", interval);
//
//   pavo_config.set_interval(interval);
// }
//
// #[tauri::command]
// pub async fn set_randomly(randomly: bool) {
//   let pavo_config = config::PavoConfig::get_config();
//
//   pavo_config.set_randomly(randomly);
// }
//
// #[tauri::command]
// pub async fn set_rotate_source(source: String, checked: bool) {
//   let pavo_config = config::PavoConfig::get_config();
//
//   pavo_config.set_rotate_source(source, checked);
// }
//
// #[tauri::command]
// pub async fn view_photo(handle: tauri::AppHandle, href: String) {
//   services::view_photo(handle, href);
// }


#[tauri::command]
pub async fn get_photo<'a>(page: Page, storage:State<'a, Mutex<Storage>>) -> Result<PageResult, String> {
   info!("get photo: {:?}", page);
   let mut cache = storage.lock().await;
    match cache.get_page(page).await {
       Ok(res) => {Ok(res)}
       Err(error) => {
          error!("{:?}", error);
          Err(String::from("get photo error"))
       }
    }
}
