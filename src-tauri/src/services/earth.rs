use std::sync::Mutex;
use image::{GenericImage, RgbaImage};
use anyhow::Result;
use std::{ops::Sub};
use chrono::{Datelike, DateTime, FixedOffset, NaiveDateTime, Timelike, TimeZone, Utc};
use once_cell::sync::Lazy;
use tracing::info;

//
// fn set_wallpaper<C:Fn(u32, u32) + 'static>(width: u32, height: u32, half: bool, callback: C) -> Result<()>{
//     let mut cfg = config::load();
//     //保存原有壁纸路径
//     if cfg.old_wallpaper.len() == 0{
//         if let Ok(old) = wallpaper::get(){
//             cfg.old_wallpaper = old;
//             config::save(&mut cfg);
//         }
//     }
//     //创建一张黑色背景图片
//     let mut paper = RgbaImage::new(width, height);
//     paper.pixels_mut().for_each(|p| *p = Rgba([0, 0, 0, 255]));
//     let d = if height > 1080||half { 4 }else{ 2};
//     let image =
//         match cfg.satellite_name.as_str(){
//             "h8" => h8::download_lastest(&mut cfg, d, callback)?,
//             _ => fy4a::download_lastest(&mut cfg, d, callback)?,
//         };
//     if image.is_none(){
//         return Ok(());
//     }
//     let image = image.unwrap();
//     // image.save("test.png").unwrap();
//     // 图片稍微缩小一点
//     let scale = if !half{
//         (height as f32 * 0.9) / image.height() as f32
//     }else{
//         (width as f32 * 0.95) / image.width() as f32
//     };
//     let mut image = resize(&image, (image.width() as f32 * scale) as u32, (image.height() as f32*scale) as u32, image::imageops::FilterType::Lanczos3);
//
//     // 复制到桌面背景中
//     if half{
//         //复制上半块
//         let x = (paper.width()-image.width())/2;
//         let y = (paper.height() as f32 * 0.05) as u32;
//         //要复制的高度
//         let ch = paper.height()-y;
//         //要复制的图像
//         image = image.sub_image(0, 0, image.width(), ch).to_image();
//         paper.sub_image(x, y, image.width(), image.height()).copy_from(&image, 0, 0)?;
//     }else{
//         let x = (paper.width()-image.width())/2;
//         let y = (paper.height()-image.height())/2;
//         paper.sub_image(x, y, image.width(), image.height()).copy_from(&image, 0, 0)?;
//     }
//
//     let wallpaper_file_path = get_wallpaper_file_path();
//     paper.save(&wallpaper_file_path)?;
//     cfg.current_wallpaper_file = wallpaper_file_path.clone();
//
//     // 生成略缩图
//     let thumb_scale = 500. / image.width() as f32;
//     let (thumb_width, thumb_height) = (image.width() as f32 * thumb_scale, image.height() as f32 * thumb_scale);
//     let thumb = resize(&image, thumb_width as u32, thumb_height as u32, image::imageops::FilterType::Triangle);
//     let thumb_path = format!("{}\\thumbnail.png", get_app_home_dir());
//     if let Ok(_) = thumb.save(&thumb_path){
//         if let Ok(mut f) = File::open(thumb_path){
//             let mut png_data = vec![];
//             let _ = f.read_to_end(&mut png_data);
//             if png_data.len() > 0{
//                 cfg.current_wallpaper_thumbnail = Some(base64::encode(&png_data));
//             }
//         }
//     }
//
//     config::save(&mut cfg);
//
//     // 设置锁屏
//     let loc_res = app::set_lock_screen_image(&wallpaper_file_path);
//     info!("锁屏设置结果: {:?}", loc_res);
//
//     // 设置壁纸
//     match wallpaper::set_from_path(&wallpaper_file_path){
//         Ok(()) => Ok(()),
//         Err(err) => {
//             Err(anyhow!("{:?}", err))
//         }
//     }
// }
//
// pub fn set_wallpaper_default(){
//     {
//         if let Ok(mut d) = IS_DOWNLOADING.lock(){
//             *d = true;
//         }
//     }
//     let cfg = config::load();
//     // 获取屏幕宽高
//     let (screen_width, screen_height) = get_screen_size();
//     //下载最新壁纸
//     if let Err(err) = set_wallpaper(screen_width as u32, screen_height as u32, cfg.display_type==2, |i,t|{
//         info!("正在下载: {}/{}", i, t);
//     }){
//         error!("壁纸下载失败: {:?}", err);
//     }
//     {
//         if let Ok(mut d) = IS_DOWNLOADING.lock(){
//             *d = false;
//         }
//     }
// }
//
// pub fn is_downloading() -> bool{
//     if let Ok(d) = IS_DOWNLOADING.lock(){
//         *d
//     }else{
//         false
//     }
// }
//
// pub fn set_wallpaper_default_async(){
//     thread::spawn(move ||{
//         set_wallpaper_default();
//     });
// }
//
// fn get_wallpaper_file_path() -> String {
//     let wallpaper_path_name = format!( "{}\\wallpaper.png", get_app_home_dir());
//     info!("wallpaper {:?}", wallpaper_path_name);
//     wallpaper_path_name
// }
//
// fn get_app_home_dir() -> String {
//     let mut app_home_dir = String::from(".");
//     if let Some(home_dir) = dirs::home_dir(){
//         if let Some(home_dir) = home_dir.to_str(){
//             let app_home_dir_tmp = format!("{}\\{}", home_dir, APP_NAME_E);
//             if Path::exists(Path::new(&app_home_dir_tmp)){
//                 app_home_dir = app_home_dir_tmp;
//             }else{
//                 if let Ok(()) = create_dir(&app_home_dir_tmp){
//                     app_home_dir = app_home_dir_tmp;
//                 }
//             }
//         }
//     }
//     info!("app_home_dir {}", app_home_dir);
//     app_home_dir
// }
pub const DEFAULT_DOWNLOAD_URL_FY4A: &str = "http://rsapp.nsmc.org.cn/swapQuery/public/tileServer/getTile/fy-4a/full_disk/NatureColor/";
pub const DEFAULT_DOWNLOAD_URL_H8: &str = "https://himawari8.nict.go.jp/img/";

