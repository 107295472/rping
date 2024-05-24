// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]
mod bll;
mod commons;

use bll::{exwaybill::*, tools::*};
use commons::global;

#[tokio::main]
async fn main() {
    global::init();
    ex_waybill_gen().await;
    #[cfg(feature = "ex_waybill")]
    {
        ex_waybill_gen().await;
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
    #[cfg(feature = "ping")]
    {
        let (_, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let _ = tokio::spawn(print()).await;
        _ = rx.recv();
    }
    println!("按任意键退出...");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    // std::process::exit(1)
}

// async fn send_api(m: AlctAPIModel) -> reqwest::Result<String> {
//     // let mut data = String::from("");
//     // data = serde_json::to_string(m.data);
//     let mut headers = header::HeaderMap::new();
//     if m.token.is_some() {
//         let auth_value =
//             header::HeaderValue::from_str(&format!("Bearer {}", m.token.unwrap())).unwrap();
//         // auth_value.set_sensitive(true);
//         headers.insert(header::AUTHORIZATION, auth_value);
//     }
//     let t = header::HeaderValue::from_str("application/json").unwrap();
//     headers.insert(header::CONTENT_TYPE, t);
//     let client = reqwest::Client::builder()
//         .default_headers(headers)
//         .timeout(std::time::Duration::from_secs(30))
//         .build()
//         .unwrap();

//     let url = format!("{}{}", m.method_domain, m.method_adress);
//     // utils::common::info("url".to_string(), url.clone());
//     if m.menthod == "post" {
//         let res = client.post(url).body(m.data).send().await;
//         match res {
//             Ok(t) => t.text().await,
//             Err(e) => {
//                 panic!("{}", e)
//             }
//         }
//     } else if m.menthod == "put" {
//         let res = client.put(url).body(m.data).send().await;
//         match res {
//             Ok(t) => t.text().await,

//             Err(e) => {
//                 panic!("{}", e)
//             }
//         }
//     } else {
//         let res = client.get(url).body(m.data).send().await;
//         match res {
//             Ok(t) => t.text().await,

//             Err(e) => {
//                 panic!("{}", e)
//             }
//         }
//     }
//     // return res;
//     // match res {
//     //     Ok(t) => t.json::<T>().await,
//     //     Err(e) => todo!(),
//     // }
// }
