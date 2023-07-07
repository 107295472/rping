use log::{error, info};
pub fn error(e: String) {
    error!("{}", e)
}
pub fn info(flag: &str, e: String) {
    info!("{}:{}", flag, e)
}
