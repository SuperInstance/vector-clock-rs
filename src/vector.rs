//! Vector clock implementation.

use std::collections::HashMap;

pub type ProcessId = u64;

/// A vector clock tracking causal history across processes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorClock {
    clock: HashMap<ProcessId, u64>,
}

impl VectorClock {
    pub fn new() -> Self {
        Self { clock: HashMap::new() }
    }

    /// Create from a map.
    pub fn from_map(clock: HashMap<ProcessId, u64>) -> Self {
        Self { clock }
    }

    /// Increment clock for a local event on process `pid`.
    pub fn increment(&mut self, pid: ProcessId) -> u64 {
        let val = self.clock.entry(pid).or_insert(0);
        *val += 1;
        *val
    }

    /// Get the clock value for a process.
    pub fn get(&self, pid: ProcessId) -> u64 {
        *self.clock.get(&pid).unwrap_or(&0)
    }

    /// Merge with another clock (take component-wise max).
    pub fn merge(&mut self, other: &VectorClock) {
        for (&pid, &ts) in &other.clock {
            let entry = self.clock.entry(pid).or_insert(0);
            *entry = std::cmp::max(*entry, ts);
        }
    }

    /// Create a merged copy without modifying self.
    pub fn merged(&self, other: &VectorClock) -> VectorClock {
        let mut result = self.clone();
        result.merge(other);
        result
    }

    /// Check if self happened-before other (strict: all components <=, at least one <).
    pub fn happened_before(&self, other: &VectorClock) -> bool {
        let all_leq = self.clock.keys().chain(other.clock.keys()).all(|&pid| {
            self.get(pid) <= other.get(pid)
        });
        let at_least_one_lt = self.clock.keys().chain(other.clock.keys()).any(|&pid| {
            self.get(pid) < other.get(pid)
        });
        all_leq && at_least_one_lt
    }

    /// Check if concurrent (neither happened before the other).
    pub fn is_concurrent_with(&self, other: &VectorClock) -> bool {
        !self.happened_before(other) && !other.happened_before(self) && self != other
    }

    /// Get all process IDs in this clock.
    pub fn processes(&self) -> Vec<ProcessId> {
        let mut pids: Vec<_> = self.clock.keys().copied().collect();
        pids.sort();
        pids
    }

    /// Get the inner map reference.
    pub fn as_map(&self) -> &HashMap<ProcessId, u64> {
        &self.clock
    }
}

impl Default for VectorClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_clock_empty() {
        let vc = VectorClock::new();
        assert_eq!(vc.get(1), 0);
    }

    #[test]
    fn test_increment() {
        let mut vc = VectorClock::new();
        vc.increment(1);
        vc.increment(1);
        assert_eq!(vc.get(1), 2);
    }

    #[test]
    fn test_multiple_processes() {
        let mut vc = VectorClock::new();
        vc.increment(1);
        vc.increment(2);
        vc.increment(1);
        assert_eq!(vc.get(1), 2);
        assert_eq!(vc.get(2), 1);
    }

    #[test]
    fn test_happened_before() {
        let mut vc1 = VectorClock::new();
        vc1.increment(1);
        // vc2 is vc1 + increment on process 2
        let mut vc2 = vc1.clone();
        vc2.increment(2);
        assert!(vc1.happened_before(&vc2));
        assert!(!vc2.happened_before(&vc1));
    }

    #[test]
    fn test_concurrent() {
        let mut vc1 = VectorClock::new();
        vc1.increment(1);
        let mut vc2 = VectorClock::new();
        vc2.increment(2);
        assert!(vc1.is_concurrent_with(&vc2));
    }

    #[test]
    fn test_merge() {
        let mut vc1 = VectorClock::new();
        vc1.increment(1);
        let mut vc2 = VectorClock::new();
        vc2.increment(2);
        let merged = vc1.merged(&vc2);
        assert_eq!(merged.get(1), 1);
        assert_eq!(merged.get(2), 1);
    }

    #[test]
    fn test_equal_clocks() {
        let mut vc1 = VectorClock::new();
        vc1.increment(1);
        let mut vc2 = VectorClock::new();
        vc2.increment(1);
        assert_eq!(vc1, vc2);
        assert!(!vc1.happened_before(&vc2));
        assert!(!vc1.is_concurrent_with(&vc2));
    }
}
