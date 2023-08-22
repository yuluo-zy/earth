use std::collections::HashMap;
use std::sync::Mutex;
use crate::services::{bing, PaperInfo, PhotoService, WallpaperTrait};
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};

use once_cell::sync::Lazy;
use crate::services::bing::BingPrimitiveResources;

const BING_EXPIRE_TIME: i64 = 60 * 60 * 12;

#[derive(Clone)]
pub struct Page {
    pub type_of: PhotoService,
    pub index: Option<u8>,
    pub page_number: Option<u8>,
}

pub struct PageResult {
    pub page: Page,
    pub result: Vec<PaperInfo>,
}

impl PageResult {
    pub fn build(page: Page, result: Vec<PaperInfo>) -> Result<Self> {
        Ok(Self {
            page,
            result,
        })
    }
}

pub struct Storage {
    storage: HashMap<PhotoService, Vec<Box<dyn WallpaperTrait>>>,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            storage: HashMap::new()
        }
    }
}

impl Storage {
    /// 初始化请求缓存
    pub async fn init_storage(&mut self, type_of: &PhotoService) -> Result<()> {
        match type_of {
            PhotoService::BingDaily => {}
            PhotoService::BingList => {
                BingPrimitiveResources::init_resources(self, 0, 0).await?;
            }
            PhotoService::Pexels => {}
            PhotoService::Unsplash => {}
            PhotoService::Earth => {}
        }
        Ok(())
    }

    pub async fn get_page(&mut self, page: Page) -> Result<PageResult> {
        if let None = self.storage.get(&page.type_of) {
            self.init_storage(&page.type_of).await?;
        }
        let result = self.storage.get(&page.type_of).ok_or(anyhow!("获取缓存错误"))?;
        // TODO:: 针对查询进行数组切片
        let mut page_info = vec![];
        for item in result.iter() {
            page_info.push(item.get_wallpaper_info()?);
        }

        return PageResult::build(page.clone(), page_info);
    }

    pub fn set_storage(&mut self, key: PhotoService, value: Vec<Box<dyn WallpaperTrait>>) {
        self.storage.insert(key, value);
    }
}

// impl Cache {
//   /// update cache list
//   pub fn update_cache_list(&mut self, list: Vec<SchedulerPhoto>) {
//     self.cache_list = list;
//   }
//
//   /// get cached list, save request
//   pub fn get_cache_list() {}
//
//   /// get current photo which set as wallpaper
//   pub fn get_current_photo() {}
//
//   /// get photo list rotating
//   pub fn get_rotate_list(self) -> Vec<SchedulerPhoto> {
//     self.cache_list
//   }
//
//   pub fn rotate_to_next(&mut self) -> SchedulerPhoto {
//     if self.current_idx >= self.cache_list.len() - 1 {
//       self.current_idx = 0;
//     } else {
//       self.current_idx += 1;
//     }
//
//     self.cache_list[self.current_idx].clone()
//   }
//
//   pub fn rotate_to_previous(&mut self) -> SchedulerPhoto {
//     if self.current_idx <= 0 {
//       self.current_idx = self.cache_list.len() - 1;
//     } else {
//       self.current_idx -= 1;
//     }
//
//     self.cache_list[self.current_idx].clone()
//   }
//
//   pub fn get_random_photo(&mut self) -> SchedulerPhoto {
//     let mut rng = rand::thread_rng();
//
//     // self.current_idx = rng.gen_range(0, self.cache_list.len());
//     self.current_idx = rng.sample(Uniform::new_inclusive(0, self.cache_list.len()));
//
//     self.cache_list[self.current_idx].clone()
//   }
//
//   // cache service data
//
//   pub async fn get_bing_daily(&mut self) -> bing::Images {
//     let now = get_now_timestamp();
//
//     if !self.bing_daily.url.is_empty() && now - self.timestamp < BING_EXPIRE_TIME {
//       return self.bing_daily.clone();
//     }
//
//     let bing = services::bing::Wallpaper::new(0, 1).await.unwrap();
//
//     self.bing_daily = bing.json.images[0].clone();
//
//     self.timestamp = now;
//
//     self.bing_daily.clone()
//   }
//
//   /// get bing photo list. return cached data if not expired.
//   pub async fn get_bing_list(&mut self) -> Vec<bing::Images> {
//     let now = Utc::now().timestamp();
//
//     if !self.bing_list.is_empty() && now - self.timestamp < BING_EXPIRE_TIME {
//       return self.bing_list.clone();
//     }
//
//     let res1 = services::bing::Wallpaper::new(0, 8).await.unwrap();
//     let res2 = services::bing::Wallpaper::new(7, 8).await.unwrap();
//
//     let images1 = res1.json.images;
//     let images2 = res2.json.images;
//
//     self.bing_list = images1
//       .into_iter()
//       .chain(images2.into_iter())
//       .map(|mut i| {
//         i.url = i.url();
//         i
//       })
//       .collect();
//
//     self.timestamp = Utc::now().timestamp();
//
//     println!("timestamp: {:?}", self.timestamp);
//
//     self.bing_list.clone()
//   }
//
//   pub async fn get_pexels_list(&mut self) -> Vec<pexels::Photo> {
//     let now = Utc::now().timestamp();
//
//     if !self.pexels_list.is_empty() && now - self.timestamp < BING_EXPIRE_TIME {
//       return self.pexels_list.clone();
//     }
//
//     let pexels_client =
//       pexels::Pexels::new("s9GlfCrhK5qzYQTQjMipbIQ25spgFJnThF9n3uW73g9dge6eFzMJ7aeY".to_string());
//     let res: pexels::PexlesJSON = pexels_client.get_photo_curated(30, 1).await;
//
//     self.pexels_list = res.photos;
//
//     self.timestamp = Utc::now().timestamp();
//
//     self.pexels_list.clone()
//   }
//
//   /// update the time of last request to bing if 24 hours pasted
//   pub fn update_timestamp_if_need(&mut self) -> i64 {
//     let now = get_now_timestamp();
//
//     if now - self.timestamp > 24 * 60 * 60 {
//       self.update_timestamp();
//     }
//
//     self.timestamp
//   }
//
//   /// update the time of last request to bing
//   pub fn update_timestamp(&mut self) -> i64 {
//     let now = get_now_timestamp();
//
//     self.timestamp = now;
//
//     now
//   }
// }

// pub static STORAGE: Lazy<Mutex<Storage>> = Lazy::new(|| {
//     Mutex::new(Storage::default())
// });
