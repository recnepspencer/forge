//! Consolidated PostToolUse routing and equivalence contracts.

mod post_tool_use_fixture;

#[path = "post_tool_use_contracts/equivalence.rs"]
mod equivalence;
#[path = "post_tool_use_contracts/routing.rs"]
mod routing;
