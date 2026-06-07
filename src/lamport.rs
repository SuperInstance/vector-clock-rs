//! Lamport logical timestamps.

pub type LamportTimestamp = u64;

/// A Lamport clock tracks logical time with simple increment rules.
#[derive(Debug, Clone)]
pub struct LamportClock {
    timestamp: LamportTimestamp,
}

impl LamportClock {
    pub fn new() -> Self {
        Self { timestamp: 0 }
    }

    /// Create with initial timestamp.
    pub fn with_timestamp(ts: LamportTimestamp) -> Self {
        Self { timestamp: ts }
    }

    /// Get current timestamp.
    pub fn timestamp(&self) -> LamportTimestamp {
        self.timestamp
    }

    /// Increment before a local event.
    pub fn increment(&mut self) -> LamportTimestamp {
        self.timestamp += 1;
        self.timestamp
    }

    /// Update on receiving a message: take max + 1.
    pub fn receive(&mut self, incoming: LamportTimestamp) -> LamportTimestamp {
        self.timestamp = std::cmp::max(self.timestamp, incoming) + 1;
        self.timestamp
    }

    /// Get timestamp for sending a message (increment first).
    pub fn send(&mut self) -> LamportTimestamp {
        self.increment()
    }

    /// Compare two Lamport timestamps (partial order).
    pub fn compare(&self, other: &LamportClock) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl Default for LamportClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_timestamp() {
        let clock = LamportClock::new();
        assert_eq!(clock.timestamp(), 0);
    }

    #[test]
    fn test_increment() {
        let mut clock = LamportClock::new();
        assert_eq!(clock.increment(), 1);
        assert_eq!(clock.increment(), 2);
    }

    #[test]
    fn test_send_increments() {
        let mut clock = LamportClock::new();
        let ts = clock.send();
        assert_eq!(ts, 1);
        assert_eq!(clock.timestamp(), 1);
    }

    #[test]
    fn test_receive_higher() {
        let mut clock = LamportClock::with_timestamp(5);
        let ts = clock.receive(10);
        assert_eq!(ts, 11);
    }

    #[test]
    fn test_receive_lower() {
        let mut clock = LamportClock::with_timestamp(10);
        let ts = clock.receive(3);
        assert_eq!(ts, 11);
    }

    #[test]
    fn test_receive_equal() {
        let mut clock = LamportClock::with_timestamp(5);
        let ts = clock.receive(5);
        assert_eq!(ts, 6);
    }
}
