use uuid::Uuid;

pub struct Config {
    pub uuid: Uuid,
    pub host: String,
    pub proxy_addr: String,
    pub proxy_port: u16,
}
