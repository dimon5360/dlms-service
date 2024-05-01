
#[derive(Clone)]
pub struct Endpoint {
    _port: String
}

impl Endpoint {
    pub fn _get_port(&self) -> &str {
        self._port.as_str()
    }
}

pub fn _new(value: String) -> Endpoint {
    Endpoint{
        _port: value,
    }
}