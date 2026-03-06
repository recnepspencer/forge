//! Vertex placement decision validation.
//!
//! DOMAIN: Validates that a `DecisionLog` from a vertex placement operation
//! contains well-formed `NearBoundary` decisions with correct entity scope,
//! threshold, and margin.
//!
//! This is production code, not test infrastructure. The AI agent,
//! self-check pipelines, and test harnesses all call the same validator.

use forge_core::{DecisionKind, DecisionLog, EntityKind, KernelError};

/// Validate vertex placement decisions in a `DecisionLog`.
///
/// For `expected_vertices` final vertices, expects at least `expected_vertices - 1`
/// `NearBoundary` decisions (first vertex has no proximity context; extras
/// arise from BSP candidate vertices merged via coincidence detection).
///
/// Each decision is structurally verified:
/// - `DecisionKind::NearBoundary { threshold }` matching `tolerance`
/// - `EntityRef { kind: Vertex, .. }` entity scope
/// - Margin ≥ 0 (0 = legitimate BSP merge, > tolerance = clean placement)
///
/// Returns `Ok(())` if all decisions are valid, or `Err(KernelError)` with details.
pub fn validate_vertex_decisions(
    log: &DecisionLog,
    expected_vertices: usize,
    tolerance: f64,
) -> Result<(), KernelError> {
    let decisions: Vec<_> = log.decisions().collect();
    let min_decisions = expected_vertices.saturating_sub(1);

    if decisions.len() < min_decisions {
        return Err(KernelError::InvalidInput {
            message: format!(
                "expected at least {min_decisions} NearBoundary decisions for \
                 {expected_vertices} vertices, got {}",
                decisions.len()
            ),
            context: None,
        });
    }

    for (i, d) in decisions.iter().enumerate() {
        match d.get_kind() {
            DecisionKind::NearBoundary { threshold } => {
                if (*threshold - tolerance).abs() >= 1e-15 {
                    return Err(KernelError::InvalidInput {
                        message: format!(
                            "decision {i} threshold={threshold}, expected {tolerance}"
                        ),
                        context: None,
                    });
                }
            }
            other => {
                return Err(KernelError::InvalidInput {
                    message: format!("decision {i} expected NearBoundary, got {other:?}"),
                    context: None,
                });
            }
        }

        match d.get_entity_scope() {
            Some(entity) if entity.kind() == EntityKind::Vertex => {}
            Some(entity) => {
                return Err(KernelError::InvalidInput {
                    message: format!(
                        "decision {i} entity kind is {:?}, expected Vertex",
                        entity.kind()
                    ),
                    context: None,
                });
            }
            None => {
                return Err(KernelError::InvalidInput {
                    message: format!("decision {i} has no entity scope"),
                    context: None,
                });
            }
        }

        if d.get_margin() < 0.0 {
            return Err(KernelError::InvalidInput {
                message: format!("decision {i} has negative margin={:.2e}", d.get_margin()),
                context: None,
            });
        }
    }

    Ok(())
}
