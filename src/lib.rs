//! # bl1nk-agents-manager
//!
//! An intelligent MCP orchestrator that ships as the server backend of a
//! **Claude Code plugin**. A Claude Code plugin is a bundle of `commands/`,
//! `agents/` (subagents), `skills/`, `hooks/` and one or more MCP servers; the
//! crate in this `src/` tree *is* that MCP server. When Claude Code (or any MCP
//! host) launches the plugin, it speaks to the binary built from
//! [`mcp`] over stdio and gains the tools defined here.
//!
//! ## Architecture at a glance
//!
//! The modules are organised into three layers. Code flows downward: the
//! surface receives a request, orchestration decides *who* runs it and *whether
//! it is allowed*, and the platform provides the supporting services.
//!
//! ```text
//!   Claude Code host  ─▶  Plugin Surface  ─▶  Orchestration  ─▶  Platform
//! ```
//!
//! ### 1. Plugin Surface — what the host can call
//! - [`mcp`]   — the MCP server, its typed tools and the JSON-RPC protocol glue.
//! - [`tools`] — concrete tool implementations exposed to agents and the host.
//!
//! ### 2. Orchestration — the decision-making core
//! - [`agents`] — registry, router, executor and creator: the agent lifecycle.
//! - [`hooks`]  — lifecycle hook aggregation (pre/post tool, etc.).
//!
//! ### 3. Platform — supporting services
//! - [`registry`]    — unified agent registry, smart search and policy evaluation.
//! - [`context`]     — conversation/workspace context and token-aware compaction.
//! - [`persistence`] — atomic JSON storage for durable state.
//! - [`rate_limit`]  — per-agent request budgeting.
//! - [`system`]      — filesystem discovery of agents and skills.
//! - [`config`]      — configuration loading and schema-backed validation.
//!
//! The [`layers`] module re-exports the above grouped by layer, so the
//! architecture can be navigated in code (`layers::orchestration::agents`) as
//! well as flat (`agents`). Both paths refer to the same module.

// ── Plugin Surface ─────────────────────────────────────────────────────────
pub mod mcp;
pub mod tools;

// ── Orchestration ──────────────────────────────────────────────────────────
pub mod agents;
pub mod hooks;

// ── Platform ───────────────────────────────────────────────────────────────
pub mod config;
pub mod context;
pub mod persistence;
pub mod rate_limit;
pub mod registry;
pub mod system;

// NOTE: `permissions/` contains a work-in-progress shell-aware policy engine.
// It is intentionally not wired into the build yet — its `permission_manager`
// still imports an unimplemented `shell_ast` helper — so it is excluded here to
// keep the compiled surface honest. Re-enable it once that module lands.

/// Architectural grouping of the crate's modules by layer.
///
/// These are re-exports, not new modules — `layers::surface::mcp` and the
/// top-level `mcp` are the same thing. The grouping exists purely to make the
/// layering described in the crate docs navigable from code.
pub mod layers {
    /// Plugin Surface: the API the Claude Code host talks to.
    pub mod surface {
        pub use crate::mcp;
        pub use crate::tools;
    }

    /// Orchestration: agent selection, execution and lifecycle hooks.
    pub mod orchestration {
        pub use crate::agents;
        pub use crate::hooks;
    }

    /// Platform: registry, context, persistence and other shared services.
    pub mod platform {
        pub use crate::config;
        pub use crate::context;
        pub use crate::persistence;
        pub use crate::rate_limit;
        pub use crate::registry;
        pub use crate::system;
    }
}
