use crate::utils::{copy_files, delete_files, load_config, open_launcher, ServerType};
use std::path::PathBuf;
use std::str::FromStr;

mod utils;
fn main() {
    load_config();
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
