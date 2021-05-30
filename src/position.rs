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
    use super::*;

    #[test]
    fn switchgear_position() {
        let mut pos = SwitchgearPosition::new();

        assert!(pos.is_open());
        assert!(!pos.is_closed());
        pos.close();
        assert!(!pos.is_open());
        assert!(pos.is_closed());
        pos.open();
        assert!(pos.is_open());
        assert!(!pos.is_closed());
    }
}
