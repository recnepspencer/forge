//! ConvexCell validation.
//!
//! DOMAIN: Structural validation of BSP ConvexCell output
//! before attempting topology construction.

use forge_core::KernelError;
use worth_geom::ConvexCell;

/// Validate that a ConvexCell has enough structure for a valid polyhedron.
///
/// Requires at least 4 faces and 4 vertices (the minimum for a tetrahedron).
pub fn validate_cell(cell: &ConvexCell) -> Result<(), KernelError> {
    if cell.face_count() < 4 {
        return Err(KernelError::InvalidInput {
            message: format!(
                "ConvexCell needs at least 4 faces for a polyhedron, got {}",
                cell.face_count()
            ),
            context: None,
        });
    }
    if cell.vertex_count() < 4 {
        return Err(KernelError::InvalidInput {
            message: format!(
                "ConvexCell needs at least 4 vertices for a polyhedron, got {}",
                cell.vertex_count()
            ),
            context: None,
        });
    }
    Ok(())
}
