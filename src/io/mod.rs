
pub mod tcp;
pub mod serial;

pub struct ConfigIO {
    pub config: String
}

pub trait InterfaceIO {
    fn init(&self);
    fn run(&self);
}

pub fn create_serial_io(config: ConfigIO) -> Box<dyn InterfaceIO> {
    return serial::new(config);
}


pub fn create_tcp_io(config: ConfigIO) -> Box<dyn InterfaceIO> {
    return tcp::new(config);
}