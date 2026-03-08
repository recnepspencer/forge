use forge_core::{ErrorContext, ErrorScope, KernelError, TopologyError};

pub(crate) fn compute_projected_shell_genus(
    euler_char: i64,
    rings: usize,
    shell_index: usize,
) -> Result<usize, KernelError> {
    let twice_genus = 2 - euler_char + rings as i64;

    if twice_genus < 0 {
        return Err(KernelError::TopologyViolation {
            err: TopologyError::GeneralizedEulerViolation {
                shell_index: shell_index as u32,
                vertices: 0,
                edges: 0,
                faces: 0,
                genus: 0,
                rings,
                expected_chi: 0,
                actual_chi: euler_char,
            },
            context: Some(ErrorContext {
                scope: ErrorScope::Entity {
                    entity_kind: "Shell".to_string(),
                    index: shell_index as u32,
                },
                suggested_fixes: Vec::new(),
                detail: format!(
                    "Shell {} has invalid genus: 2·G = {} (negative indicates structural damage)",
                    shell_index, twice_genus
                ),
            }),
        });
    }

    if twice_genus % 2 != 0 {
        return Err(KernelError::TopologyViolation {
            err: TopologyError::NonOrientableSurface {
                shell_index: shell_index as u32,
            },
            context: Some(ErrorContext {
                scope: ErrorScope::Entity {
                    entity_kind: "Shell".to_string(),
                    index: shell_index as u32,
                },
                suggested_fixes: Vec::new(),
                detail: format!(
                    "Shell {} has an odd Euler characteristic implying a non-orientable surface",
                    shell_index
                ),
            }),
        });
    }

    Ok((twice_genus / 2) as usize)
}
