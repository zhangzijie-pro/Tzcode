// as linux "apt" to download some file or software

// download file in web such as : http:*********
// maybe there are some error 
use std::fs::File;
use std::io::{copy, Write};
use std::path::PathBuf;
use async_std::task;
use reqwest::Client;

use super::command::xvf;

#[allow(dead_code)]
pub struct Package{
    name: String,
    version: String,
    download_link: String,
}

// apt install 


// upload soon
impl Package{
    pub  fn new(name: String, version: String, download_link: String) -> Package{
        Package{
            name,
            version,
            download_link
        }
    }
}

pub fn find_package(name: &str) -> Option<Package>{
    match name {
        "tree" => Some(Package::new("tree".to_owned(), "1.85.0".to_owned(), "http://mama.indstate.edu/users/ice/tree/src/tree-1.8.0.tgz".to_owned())),
        _ => None
    }
}

pub fn download_package(package: &Package) -> Result<(),Box<dyn std::error::Error>>{
    let _ = download(&package.download_link, &format!("{}.tar.gz",package.name));
    // 解压缩并安装
    let file = format!("{}.tar.gz", package.name);
    let _ = xvf(&file).expect("Failed to install package.");

    Ok(())
}

async fn download(link: &str, filename: &str) -> Result<(),Box<dyn std::error::Error>> {
    let cilent = Client::new();

    let response = cilent.get(link).send().await.unwrap();
    println!("{:?}",response);
    if !response.status().is_success(){
        eprint!("Fail to get Response")
    }
    let mut file = File::create(filename).unwrap();

    copy(&mut response.bytes().await.unwrap().as_ref(), &mut file)?;
    Ok(())
}


// apt update new

const _GITHUB_RELEASE_LINUX: &str = "https://github.com/zhangzijie-pro/Tiks/releases/download/1.0.3/tiks";
const _GITHUB_RELEASE_WINDOW: &str = "https://github.com/zhangzijie-pro/Tiks/releases/download/1.0.3/tiks.exe";

// upload soon
#[allow(unused_must_use)]
pub fn update(version: &str) -> std::io::Result<()>{
    let version = version;
    let _release_linux = format!("https://github.com/zhangzijie-pro/Tiks/releases/download/{}/tiks",version);
    let _release_window = format!("https://github.com/zhangzijie-pro/Tiks/releases/download/{}/tiks.exe",version);

    if cfg!(target_os = "linux") | cfg!(target_os="macos"){
        let app_linux = get_linux_dir();
        task::block_on(async {
            update_to(&_release_linux, app_linux).await;
        });
    }

    if cfg!(target_os="windows"){
        let app_window = get_window_dir();
        task::block_on(async {
            update_to(&_release_window, app_window).await;
        });
    }
    
    Ok(())
}


pub async fn update_last() -> Result<(), Box<dyn std::error::Error>>{
    let client = Client::new();

    if cfg!(target_os = "linux") | cfg!(target_os="macos"){
        let response = client.get(_GITHUB_RELEASE_LINUX).send().await.expect("Error: update error");
        let app = get_linux_dir();
        let mut file = File::create(app).expect("Can't create file : tiks");

        let byte = response.bytes().await.unwrap();
        let _ = file.write_all(&byte);
    }

    if cfg!(target_os="windows"){ 
        let response = client.get(_GITHUB_RELEASE_WINDOW).send().await.expect("Error: update error");
        let app = get_window_dir();
        let mut file = File::create(app).expect("Can't create file : tiks");

        let byte = response.bytes().await.unwrap();
        let _ = file.write_all(&byte);
    }

    Ok(())
}

async fn update_to(url: &str,filename: PathBuf) -> Result<(), Box<dyn std::error::Error>>{
    let client = Client::new();
    
    let response = client.get(url).send().await.expect("Error: Can't download file");

    let mut file = File::create(filename).expect("Can't create file : tiks");
   let byte = response.bytes().await.unwrap();
    let _ = file.write_all(&byte);

    Ok(())

}


fn get_window_dir() -> PathBuf{
    let home = dirs::home_dir().unwrap();
    let app_dir = home.join(".Tiks");
    let app_window = app_dir.join("bin").join("tiks.exe");

    app_window
}

fn get_linux_dir() -> PathBuf{
    let home = dirs::home_dir().unwrap();
    let app_dir = home.join(".Tiks");
    let app_linux = app_dir.join("bin").join("tiks");

    app_linux
}