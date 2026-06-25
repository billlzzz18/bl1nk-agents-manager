//! # Agent Orchestration Core
//!
//! The lifecycle of a delegated task, split into four responsibilities:
//!
//! - [`register`] — the in-memory agent registry: state, availability and task tracking.
//! - [`router`]   — picks the best agent for a task using dynamic trust-score weighting.
//! - [`executor`] — runs the task end to end: policy checks, rate limiting and retries.
//! - [`creator`]  — scaffolds new agents following the tiered-permission standard.

pub mod creator;
pub mod executor;
pub mod register;
pub mod router;

pub use executor::AgentExecutor;
pub use register::AgentRegistry;
pub use router::AgentRouter;
