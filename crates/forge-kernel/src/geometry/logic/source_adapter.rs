//! GeometrySource adapter — bridges GeometryStore → forge-math's trait.
//!
//! DOMAIN: The `GeometrySource` trait (forge-math) maps a bare `usize` index
//! to plane coefficients. This adapter wraps a `GeometryStore` and performs
//! the index-based scan required by the BSP/implicit vertex layer.
//!
//! DEPENDENCIES: `forge-math` (GeometrySource, PlaneCoefficients, MathError)

use forge_math::{GeometrySource, MathError, PlaneCoefficients};

use crate::geometry::data::store::GeometryStore;

/// Adapter: wraps a `GeometryStore` and implements `GeometrySource`.
///
/// The BSP layer passes bare `usize` indices (from `PlaneRef`), so we
/// scan all planes to find matches by the handle's index field. This is
/// O(n) per call but only used at the kernel ↔ BSP boundary.
pub struct GeometrySourceAdapter<'a> {
    store: &'a GeometryStore,
}

impl<'a> GeometrySourceAdapter<'a> {
    /// Create an adapter wrapping a geometry store reference.
    pub fn new(store: &'a GeometryStore) -> Self {
        Self { store }
    }
}

impl GeometrySource for GeometrySourceAdapter<'_> {
    fn get_plane(&self, index: usize) -> Result<PlaneCoefficients, MathError> {
        let mut found: Option<PlaneCoefficients> = None;

        for (face, plane) in self.store.planes.iter() {
            if face.index() as usize == index {
                let n = plane.normal();
                let coeff = PlaneCoefficients::try_new(n[0], n[1], n[2], plane.offset())?;
                if found.is_some() {
                    return Err(MathError::InvalidInput(format!(
                        "Ambiguous plane lookup for face index {}",
                        index
                    )));
                }
                found = Some(coeff);
            }
        }

        found.ok_or_else(|| {
            MathError::InvalidInput(format!("No plane found for face index {}", index))
        })
    }
}
