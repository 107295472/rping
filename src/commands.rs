use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    ///表格路径
    #[arg(short, long)]
    pub xlsx: String,

    ///文件夹路径
    #[arg(short, long)]
    pub path: String,
}
#[derive(Parser, Debug)]
#[command(author="y", version="1.0", about="表格文件比对工具,当前路径./开头", long_about = None)]
pub struct Apprgs {
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
