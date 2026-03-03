//! Command dispatch: bridges `forge-schema::Command` to `FeatureTree`.
//!
//! DOMAIN: Resolves declarative commands into concrete features
//! and registers them in the signal graph. Thin routing only —
//! construction logic lives in per-command handler files.
//!
//! INVARIANTS:
//! - Exhaustive match on `Command` — adding a variant forces a handler
//! - Entity references are resolved before feature construction
//! - All errors are typed `KernelError`, never panics

use forge_core::KernelError;
use forge_schema::{Command, EntityRef};
use forge_signal::facade::NodeId;

use crate::engine::facade::FeatureTree;
use crate::operations::boolean::BooleanOp;
use super::handlers;
use super::native_feature::NativeFeature;

/// Bridges `forge-schema::Command` variants to `FeatureTree` registration.
///
/// Each `Command` variant is routed to a handler function that constructs
/// a `NativeFeature`. Entity references are resolved here before calling
/// handlers. Registration and insertion-order tracking stay in this struct.
pub struct CommandDispatcher<'a> {
    tree: &'a mut FeatureTree<NativeFeature>,
    /// Insertion-order tracking for `EntityRef::ByIndex` resolution.
    insertion_order: Vec<NodeId>,
}

impl<'a> CommandDispatcher<'a> {
    /// Create a dispatcher targeting the given feature tree.
    pub fn new(tree: &'a mut FeatureTree<NativeFeature>) -> Self {
        Self {
            tree,
            insertion_order: Vec::new(),
        }
    }

    /// Execute a schema command: resolve targets, construct the feature,
    /// insert into tree, and return the new node's ID.
    pub fn dispatch(&mut self, cmd: &Command) -> Result<NodeId, KernelError> {
        let seq = self.tree.next_seq();
        let feature = match cmd {
            Command::AddBlock { origin, dimensions } => {
                let name = format!("block_{}", seq);
                handlers::add_block::add_block(&name, *origin, *dimensions)
            }
            Command::AddHole { .. } => {
                return Err(KernelError::InvalidInput {
                    message: "AddHole is not yet implemented".into(),
                    context: None,
                });
            }
            Command::AddFillet { .. } => {
                return Err(KernelError::InvalidInput {
                    message: "AddFillet is not yet implemented".into(),
                    context: None,
                });
            }
            Command::BooleanUnion { target, tool } => {
                let target_id = self.resolve_entity_ref(target)?;
                let tool_id = self.resolve_entity_ref(tool)?;
                let name = format!("boolean_union_{}", seq);
                handlers::boolean::boolean(&name, BooleanOp::Union, target_id, tool_id)
            }
            Command::BooleanSubtract { target, tool } => {
                let target_id = self.resolve_entity_ref(target)?;
                let tool_id = self.resolve_entity_ref(tool)?;
                let name = format!("boolean_subtract_{}", seq);
                handlers::boolean::boolean(&name, BooleanOp::Subtraction, target_id, tool_id)
            }
            Command::SetAttribute { .. } => {
                return Err(KernelError::InvalidInput {
                    message: "SetAttribute is not yet implemented".into(),
                    context: None,
                });
            }
        };

        let node_id = self.tree.register_feature(feature)?;
        self.insertion_order.push(node_id);
        Ok(node_id)
    }

    /// Resolve an `EntityRef` to a `NodeId` in the feature tree.
    fn resolve_entity_ref(&self, entity: &EntityRef) -> Result<NodeId, KernelError> {
        match entity {
            EntityRef::ByFeature { feature_name, .. } => self
                .tree
                .get_node_by_name(feature_name)
                .ok_or_else(|| KernelError::InvalidInput {
                    message: format!("Feature '{}' not found", feature_name),
                    context: None,
                }),
            EntityRef::ByIndex { index } => {
                self.insertion_order
                    .get(*index)
                    .copied()
                    .ok_or_else(|| KernelError::InvalidInput {
                        message: format!(
                            "Index {} out of range (dispatched {} commands so far)",
                            index,
                            self.insertion_order.len()
                        ),
                        context: None,
                    })
            }
        }
    }
}
