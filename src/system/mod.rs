//! # System Discovery
//!
//! Scans the filesystem for plugin assets and validates them before they enter
//! the registry.
//!
//! - [`discovery`]       — environment/resource scan producing a `DiscoveryReport`.
//! - [`skill_discovery`] — async, schema-validated discovery of agents and skills,
//!   including frontmatter parsing, `$ARGUMENTS` substitution and skill-path injection.

pub mod discovery;
pub mod skill_discovery;
