//! Command dispatch: bridges `forge-schema::Command` to `FeatureTree`.
//!
//! DOMAIN: Resolves declarative commands into concrete features
//! and registers them in the signal graph.
//!
//! INVARIANTS:
//! - Exhaustive match on `Command` — adding a variant forces a handler
//! - Entity references are resolved before feature construction
//! - All errors are typed `KernelError`, never panics

use forge_core::KernelError;
use forge_schema::{Command, EntityRef};
use forge_signal::handles::NodeId;

use crate::engine::tree::{FeatureTree, NativeFeature};
use crate::engine::wrappers::{BooleanFeature, MakeCubeFeature};
use crate::operations::boolean::BooleanOp;

/// Bridges `forge-schema::Command` variants to `FeatureTree` registration.
///
/// Each `Command` variant maps to a `NativeFeature` construction +
/// `FeatureTree::register_feature`. Entity references are resolved
/// against the tree's name/index registry before feature construction.
pub struct CommandDispatcher<'a> {
    tree: &'a mut FeatureTree,
    /// Insertion-order tracking for `EntityRef::ByIndex` resolution.
    insertion_order: Vec<NodeId>,
}

impl<'a> CommandDispatcher<'a> {
    /// Create a dispatcher targeting the given feature tree.
    pub fn new(tree: &'a mut FeatureTree) -> Self {
        Self {
            tree,
            insertion_order: Vec::new(),
        }
    }

    /// Execute a schema command: resolve targets, construct the feature,
    /// insert into tree, and return the new node's ID.
    pub fn dispatch(&mut self, cmd: &Command) -> Result<NodeId, KernelError> {
        let node_id = match cmd {
            Command::AddBlock { origin, dimensions } => {
                let center = [
                    origin[0] + dimensions[0] / 2.0,
                    origin[1] + dimensions[1] / 2.0,
                    origin[2] + dimensions[2] / 2.0,
                ];
                let size = dimensions[0];
                let feature = NativeFeature::MakeCube(
                    MakeCubeFeature::new("block", center, size),
                );
                self.tree.register_feature(feature)?
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
                let feature = NativeFeature::Boolean(
                    BooleanFeature::new("boolean_union", BooleanOp::Union, target_id, tool_id),
                );
                self.tree.register_feature(feature)?
            }
            Command::BooleanSubtract { target, tool } => {
                let target_id = self.resolve_entity_ref(target)?;
                let tool_id = self.resolve_entity_ref(tool)?;
                let feature = NativeFeature::Boolean(
                    BooleanFeature::new("boolean_subtract", BooleanOp::Subtraction, target_id, tool_id),
                );
                self.tree.register_feature(feature)?
            }
            Command::SetAttribute { .. } => {
                return Err(KernelError::InvalidInput {
                    message: "SetAttribute is not yet implemented".into(),
                    context: None,
                });
            }
        };

        self.insertion_order.push(node_id);
        Ok(node_id)
    }

    /// Resolve an `EntityRef` to a `NodeId` in the feature tree.
    fn resolve_entity_ref(&self, entity: &EntityRef) -> Result<NodeId, KernelError> {
        match entity {
            EntityRef::ByFeature { feature_name, .. } => {
                self.tree.get_node_by_name(feature_name).ok_or_else(|| {
                    KernelError::InvalidInput {
                        message: format!("Feature '{}' not found", feature_name),
                        context: None,
                    }
                })
            }
            EntityRef::ByIndex { index } => {
                self.insertion_order.get(*index).copied().ok_or_else(|| {
                    KernelError::InvalidInput {
                        message: format!(
                            "Index {} out of range (dispatched {} commands so far)",
                            index,
                            self.insertion_order.len()
                        ),
                        context: None,
                    }
                })
            }
        }
    }
}
