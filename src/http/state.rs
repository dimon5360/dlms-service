use crate::dlms;
pub trait Interface: Send + Sync {
    fn dlms(&self) -> &Box<dyn dlms::service::Interface>;
}

struct State {
    _dlms: Box<dyn dlms::service::Interface>,
}

impl Interface for State {
    fn dlms(&self) -> &Box<dyn dlms::service::Interface> {
        return &self._dlms;
    }
}

pub fn new(dlms: Box<dyn dlms::service::Interface>) -> Box<dyn Interface> {
    Box::new(State { _dlms: dlms })
}
