use std::{fs::File, io::Write};

use log::{error, info};
pub fn error(e: String) {
    error!("{}", e)
}
pub fn info(flag: &str, e: String) {
    info!("{}:{}", flag, e)
}
pub fn write_file(fname: String, b: &Vec<u8>) {
    if b.is_empty() {
        return;
    }
    let mut file = match File::create(fname) {
        Ok(file) => file,
        Err(error) => panic!("Failed to create file: {}", error),
    };
    // 将字符串写入文件
    match file.write_all(b) {
        Ok(_) => println!("File written successfully."),
        Err(error) => panic!("Failed to write to file: {}", error),
    }
}
