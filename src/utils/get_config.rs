use std::{fs::File, io::Read};

use once_cell::sync::Lazy;

use super::cfgs::Configs;
const CFG_FILE: &str = "config.toml";
//  只要是配置文件中的配置项，都可以通过这个结构体来获取，
// 只要读取一次值后保存到内存，一直可供使用
pub static CFG: Lazy<Configs> = Lazy::new(self::Configs::init);

impl Configs {
    pub fn init() -> Self {
        let mut file = match File::open(CFG_FILE) {
            Ok(f) => f,
            Err(e) => {
                panic!("不存在配置文件：{}，错误信息：{}", CFG_FILE, e)
            }
        };
        let mut cfg_contents = String::new();
        match file.read_to_string(&mut cfg_contents) {
            Ok(s) => s,
            Err(e) => panic!("读取配置文件失败，错误信息：{}", e),
        };
        toml::from_str(&cfg_contents).expect("解析配置文件错误")
    }
}
// lazy_static! {
//     static ref ALCT_USERINFO: RwLock<Option<UserInfo>> = RwLock::new(None);
// }
// pub fn user_set(res: UserInfo) {
//     let mut guard = ALCT_USERINFO.write();
//     *guard = Some(res);
//     drop(guard);
// }
// pub fn user_get() -> UserInfo {
//     return ALCT_USERINFO.read().clone().unwrap();
// }
