/// Measurement
pub struct Measurement {
    value: f64,
}

impl Measurement {
    /// Constructor
    pub fn new() -> Measurement {
        Measurement { value: 0f64 }
    }

    /// Update value
    pub fn update(&mut self, value: f64) {
        self.value = value;
    }

    /// Get current value
    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measurement_update() {
        let mut m = Measurement::new();
        assert_eq!(m.value(), 0f64);
        m.update(10.516);
        assert_eq!(m.value(), 10.516);
    }
}
