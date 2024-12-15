use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::{env, fs, string};
use serde_json::json;
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK};
pub fn delete_files(base: PathBuf) -> bool {
    let mut successful = true;
    let path_to_be_deleted = vec![
        base.clone().join("../InfinityNikki.exe"),
        //base.clone().join("../product.db"),
        //base.clone().join("../productVersion.json"),
    ];
    for path in path_to_be_deleted {
        if path.exists() {
            if let Err(e) = std::fs::remove_file(&path) {
                successful = false;
                println!("Failed to delete file {}: {}", &path.display(), e);
            }
        }
    }
    return successful;
}
pub enum ServerType {
    GLOBAL,
    CHINA,
}

pub fn copy_files(server_type: &ServerType) -> bool {
    let mut successful = true;
    let resource_base_path = env::current_dir().expect("Failed to get current directory");
    let resource_base_path = match server_type {
        ServerType::GLOBAL => resource_base_path.join("global"),
        ServerType::CHINA => resource_base_path.join("cn"),
    };
    let path_to_copy = vec![
        resource_base_path.clone().join("InfinityNikki.exe"),
        //resource_base_path.clone().join("product.db"),
        //resource_base_path.clone().join("productVersion.json"),
    ];
    let copy_destination = env::current_dir()
        .expect("Failed to get current directory")
        .parent()
        .unwrap()
        .to_owned();
    for path in path_to_copy {
        let exists = fs::exists(&path).expect("");
        if !exists {
            if let Err(e) = fs::File::create(&path) {
                return false;
            }
        }
        if let Err(e) = std::fs::copy(&path, &copy_destination.join(path.file_name().unwrap())) {
            successful = false;
            println!(
                "从 {} 复制文件到 {} 失败 : {}",
                &path.display(),
                &copy_destination.display(),
                e
            );
        }
    }
    let product_database_path=copy_destination.clone().join("product.db");
    if !product_database_path.exists(){
        if let Err(e) = fs::File::create(&product_database_path) {
            return false;
        }
    }
    let database_info_object=json!({
        "name":match server_type{
            ServerType::CHINA=>"InfinityNikki Launcher",
            ServerType::GLOBAL=>"InfinityNikkiGlobal Launcher",
            _=>{return false;},
        },
        "version":"341" 
    });
    println!("客户端文件已切换成功!");
    return successful;
}

pub fn open_launcher(server_type: &ServerType) -> Result<(), String> {
    let launcher_path = match server_type {
        ServerType::GLOBAL => env::current_dir()
            .expect("Failed to get current directory")
            .join("global")
            .join("Launcher")
            .join("launcher.exe"),
        ServerType::CHINA => env::current_dir()
            .expect("Failed to get current directory")
            .join("cn")
            .join("Launcher")
            .join("launcher.exe"),
    };
    if let Err(e) = Command::new(launcher_path.as_os_str()).spawn() {
        return Err(e.to_string());
    }
    Ok(())
}
pub fn select_and_open(server_type: &ServerType) {
    let mut successful = true;
    successful = successful && delete_files(PathBuf::from("."));
    successful = successful & copy_files(&ServerType::GLOBAL);
    if !successful {
        let text_wide_content: Vec<u16> = "服务器切换失败 请重新启动尝试!"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let text_wide_title: Vec<u16> = "错误".encode_utf16().chain(std::iter::once(0)).collect();
        unsafe {
            MessageBoxW(
                HWND(std::ptr::null_mut()), // 父窗口句柄，0 表示无父窗口
                PCWSTR(text_wide_content.as_ptr()),
                PCWSTR(text_wide_title.as_ptr()),
                MB_OK,
            );
        }
    } else {
        if let Err(e) = open_launcher(&ServerType::GLOBAL) {
            let text_wide_content: Vec<u16> = "启动器开启失败 请重新启动尝试!"
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            let text_wide_title: Vec<u16> =
                "错误".encode_utf16().chain(std::iter::once(0)).collect();
            unsafe {
                MessageBoxW(
                    HWND(std::ptr::null_mut()), // 父窗口句柄，0 表示无父窗口
                    PCWSTR(text_wide_content.as_ptr()),
                    PCWSTR(text_wide_title.as_ptr()),
                    MB_OK,
                );
            }
        }
    }
}
