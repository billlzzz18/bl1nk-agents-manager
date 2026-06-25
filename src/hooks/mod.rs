//! # Lifecycle Hooks
//!
//! Aggregates hook handlers for lifecycle events (e.g. pre/post tool use) and
//! merges their results into a single decision via [`hook_aggregator`].

pub mod hook_aggregator;

pub use hook_aggregator::{AggregatedHookResult, HookAggregator, HookEventName, HookExecutionResult};
