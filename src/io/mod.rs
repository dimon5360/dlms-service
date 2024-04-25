
pub mod tcp;

pub struct ConfigIO {
    pub config: String
}

pub fn create_tcp_io(config: ConfigIO) -> tcp::Tcp {
    return tcp::new(config);
}