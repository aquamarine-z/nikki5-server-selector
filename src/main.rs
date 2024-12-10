use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::{env, fs, string};

fn delete_files(base: PathBuf) -> bool {
    let mut successful = true;
    let path_to_be_deleted = vec![
        base.clone().join("../InfinityNikki.exe"),
        base.clone().join("../product.db"),
        base.clone().join("../productVersion.json"),
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
enum ServerType {
    GLOBAL,
    CHINA,
}

fn copy_files(server_type: &ServerType) -> bool {
    let mut successful = true;
    let resource_base_path = env::current_dir().expect("Failed to get current directory");
    let resource_base_path = match server_type {
        ServerType::GLOBAL => resource_base_path.join("global"),
        ServerType::CHINA => resource_base_path.join("cn"),
    };
    let path_to_copy = vec![
        resource_base_path.clone().join("InfinityNikki.exe"),
        resource_base_path.clone().join("product.db"),
        resource_base_path.clone().join("productVersion.json"),
    ];
    let copy_destination = env::current_dir()
        .expect("Failed to get current directory")
        .parent()
        .unwrap()
        .to_owned();
    for path in path_to_copy {
        let exists = fs::exists(&path).expect("");
        if !exists {
            let file = fs::File::create(&path).expect("Failed to create file");
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
    println!("客户端文件已切换成功!");
    return successful;
}

fn open_launcher(server_type: &ServerType) -> Result<(), String> {
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

fn main() {
    println!("请输入需要切换的服务器代号");
    println!("1.国服");
    println!("2.国际服");
    let mut x = String::new();
    std::io::stdin().read_line(&mut x).expect("行号读取失败");
    let mut x = x.trim().to_string();
    let server_code = i32::from_str(&x).expect("请输入正确的数字");
    let server_type = match server_code {
        1 => ServerType::CHINA,
        2 => ServerType::GLOBAL,
        _ => {
            println!("请输入正确的数字!");
            return;
        }
    };
    let mut successful = true;
    successful = successful & delete_files(PathBuf::from("."));
    successful = successful & copy_files(&server_type);
    if successful {
        println!(
            "服务器已经切换到{}",
            match server_type {
                ServerType::CHINA => "国服",
                ServerType::GLOBAL => "国际服",
                _ => return,
            }
        );
        println!("正在打开启动器客户端......");
        if let Err(e) = open_launcher(&server_type) {
            println!("打开启动器失败:{}", e);
            println!("输入任意内容以退出:");
            std::io::stdin().read_line(&mut x).unwrap();
        }
    } else {
        println!("切换出现异常 请重新运行!");
        println!("输入任意内容以退出:");
        std::io::stdin().read_line(&mut x).unwrap();
    }

    return;
}
