use clap::Parser;

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
