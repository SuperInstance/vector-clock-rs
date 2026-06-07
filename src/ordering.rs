//! Causal ordering: happens-before relation, concurrent detection.

use crate::vector::VectorClock;

/// Causal ordering between two vector clocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CausalOrder {
    /// First happened before second.
    Before,
    /// Second happened before first.
    After,
    /// Events are concurrent (no causal relation).
    Concurrent,
    /// Clocks are equal.
    Equal,
}

/// Determine causal ordering between two vector clocks.
pub fn compare_clocks(a: &VectorClock, b: &VectorClock) -> CausalOrder {
    if a == b {
        return CausalOrder::Equal;
    }
    if a.happened_before(b) {
        return CausalOrder::Before;
    }
    if b.happened_before(a) {
        return CausalOrder::After;
    }
    CausalOrder::Concurrent
}

/// Check if `a` happened before `b`.
pub fn happens_before(a: &VectorClock, b: &VectorClock) -> bool {
    a.happened_before(b)
}

/// Check if two events are concurrent (neither happened before the other).
pub fn are_concurrent(a: &VectorClock, b: &VectorClock) -> bool {
    a.is_concurrent_with(b)
}

/// Sort a list of vector clocks in causal order (topological sort).
/// Returns indices sorted by causal order. Concurrent events maintain their insertion order.
pub fn causal_sort(clocks: &[VectorClock]) -> Vec<usize> {
    let n = clocks.len();
    let mut indices: Vec<usize> = (0..n).collect();

    // Simple bubble sort respecting happens-before
    // Not the most efficient but correct for small sets
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..indices.len().saturating_sub(1) {
            let a_idx = indices[i];
            let b_idx = indices[i + 1];
            if clocks[b_idx].happened_before(&clocks[a_idx]) {
                indices.swap(i, i + 1);
                changed = true;
            }
        }
    }
    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happens_before_relation() {
        let mut a = VectorClock::new();
        a.increment(1);
        let mut b = a.clone();
        b.increment(2);
        assert!(happens_before(&a, &b));
        assert!(!happens_before(&b, &a));
    }

    #[test]
    fn test_concurrent_detection() {
        let mut a = VectorClock::new();
        a.increment(1);
        let mut b = VectorClock::new();
        b.increment(2);
        assert!(are_concurrent(&a, &b));
    }

    #[test]
    fn test_compare_before() {
        let mut a = VectorClock::new();
        a.increment(1);
        let mut b = a.clone();
        b.increment(1);
        assert_eq!(compare_clocks(&a, &b), CausalOrder::Before);
    }

    #[test]
    fn test_compare_equal() {
        let mut a = VectorClock::new();
        a.increment(1);
        let b = a.clone();
        assert_eq!(compare_clocks(&a, &b), CausalOrder::Equal);
    }

    #[test]
    fn test_causal_sort() {
        let mut vc1 = VectorClock::new(); vc1.increment(1);
        let vc3 = vc1.clone();
        let mut vc2 = vc1.clone(); vc2.increment(2);
        // vc1 < vc2, vc3 == vc1, so sorted should put vc1/vc3 before vc2
        let sorted = causal_sort(&[vc2.clone(), vc1.clone(), vc3.clone()]);
        // vc2 should come after vc1
        let pos_vc1 = sorted.iter().position(|&i| i == 1).unwrap();
        let pos_vc2 = sorted.iter().position(|&i| i == 0).unwrap();
        assert!(pos_vc1 < pos_vc2);
    }
}
