//! # Toolset
//!
//! Standard tools exposed to agents and the Claude Code host. Each tool
//! implements the [`AgentTool`] trait — a name, a description and a JSON
//! `execute` entry point — so it can be surfaced uniformly over MCP.

use anyhow::Result;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod ask_user_question;
pub mod bash;
pub mod exit_plan_mode;
pub mod file_ops;

pub use ask_user_question::AskUserQuestionInput;
pub use exit_plan_mode::ExitPlanModeHandler;
pub use file_ops::FileOpsTools;

/// Standard interface every agent tool implements.
#[async_trait]
pub trait AgentTool: Send + Sync {
    /// Tool name (e.g. "read_file" or "write_file").
    fn name(&self) -> &str;

    /// Human-readable description of what the tool does.
    fn description(&self) -> &str;

    /// Run the tool with a JSON input and return a JSON output.
    async fn execute(&self, input: serde_json::Value) -> Result<serde_json::Value>;
}

/// Tool summary advertised to the model.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}
