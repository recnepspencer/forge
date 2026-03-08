use std::collections::BTreeSet;

use forge_core::{ErrorContext, ErrorScope, KernelError, TopologyError};
use forge_spec::facade::SpecShellKind;

use crate::projection::data::{ProjectedShellId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::genus::compute_projected_shell_genus;

pub fn validate_projected_per_component_euler(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for shell_index in 0..topology.shell_count() {
        let shell = ProjectedShellId::new(shell_index as u32);
        if !matches!(topology.shell(shell).kind, SpecShellKind::Solid(_)) {
            continue;
        }

        let mut vertices = BTreeSet::new();
        let mut edges = BTreeSet::new();
        let faces = topology.shell_faces(shell);
        let mut rings = 0_usize;

        for face in &faces {
            let face_data = topology.face(*face);
            rings += face_data.inner_loops.len();

            for half_edge in topology.face_half_edges(*face).map_err(|error| {
                KernelError::InvalidInput {
                    message: format!("Projected Euler traversal failed: {}", error),
                    context: None,
                }
            })? {
                let half_edge_data = topology.half_edge(half_edge);
                vertices.insert(half_edge_data.origin.raw());
                edges.insert(half_edge_data.edge.raw());
            }
        }

        let v = vertices.len() as i64;
        let e = edges.len() as i64;
        let f = faces.len() as i64;
        let euler_char = v - e + f;
        let genus = compute_projected_shell_genus(euler_char, rings, shell_index)?;
        let expected = 2_i64 - 2 * genus as i64 + rings as i64;

        if euler_char != expected {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::GeneralizedEulerViolation {
                    shell_index: shell_index as u32,
                    vertices: v as usize,
                    edges: e as usize,
                    faces: f as usize,
                    genus,
                    rings,
                    expected_chi: expected,
                    actual_chi: euler_char,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity {
                        entity_kind: "Shell".to_string(),
                        index: shell_index as u32,
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Shell {} generalized Euler: V-E+F = {}-{}+{} = {}, genus={}, rings={}, expected χ={}",
                        shell_index, v, e, f, euler_char, genus, rings, expected
                    ),
                }),
            });
        }
    }

    Ok(())
}
