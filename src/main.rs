// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]
mod commons;
use calamine::open_workbook;
use calamine::Reader;
use calamine::Xlsx;
use chrono::Local;
use clap::Parser;
use commons::get_config::CFG;
use dns_lookup::lookup_host;
use ftlog::appender::FileAppender;
use ftlog::appender::Period;
use ftlog::FtLogFormatter;
use log::LevelFilter;
use raw_cpuid::CpuId;
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::panic;
use std::path::PathBuf;
use std::str::FromStr;

use std::thread;

#[tokio::main]
async fn main() {
    #[cfg(feature = "init")]
    {
        init();
    }
    #[cfg(feature = "test")]
    {
        test().await;
    }
    #[cfg(feature = "cpu")]
    {
        cpu_avx().await;
    }
    #[cfg(feature = "excel")]
    {
        let args = Apprgs::parse();
        excel(args.xlsx, args.path, args.is_head).await;
    }
    // let d: String = format!("{:.2?}", 51.015);
    // println!("{}", d);

    #[cfg(feature = "ping")]
    {
        let (_, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let _ = tokio::spawn(print()).await;
        _ = rx.recv();
    }
    // println!("按任意键退出...");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    // std::process::exit(1)
}
#[derive(Parser, Debug)]
#[command(author="yin", version="1.0", about="表格文件比对工具,当前路径./开头", long_about = None)]
struct Apprgs {
    ///表格路径
    #[arg(short, long)]
    xlsx: String,

    ///文件夹路径
    #[arg(short, long)]
    path: String,
    ///是否有头 1=是
    #[arg(short, long, default_value_t = 0)]
    is_head: i32,
}

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct AlctAPIModel {
    pub token: Option<String>,
    pub method_adress: String,
    pub method_domain: String,
    pub menthod: String,
    pub data: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct IpResult {
    ip: String,
    timeout: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct CommandResult {
    name: String,
    ipv4: Vec<IpResult>,
    ipv6: Vec<IpResult>,
}
// fn query_dns_ping(addresses: Vec<&str>) -> Vec<CommandResult> {
//     let mut results = Vec::new();
//     for address in addresses {
//         let ips = lookup_host(address).unwrap_or(Vec::new());
//         let mut r = CommandResult {
//             name: address.to_string(),
//             ipv4: Vec::new(),
//             ipv6: Vec::new(),
//         };
//         for ip in ips {
//             let timeout = ping(&ip.to_string());
//             let ip_result = IpResult {
//                 ip: ip.to_string(),
//                 timeout,
//             };
//             if ip.is_ipv4() {
//                 r.ipv4.push(ip_result);
//             } else if ip.is_ipv6() {
//                 r.ipv6.push(ip_result);
//             }
//         }
//         results.push(r);
//     }
//     results
// }
// #[cfg(feature = "ping")]
async fn ping(address: &str) -> String {
    match surge_ping::ping(address.parse().unwrap(), &[0; 32]).await {
        Ok((_packet, duration)) => format!("{:.2?}", duration),
        Err(e) => format!("{:?}", e),
    }
}

async fn excel(xlsx: String, path: String, is_head: i32) {
    // let cur = std::env::current_dir();
    // let p = format!("{}\\cs\\1.xlsx", cur.unwrap().to_str().unwrap());
    // println!("{}", p.clone());
    let os_str = OsStr::new(&path);
    let rust_path = os_str.to_str().unwrap();

    let os_xlsxstr = OsStr::new(&xlsx);
    let rust_xlsx = os_xlsxstr.to_str().unwrap();

    let mut workbook: Xlsx<_> = open_workbook(rust_xlsx).expect("Cannot open file");
    let mut lists = vec![];
    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        // let mut s = String::from("");

        let mut index = 0;
        for row in range.rows() {
            if is_head == 1 {
                index += 1;
                if index != 1 && !row[0].is_empty() {
                    let rmbd = row[0].as_string().unwrap();
                    lists.push(rmbd);
                }
            } else if !row[0].is_empty() {
                let rmbd = row[0].as_string().unwrap();
                lists.push(rmbd);
            }
        }
    }
    let mut sf = vec![];
    let p = std::path::PathBuf::from_str(rust_path).unwrap();
    let entries = fs::read_dir(p).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let fp = entry.file_name();
        let ss = fp.into_string().unwrap();
        let sp = ss.clone();
        if ss.contains(".pdf") {
            let sss = sp.split('.').next().unwrap();
            sf.push(sss.to_string());
        }
    }
    // println!("{:?}", lists);
    // println!("{:?}", sf);
    let diff: Vec<_> = lists.iter().filter(|x| !sf.contains(x)).cloned().collect();
    let diff_reverse: Vec<_> = sf.iter().filter(|x| !lists.contains(x)).cloned().collect();
    let new_content = diff.join("\r\n");
    let _ = std::fs::write(format!("{}/diff.txt", path), new_content);

    let reverse_content = diff_reverse.join("\r\n");
    let _ = std::fs::write(format!("{}/diff_reverse.txt", rust_path), reverse_content);
    println!("未匹配数量: {}", diff.len());
    println!("已写入: {}/diff.txt", rust_path);

    println!("反向未匹配数量: {}", diff_reverse.len());
    println!("已写入: {}/diff_reverse.txt", rust_path);
}
// #[cfg(feature = "test")]
async fn test() {
    println!("test")
}
//cpu指令检测
// #[cfg(feature = "cpu")]
async fn cpu_avx() {
    let cpuid = CpuId::new();

    let has_sse = cpuid
        .get_feature_info()
        .map_or(false, |finfo| finfo.has_avx());
    if has_sse {
        println!("CPU支持!");
    } else {
        println!("CPU不支持!");
    }
    thread::sleep(std::time::Duration::from_secs(5));
}
// #[cfg(feature = "ping")]
async fn print() {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(CFG.system.t));
    loop {
        interval.tick().await;
        let hostname = CFG.system.pint_host.clone();
        // thread::sleep(time::Duration::from_secs(8));
        let mut list = vec![];
        for h in hostname {
            let ips = lookup_host(&h).unwrap_or_default();
            for ip in ips {
                let timeout = ping(&ip.to_string()).await;
                let fmt = "%Y-%m-%d %H:%M:%S";
                let now = Local::now().format(fmt);
                let d = Sdata {
                    name: h.to_string(),
                    avg: timeout.clone(),
                    max: 0,
                    min: 0,
                    curtime: now.to_string(),
                };
                list.push(d);
                println!("{},{}", h, timeout)
            }
            let sd = Pingdb {
                name: CFG.system.client_name.clone(),
                host: list.clone(),
            };
            let req = AlctAPIModel {
                menthod: "post".to_string(),
                token: Some("".to_string()),
                method_adress: String::from(""),
                method_domain: CFG.system.send_host.clone(),
                data: serde_json::to_string(&sd).expect("转json失败"),
            };
            // println!("sendjson:{}", req.data.clone());
            let re = send_api(req).await;
            match re {
                Ok(v) => println!("{}", v),
                Err(e) => println!("{}", e),
            }
        }
    }
}
// #[cfg(feature = "ping")]
async fn send_api(m: AlctAPIModel) -> reqwest::Result<String> {
    // let mut data = String::from("");
    // data = serde_json::to_string(m.data);
    let mut headers = header::HeaderMap::new();
    if m.token.is_some() {
        let auth_value =
            header::HeaderValue::from_str(&format!("Bearer {}", m.token.unwrap())).unwrap();
        // auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
    }
    let t = header::HeaderValue::from_str("application/json").unwrap();
    headers.insert(header::CONTENT_TYPE, t);
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap();

    let url = format!("{}{}", m.method_domain, m.method_adress);
    // utils::common::info("url".to_string(), url.clone());
    if m.menthod == "post" {
        let res = client.post(url).body(m.data).send().await;
        match res {
            Ok(t) => t.text().await,
            Err(e) => {
                panic!("{}", e)
            }
        }
    } else if m.menthod == "put" {
        let res = client.put(url).body(m.data).send().await;
        match res {
            Ok(t) => t.text().await,

            Err(e) => {
                panic!("{}", e)
            }
        }
    } else {
        let res = client.get(url).body(m.data).send().await;
        match res {
            Ok(t) => t.text().await,

            Err(e) => {
                panic!("{}", e)
            }
        }
    }
    // return res;
    // match res {
    //     Ok(t) => t.json::<T>().await,
    //     Err(e) => todo!(),
    // }
}

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct Pingdb {
    pub name: String,
    pub host: Vec<Sdata>,
}

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct Sdata {
    pub name: String,
    pub avg: String,
    pub curtime: String,
    pub min: i32,
    pub max: i32,
}
//后端初始化
pub fn init() {
    // let utc_p8_tz = UtcOffset::from_hms(8, 0, 0).unwrap();
    let mut pathstr = String::default();
    let mut log_path = String::default();
    let time_format = time::format_description::parse_owned::<1>(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]",
    )
    .unwrap();

    // if cfg!(debug_assertions) {
    //     log_path.push_str("../logs/alct.log");
    //     pathstr.push_str("../logs");
    // } else {

    // }
    log_path.push_str("./logs/alct.log");
    pathstr.push_str("./logs");
    let path = PathBuf::from(pathstr);
    if !path.exists() {
        if let Err(e) = fs::create_dir(path) {
            println!("Error creating directory: {}", e);
        }
    }
    ftlog::builder()
        // global max log level
        .max_log_level(LevelFilter::Info)
        // .fixed_timezone(utc_p8_tz)
        // custom timestamp format
        .time_format(time_format)
        // set global log formatter
        .format(FtLogFormatter)
        // use bounded channel to avoid large memory comsumption when overwhelmed with logs
        // Set `false` to tell ftlog to discard excessive logs.
        // Set `true` to block log call to wait for log thread.
        // here is the default settings
        .bounded(100_000, false) // .unbounded()
        // define root appender, pass anything that is Write and Send
        // omit `Builder::root` will write to stderr
        .root(
            FileAppender::builder()
                .path(log_path)
                .rotate(Period::Day)
                .expire(time::Duration::days(7))
                .build(),
        )
        // Do not convert to local timezone for timestamp, this does not affect worker thread,
        // but can boost log thread performance (higher throughput).
        // .utc()
        // level filter for root appender
        .root_log_level(LevelFilter::Info)
        // write logs in ftlog::appender to "./ftlog-appender.log" instead of "./current.log"
        // .filter("ftlog::appender", "ftlog-appender", LevelFilter::Error)
        // .appender("ftlog-appender", FileAppender::new("ftlog-appender.log"))
        .try_init()
        .expect("logger build or set failed");
}
