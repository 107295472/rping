use chrono::Local;
use std::thread::sleep;
use std::{
    fs::File,
    io::{Read, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use super::get_config::CFG;
//连接redis
pub fn get_redis() -> redis::RedisResult<redis::Connection> {
    let url = format!(
        "redis://:{}@{}:{}",
        CFG.system.redis_pwd, CFG.system.redis_ip, CFG.system.reids_port
    );
    // info!("redis-url:{}", url);
    // info!("pwd:{}", CFG.system.redis_pwd);
    let client = redis::Client::open(url)?;
    let con = client.get_connection()?;
    // match con {
    //     Ok(t) => t,
    //     Err(e) => info!("{}", e),
    // }
    Ok(con)
}
// ping id
pub async fn ping(address: &str) -> String {
    match surge_ping::ping(address.parse().unwrap(), &[0; 32]).await {
        Ok((_packet, duration)) => format!("{:.2?}", duration),
        Err(e) => panic!("ocr主机不可访问:{}", e),
    }
}
pub fn sleep_sync(sec: u64) {
    let sec_d = std::time::Duration::from_secs(sec);
    sleep(sec_d);
}
pub async fn sleep_async(sec: u64) {
    let sec_d = std::time::Duration::from_secs(sec);
    tokio::time::sleep(sec_d).await;
}
pub async fn sleep_async_dur(sec_d: std::time::Duration) {
    // let sec_d = std::time::Duration::from_secs(sec);
    tokio::time::sleep(sec_d).await;
}
pub async fn parse_url(url: String) -> (String, i32) {
    // let urlstr = url.clone();
    // let ip1 = url.split(':');
    let words = url.split(':').collect::<Vec<&str>>();
    let mut index = 0;
    let mut val = vec![];
    for word in words.clone() {
        index += 1;
        if index == 1 {
            continue;
        }
        val.push(word);
    }
    // println!("{:?}", val.clone());
    // ("3333".to_string(), 3333)
    (val[0][2..].to_string(), val[1].parse().unwrap())
}
pub fn write_file(fname: String, b: &[u8]) {
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
pub fn read_file(fname: String) -> Vec<u8> {
    let mut file1 = match File::open(fname) {
        Ok(file) => file,
        Err(error) => panic!("Failed to open file: {}", error),
    };
    let mut contents = vec![];
    match file1.read_to_end(&mut contents) {
        Ok(_) => contents,
        Err(error) => panic!("Failed to read file: {}", error),
    }
}
///时间戳-单位秒或毫秒
pub fn get_timestamp(is_sec: bool) -> i64 {
    let current_time = SystemTime::now();
    let since_epoch = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // 以秒为单位
    if is_sec {
        let timestamp = since_epoch.as_secs();
        timestamp as i64
    } else {
        let timestamp = since_epoch.as_millis();
        timestamp as i64
    }
}
pub fn get_time_str() -> String {
    let now = Local::now();
    let time_str = now.format("%Y%m%d%H%M%S").to_string();
    time_str
}