#[derive(Copy, Clone)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl From<&DateTime<Utc>> for Date {
    fn from(value: &DateTime<Utc>) -> Self {
        Self {
            year: value.year(),
            month: value.month(),
            day: value.day(),
            hour: value.hour(),
            minute: value.minute(),
        }
    }
}

// #[async_trait]
// pub trait Satellite {
//     async fn download(&self, d: u32, date: Date) -> Result<RgbaImage>;
//     async fn download_img(&self, url: &str) -> Result<RgbaImage>;
//     async fn download_latest(&self, d: u32) -> Result<Option<RgbaImage>>;
// }
#[derive(PartialEq)]
pub enum SatelliteType {
    FengYun,
    XiangRIKui,
}

impl SatelliteType {
    pub fn format_url(&self, date: Date,
                      d: u32, x: u32, y: u32, ) -> String {
        match self {
            SatelliteType::FengYun => {
                format!("{}{}{:02}{:02}{:02}{:02}00/jpg/{}/{}/{}.png",
                        DEFAULT_DOWNLOAD_URL_FY4A, date.year, date.month, date.day,
                        date.hour, date.minute, d / 2, x, y)
            }

            SatelliteType::XiangRIKui => {
                format!("{}D531106/{}d/550/{}/{:02}/{:02}/{:02}{}000_{}_{}.png",
                        DEFAULT_DOWNLOAD_URL_H8, d, date.year, date.month, date.day,
                        date.hour, date.minute / (10 * 10), x, y)
            }
        }
    }


    async fn download_img(url: &str) -> Result<RgbaImage> {
        info!("download_image {}", url);
        let response = reqwest::get(url).await?;
        let image_data = response.bytes().await?;
        let img = image::load_from_memory(image_data.as_ref())?.to_rgba8();
        Ok(img)
    }

    async fn download(&self, d: u32, date: Date) -> Result<RgbaImage> {
        let mut images = vec![];
        for y in 0..d {
            for x in 0..d {
                images.push(
                    Self::download_img(&self.format_url(date, d, x, y)).await?
                );
            }
        }

        let (width, height) = (images[0].width(), images[0].height());
        let mut big_img = RgbaImage::new(width * d, height * d);
        for x in 0..d {
            for y in 0..d {
                let img = images.remove(0);
                big_img.sub_image(x * width, y * height,
                                  img.width(), img.height())
                    .copy_from(&img, 0, 0)?;
            }
        }
        Ok(big_img)
    }


    async fn download_latest(&self, d: u32) -> Result<Option<RgbaImage>> {
        let mut time;
        match self {
            SatelliteType::FengYun => {
                let now = Utc::now();
                let hour: DateTime<Utc>= Utc.with_ymd_and_hms(now.year(), now.month(), now.day(),
                                                              now.hour(), (now.minute() / 15) * 15, 0).unwrap();
                time = hour.sub(FixedOffset::east_opt(60 * 15).unwrap());

                if self == &SatelliteType::XiangRIKui {
                    time = time.sub(FixedOffset::east_opt(60 * 5).unwrap());
                }

                let mut try_times = 0;
                if self == &SatelliteType::FengYun {
                    while try_times < 4 {
                        //尝试下载最新一张图片, 递减15分钟
                        if Self::download_img(
                            &self.format_url(Date::from(&time), 1, 0, 0)
                        ).await.is_err() {
                            info!("卫星图片不存在，尝试下载更早的图片.");
                            time = time.sub(FixedOffset::east_opt(60 * 15).unwrap());
                            try_times += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
            SatelliteType::XiangRIKui => {
                let mut timestamp = Utc::now().timestamp_millis();
                //减去20分钟
                timestamp -= 20 * 60 * 1000;
                time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(timestamp/1000, 0).unwrap(), Utc);
            }
        }


        let res = self.download(d, Date::from(&time)).await?;
        Ok(Some(res))
    }
}