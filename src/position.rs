#[derive(PartialEq)]
enum SwitchPosition {
    Open,
    Close,
}

/// Switchgear Position
pub struct SwitchgearPosition {
    position: SwitchPosition,
}

impl SwitchgearPosition {
    /// Constructor
    pub fn new() -> SwitchgearPosition {
        SwitchgearPosition {
            position: SwitchPosition::Open,
        }
    }

    /// True if closed
    pub fn is_closed(&self) -> bool {
        self.position == SwitchPosition::Close
    }

    /// True if open
    pub fn is_open(&self) -> bool {
        self.position == SwitchPosition::Open
    }

    /// Close and error if already closed
    pub fn close(&mut self) -> Result<(), String> {
        if self.is_closed() {
            Err("Switchgear already closed".to_string())
        } else {
            self.position = SwitchPosition::Close;
            Ok(())
        }
    }

    /// Open and error if already open
    pub fn open(&mut self) -> Result<(), String> {
        if self.is_open() {
            Err("Switchgear already open".to_string())
        } else {
            self.position = SwitchPosition::Open;
            Ok(())
        }
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
        pos.close().unwrap();
        assert!(pos.close().is_err());
        assert!(!pos.is_open());
        assert!(pos.is_closed());
        pos.open().unwrap();
        assert!(pos.open().is_err());
        assert!(pos.is_open());
        assert!(!pos.is_closed());
    }
}
