//! Clock merging and synchronization.

use crate::vector::VectorClock;

/// Merge two vector clocks by taking component-wise maximum.
pub fn merge_clocks(a: &VectorClock, b: &VectorClock) -> VectorClock {
    a.merged(b)
}

/// Merge clocks where one dominates: if `dominant` has a higher value for every component,
/// return dominant. Otherwise merge.
pub fn merge_with_dominance(a: &VectorClock, b: &VectorClock) -> VectorClock {
    let all_a_ge = a.processes().iter().all(|&pid| a.get(pid) >= b.get(pid));
    let all_b_ge = b.processes().iter().all(|&pid| b.get(pid) >= a.get(pid));
    if all_a_ge {
        a.clone()
    } else if all_b_ge {
        b.clone()
    } else {
        merge_clocks(a, b)
    }
}

/// Merge a list of clocks into one.
pub fn merge_all(clocks: &[VectorClock]) -> VectorClock {
    let mut result = VectorClock::new();
    for clock in clocks {
        result.merge(clock);
    }
    result
}

/// Compute the "difference" between two clocks (components where a > b).
pub fn clock_difference(a: &VectorClock, b: &VectorClock) -> VectorClock {
    let mut diff = VectorClock::new();
    for &pid in &a.processes() {
        let va = a.get(pid);
        let vb = b.get(pid);
        if va > vb {
            for _ in 0..(va - vb) {
                diff.increment(pid);
            }
        }
    }
    diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_clocks_basic() {
        let mut a = VectorClock::new();
        a.increment(1);
        let mut b = VectorClock::new();
        b.increment(2);
        let merged = merge_clocks(&a, &b);
        assert_eq!(merged.get(1), 1);
        assert_eq!(merged.get(2), 1);
    }

    #[test]
    fn test_merge_takes_max() {
        let mut a = VectorClock::new();
        a.increment(1);
        a.increment(1); // pid 1 = 2
        let mut b = VectorClock::new();
        b.increment(1); // pid 1 = 1
        let merged = merge_clocks(&a, &b);
        assert_eq!(merged.get(1), 2);
    }

    #[test]
    fn test_merge_with_dominance_a_dominates() {
        let mut a = VectorClock::new();
        a.increment(1);
        a.increment(2);
        let mut b = VectorClock::new();
        b.increment(1);
        let result = merge_with_dominance(&a, &b);
        assert_eq!(result, a);
    }

    #[test]
    fn test_merge_all() {
        let mut a = VectorClock::new(); a.increment(1);
        let mut b = VectorClock::new(); b.increment(2);
        let mut c = VectorClock::new(); c.increment(1); c.increment(1); // pid 1 = 2
        let merged = merge_all(&[a, b, c]);
        assert_eq!(merged.get(1), 2);
        assert_eq!(merged.get(2), 1);
    }

    #[test]
    fn test_clock_difference() {
        let mut a = VectorClock::new();
        a.increment(1);
        a.increment(1);
        a.increment(1); // pid 1 = 3
        let mut b = VectorClock::new();
        b.increment(1); // pid 1 = 1
        let diff = clock_difference(&a, &b);
        assert_eq!(diff.get(1), 2); // 3 - 1
    }
}
