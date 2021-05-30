#[derive(PartialEq)]
enum SwitchPosition {
    Open,
    Close,
}

pub struct SwitchgearPosition {
    position: SwitchPosition,
}

impl SwitchgearPosition {
    pub fn new() -> SwitchgearPosition {
        SwitchgearPosition {
            position: SwitchPosition::Open,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.position == SwitchPosition::Close
    }

    pub fn is_open(&self) -> bool {
        self.position == SwitchPosition::Open
    }

    pub fn close(&mut self) {
        self.position = SwitchPosition::Close;
    }

    pub fn open(&mut self) {
        self.position = SwitchPosition::Open;
    }
}

#[cfg(test)]
mod tests {
    use super::super::component::{CircuitBreaker, Disconnector, EarthingSwitch};

    #[test]
    fn switchgear_position() {
        let mut cb = CircuitBreaker::new();
        let mut ds = Disconnector::new();
        let mut es = EarthingSwitch::new();

        assert!(cb.position.is_open());
        cb.position.close();
        assert!(cb.position.is_closed());
        cb.position.open();
        assert!(cb.position.is_open());

        assert!(ds.position.is_open());
        ds.position.close();
        assert!(ds.position.is_closed());
        ds.position.open();
        assert!(ds.position.is_open());

        assert!(es.position.is_open());
        es.position.close();
        assert!(es.position.is_closed());
        es.position.open();
        assert!(es.position.is_open());
    }
}
