//! # vector-clock-rs
//!
//! Vector clocks and causality tracking: Lamport timestamps, vector clocks,
//! conflict detection, and causal ordering.
//!
//! ## Modules
//! - `lamport` — Lamport logical timestamps
//! - `vector` — Vector clock implementation
//! - `conflict` — Conflict detection between concurrent events
//! - `ordering` — Causal ordering relations (happens-before)
//! - `merge` — Clock merging and synchronization

pub mod lamport;
pub mod vector;
pub mod conflict;
pub mod ordering;
pub mod merge;

pub use lamport::LamportClock;
pub use vector::VectorClock;
pub use conflict::{ConflictDetector, ConflictResult};
pub use ordering::{CausalOrder, happens_before, are_concurrent};
pub use merge::{merge_clocks, merge_with_dominance};
