// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]
mod utils;
use chrono::prelude::*;
use dns_lookup::lookup_host;
use flexi_logger::{DeferredNow, FileSpec, Logger, WriteMode};
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::panic;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use tokio::time;
use utils::get_config::CFG;

use crate::utils::common;
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

async fn ping(address: &str) -> String {
    match surge_ping::ping(address.parse().unwrap(), &[0; 32]).await {
        Ok((_packet, duration)) => format!("{:.2?}", duration),
        Err(e) => format!("{:?}", e),
    }
}
#[tokio::main]
async fn main() {
    init();
    println!("program start");
    let (_, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    // print().await;
    let _ = tokio::spawn(print()).await;
    _ = rx.recv();
}
async fn print() {
    let mut interval = time::interval(time::Duration::from_secs(CFG.system.t));
    loop {
        interval.tick().await;
        let hostname = CFG.system.pint_host.clone();
        // thread::sleep(time::Duration::from_secs(8));
        let mut list = vec![];
        for h in hostname {
            let ips = lookup_host(&h).unwrap_or(Vec::new());
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
        .timeout(Duration::from_secs(20))
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
//后端初始化
pub fn init() {
    const TS_USCORE_DASHES_USCORE_DASHES: &str = "_%Y-%m-%d";
    let f = DeferredNow::new()
        .format(TS_USCORE_DASHES_USCORE_DASHES)
        .to_string();
    let s: FileSpec = FileSpec::default()
        .basename(format!("log{}", f))
        .directory("logs")
        .use_timestamp(false);
    // println!("{}", s);
    let _ = Logger::try_with_str("info")
        .unwrap()
        .log_to_file(s)
        .append()
        .write_mode(WriteMode::BufferAndFlush)
        .start();

    // info!("98546545");
    panic::set_hook(Box::new(move |panic_info| {
        common::error(panic_info.to_string());
        println!("error:{}", panic_info);
        thread::sleep(Duration::from_secs(2));
        // 将 panic 信息写入文件中
        // let e = format!(
        //     "info:{},\nLocation: {}",
        //     panic_info,
        //     panic_info.location().unwrap()
        // );

        // 继续 panic，以便程序退出
        std::process::exit(1);
    }));
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
