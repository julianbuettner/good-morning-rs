#[derive(Clone, Debug)]
pub enum BadMorning {
    WifiTimeout,
    IpTimeout,
    SntpError,
    SntpTimeout,
    MetoResponseFormat(String),
    HttpConnection,
}
