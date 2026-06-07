//! Conflict detection between concurrent events.

use crate::vector::VectorClock;

/// Result of conflict detection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictResult {
    /// No conflict: one happened before the other.
    Ordered { first: usize, second: usize },
    /// Conflict: events are concurrent.
    Conflict { a: usize, b: usize },
    /// Identical clocks — same event or merge.
    Identical,
}

/// Detects conflicts between events using vector clocks.
#[derive(Debug)]
pub struct ConflictDetector {
    events: Vec<VectorClock>,
}

impl ConflictDetector {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Register an event's vector clock, returning its index.
    pub fn register(&mut self, clock: VectorClock) -> usize {
        let idx = self.events.len();
        self.events.push(clock);
        idx
    }

    /// Check for conflict between two registered events.
    pub fn check(&self, a: usize, b: usize) -> ConflictResult {
        let vc_a = &self.events[a];
        let vc_b = &self.events[b];

        if vc_a == vc_b {
            return ConflictResult::Identical;
        }
        if vc_a.happened_before(vc_b) {
            return ConflictResult::Ordered { first: a, second: b };
        }
        if vc_b.happened_before(vc_a) {
            return ConflictResult::Ordered { first: b, second: a };
        }
        ConflictResult::Conflict { a, b }
    }

    /// Find all conflicting pairs among registered events.
    pub fn find_all_conflicts(&self) -> Vec<(usize, usize)> {
        let mut conflicts = Vec::new();
        for i in 0..self.events.len() {
            for j in (i + 1)..self.events.len() {
                if let ConflictResult::Conflict { a: _, b: _ } = self.check(i, j) {
                    conflicts.push((i, j));
                }
            }
        }
        conflicts
    }

    /// Number of registered events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Is the detector empty?
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for ConflictDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_clock(pairs: &[(u64, u64)]) -> VectorClock {
        let mut vc = VectorClock::new();
        for &(pid, ts) in pairs {
            for _ in 0..ts {
                vc.increment(pid);
            }
        }
        vc
    }

    #[test]
    fn test_ordered_events() {
        let mut det = ConflictDetector::new();
        let mut vc1 = VectorClock::new();
        vc1.increment(1);
        let mut vc2 = vc1.clone();
        vc2.increment(2);
        let a = det.register(vc1);
        let b = det.register(vc2);
        assert_eq!(det.check(a, b), ConflictResult::Ordered { first: 0, second: 1 });
    }

    #[test]
    fn test_conflicting_events() {
        let mut det = ConflictDetector::new();
        let mut vc1 = VectorClock::new();
        vc1.increment(1);
        let mut vc2 = VectorClock::new();
        vc2.increment(2);
        let a = det.register(vc1);
        let b = det.register(vc2);
        assert_eq!(det.check(a, b), ConflictResult::Conflict { a: 0, b: 1 });
    }

    #[test]
    fn test_identical_events() {
        let mut det = ConflictDetector::new();
        let mut vc1 = VectorClock::new();
        vc1.increment(1);
        let vc2 = vc1.clone();
        let a = det.register(vc1);
        let b = det.register(vc2);
        assert_eq!(det.check(a, b), ConflictResult::Identical);
    }

    #[test]
    fn test_find_all_conflicts() {
        let mut det = ConflictDetector::new();
        // Three concurrent events
        let mut vc1 = VectorClock::new(); vc1.increment(1);
        let mut vc2 = VectorClock::new(); vc2.increment(2);
        let mut vc3 = VectorClock::new(); vc3.increment(3);
        det.register(vc1);
        det.register(vc2);
        det.register(vc3);
        let conflicts = det.find_all_conflicts();
        assert_eq!(conflicts.len(), 3); // (0,1), (0,2), (1,2)
    }

    #[test]
    fn test_no_conflicts_in_chain() {
        let mut det = ConflictDetector::new();
        let mut vc1 = VectorClock::new(); vc1.increment(1);
        let mut vc2 = vc1.clone(); vc2.increment(2);
        let mut vc3 = vc2.clone(); vc3.increment(1);
        det.register(vc1);
        det.register(vc2);
        det.register(vc3);
        let conflicts = det.find_all_conflicts();
        assert!(conflicts.is_empty());
    }
}
