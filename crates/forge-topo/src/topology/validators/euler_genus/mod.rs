//! Euler formula and genus invariant validators.
//!
//! DOMAIN: Classic and generalized Euler formula verification,
//! genus computation consistency, and per-component Euler checks.
//!
//! VALIDATORS (from validators.md §7):
//! - ValidateEulerClassic
//! - ValidateEulerGeneralizedWithRegions
//! - ValidateGenusComputationConsistency
//! - ValidatePerComponentEuler
//!
//! DEPENDENCIES: `arena`, `handles`, `queries::shell`

use crate::b_rep::TopologyArena;
use crate::handles::FaceId;
use crate::topology::bitset::EntityBitset;
use forge_core::KernelError;
use std::collections::VecDeque;

/// Validate the generalized Euler formula for each connected shell.
///
/// Supports genus > 0 topology (tori, solids with through-holes) and
/// faces with inner loops (holes). Uses the full formula:
///   V - E + F = 2 - 2G + R
/// where G = genus, R = total inner loop count across all faces in the shell.
///
/// Validates that genus is non-negative — a negative genus indicates
/// a structurally broken shell.
pub(crate) fn validate_euler(arena: &TopologyArena) -> Result<(), KernelError> {
    let f_total = arena.face_count();
    if f_total == 0 && arena.vertex_count() == 0 {
        return Ok(());
    }

    let all_faces: Vec<FaceId> = arena.iter_faces().map(|(fid, _)| fid).collect();
    let face_by_index: std::collections::BTreeMap<u32, FaceId> =
        all_faces.iter().map(|fid| (fid.index(), *fid)).collect();
    let mut visited_faces = EntityBitset::for_faces(arena);
    let mut shell_index: usize = 0;

    for &seed_face in &all_faces {
        if !visited_faces.contains(seed_face.index())? {
            let mut shell_faces = EntityBitset::for_faces(arena);
            let mut shell_vertices = EntityBitset::for_vertices(arena);
            let mut shell_edges = EntityBitset::for_edges(arena);
            let mut queue: VecDeque<FaceId> = VecDeque::new();

            queue.push_back(seed_face);
            shell_faces.insert(seed_face.index())?;

            while let Some(face_id) = queue.pop_front() {
                let (neighbors, edge_keys, vertex_indices) =
                    super::shell_closure::collect_shell_data_for_face(arena, face_id)?;

                for vid in vertex_indices {
                    shell_vertices.insert(vid)?;
                }

                for ek in edge_keys {
                    shell_edges.insert(ek)?;
                }

                for neighbor in neighbors {
                    if shell_faces.insert(neighbor.index())? {
                        queue.push_back(neighbor);
                    }
                }
            }

            for idx in shell_faces.iter_ones() {
                visited_faces.insert(idx)?;
            }

            let sv = shell_vertices.count() as i64;
            let se = shell_edges.count() as i64;
            let sf = shell_faces.count() as i64;
            let euler_char = sv - se + sf;

            let rings: usize = shell_faces
                .iter_ones()
                .filter_map(|idx| {
                    let fid = face_by_index.get(&idx)?;
                    arena.get_face(*fid).ok()
                })
                .map(|face_data| face_data.inner_loop_count())
                .sum();

            let shell_id = face_by_index
                .get(&shell_faces.iter_ones().next().unwrap())
                .unwrap();
            let shell_kind = arena.get_face(*shell_id).unwrap().shell();
            if !matches!(
                arena.get_shell(shell_kind).unwrap().kind(),
                crate::b_rep::ShellKind::Solid(_)
            ) {
                shell_index += 1;
                continue;
            }

            let genus = compute_shell_genus(euler_char, rings, shell_index)?;
            let expected = 2_i64 - 2 * (genus as i64) + (rings as i64);

            if euler_char != expected {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::GeneralizedEulerViolation {
                        shell_index: shell_index as u32,
                        vertices: sv as usize,
                        edges: se as usize,
                        faces: sf as usize,
                        genus,
                        rings,
                        expected_chi: expected,
                        actual_chi: euler_char,
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "Shell".to_string(),
                            index: shell_index as u32,
                        },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Shell {} generalized Euler: V-E+F = {}-{}+{} = {}, genus={}, rings={}, expected χ={}",
                            shell_index, sv, se, sf, euler_char, genus, rings, expected
                        ),
                    }),
                });
            }
            shell_index += 1;
        }
    }

    Ok(())
}

/// Compute the genus of a shell from its Euler characteristic and ring count.
///
/// Full formula: V - E + F = 2 - 2G + R, so G = (2 - χ + R) / 2.
/// Returns 0 for genus-0 (sphere-like), 1 for torus, etc.
/// Returns `Err` if genus is non-integer or negative — this indicates
/// structural damage in the shell rather than valid higher-genus topology.
pub(crate) fn compute_shell_genus(
    euler_char: i64,
    rings: usize,
    shell_index: usize,
) -> Result<usize, KernelError> {
    let twice_genus = 2 - euler_char + rings as i64;

    if twice_genus < 0 {
        return Err(KernelError::TopologyViolation {
            err: forge_core::TopologyError::GeneralizedEulerViolation {
                shell_index: shell_index as u32,
                vertices: 0,
                edges: 0,
                faces: 0,
                genus: 0,
                rings,
                expected_chi: 0,
                actual_chi: euler_char,
            },
            context: Some(forge_core::ErrorContext {
                scope: forge_core::ErrorScope::Entity {
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
            err: forge_core::TopologyError::NonOrientableSurface {
                shell_index: shell_index as u32,
            },
            context: Some(forge_core::ErrorContext {
                scope: forge_core::ErrorScope::Entity {
                    entity_kind: "Shell".to_string(),
                    index: shell_index as u32,
                },
                suggested_fixes: Vec::new(),
                detail: format!(
                    "Shell {} has an odd Euler characteristic implying it is a non-orientable surface (like a Möbius strip or Klein bottle).",
                    shell_index
                ),
            }),
        });
    }

    Ok((twice_genus / 2) as usize)
}
