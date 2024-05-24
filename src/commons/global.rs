use std::{fs, panic, path::PathBuf};

use ftlog::{
    appender::{Duration as du, FileAppender, Period},
    FtLogFormatter, LevelFilter,
};
use log::error;

use super::common;

//初始化
pub fn init() {
    // let utc_p8_tz = UtcOffset::from_hms(8, 0, 0).unwrap();
    // let mut pathstr = String::default();
    let mut log_path = String::default();
    let time_format = time::format_description::parse_owned::<1>(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]",
    )
    .unwrap();

    log_path.push_str("./logs");

    let path = PathBuf::from(&log_path);
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
                .path(format!("{}/alct.log", log_path))
                .rotate(Period::Day)
                .expire(du::days(10))
                .build(),
        )
        // Do not convert to local timezone for timestamp, this does not affect worker thread,
        // but can boost log thread performance (higher throughput).
        // .utc()
        // level filter for root appender
        // .root_log_level(LevelFilter::Info)
        // // write logs in ftlog::appender to "./ftlog-appender.log" instead of "./current.log"
        // .filter("tao", "tao-appender", LevelFilter::Error)
        // .appender(
        //     "tao-appender",
        //     FileAppender::builder()
        //         .path(format!("{}/err.log", log_path))
        //         .rotate(Period::Day)
        //         .expire(Duration::days(1))
        //         .build(),
        // )
        .try_init()
        .expect("logger build or set failed");
    panic::set_hook(Box::new(move |panic_info| {
        error!("{}", panic_info.to_string());
        common::sleep_sync(1);
        // msg(Rescode {
        //     code: 1,
        //     message: format!("异常请查看日志:{}", panic_info),
        //     // message: "".to_string(),
        // });
        // 将 panic 信息写入文件中
        // let e = format!("info:{},\nLocation: {}", panic_info, panic_info.location().unwrap());
        // info!("log init");
        // 继续 panic，以便程序退出
        std::process::exit(1);
    }));
    common::sleep_sync(2);
}
