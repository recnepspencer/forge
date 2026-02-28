//! Random solid generators for corpus fuzzing.
//!
//! DOMAIN: Test infrastructure — deterministic random polyhedra generation.
//! INVARIANTS: All generators are seeded and deterministic (D1).
//! DEPENDENCIES: `forge-geom` (BSP), `forge-kernel` (mesh builder, boolean schema)

use forge_core::KernelError;
use forge_geom::spatial::bsp::{build_convex_polyhedron, BspConfig};
use forge_geom::Plane;
use forge_kernel::brep::state::BrepState;
use forge_kernel::core::config::resolve::{resolve_config, ResolvedConfig};
use forge_kernel::core::config::schema::KernelConfig;
use forge_kernel::geometry_state::GeometryState;
use forge_kernel::mesh_builder::build_halfedge_mesh;
use forge_kernel::operations::boolean::{BooleanInput, BooleanOp};
use forge_topo::transactions::TopologyState;

/// Build a default `ResolvedConfig` for test generators.
fn test_config() -> ResolvedConfig {
    resolve_config(&KernelConfig::default(), None, None, None).unwrap()
}

/// Deterministic PRNG (xorshift64). No external dependencies.
pub struct Xorshift64 {
    state: u64,
}

impl Xorshift64 {
    /// Create a new PRNG with the given seed.
    pub fn new(seed: u64) -> Self {
        let safe_seed = if seed == 0 { 1 } else { seed };
        Self { state: safe_seed }
    }

    /// Generate the next pseudo-random u64.
    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    /// Generate a random f64 in [lo, hi).
    pub fn next_f64_range(&mut self, lo: f64, hi: f64) -> f64 {
        let t = (self.next_u64() as f64) / (u64::MAX as f64);
        lo + t * (hi - lo)
    }

    /// Generate a random usize in [lo, hi].
    pub fn next_usize_range(&mut self, lo: usize, hi: usize) -> usize {
        lo + (self.next_u64() as usize) % (hi - lo + 1)
    }
}

/// Build a random convex solid from N random planes.
///
/// Generates 4–10 planes with random normals and offsets,
/// then constructs via BSP + halfedge mesh builder.
pub fn random_convex_solid(seed: u64) -> Result<(TopologyState, GeometryState), KernelError> {
    let mut rng = Xorshift64::new(seed);
    let num_planes = rng.next_usize_range(4, 10);

    let mut planes = Vec::with_capacity(num_planes);
    for _ in 0..num_planes {
        let nx = rng.next_f64_range(-1.0, 1.0);
        let ny = rng.next_f64_range(-1.0, 1.0);
        let nz = rng.next_f64_range(-1.0, 1.0);
        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        if len < 1e-10 {
            continue;
        }
        let normal = [nx / len, ny / len, nz / len];
        let dist = rng.next_f64_range(0.5, 3.0);
        let point = [normal[0] * dist, normal[1] * dist, normal[2] * dist];

        if let Ok(plane) = Plane::from_point_normal(point, normal) {
            planes.push(plane);
        }
    }

    if planes.len() < 4 {
        return Err(KernelError::InvalidInput {
            message: "Too few valid planes generated".to_string(),
            context: None,
        });
    }

    let cell = build_convex_polyhedron(&planes, &BspConfig::default())?;
    let cfg = test_config();
    let result = build_halfedge_mesh(&cell, &cfg)?;
    let (topo, geom, _brep) = result.into_parts();
    Ok((topo, geom))
}

/// Build a cube at a specific position with a given half-size.
pub fn build_cube_at(
    center: [f64; 3],
    half: f64,
) -> Result<(TopologyState, GeometryState), KernelError> {
    let planes = vec![
        Plane::from_point_normal([center[0] + half, center[1], center[2]], [1.0, 0.0, 0.0])
            .map_err(|e| KernelError::InternalError {
                message: format!("{e}"),
                context: None,
            })?,
        Plane::from_point_normal([center[0] - half, center[1], center[2]], [-1.0, 0.0, 0.0])
            .map_err(|e| KernelError::InternalError {
                message: format!("{e}"),
                context: None,
            })?,
        Plane::from_point_normal([center[0], center[1] + half, center[2]], [0.0, 1.0, 0.0])
            .map_err(|e| KernelError::InternalError {
                message: format!("{e}"),
                context: None,
            })?,
        Plane::from_point_normal([center[0], center[1] - half, center[2]], [0.0, -1.0, 0.0])
            .map_err(|e| KernelError::InternalError {
                message: format!("{e}"),
                context: None,
            })?,
        Plane::from_point_normal([center[0], center[1], center[2] + half], [0.0, 0.0, 1.0])
            .map_err(|e| KernelError::InternalError {
                message: format!("{e}"),
                context: None,
            })?,
        Plane::from_point_normal([center[0], center[1], center[2] - half], [0.0, 0.0, -1.0])
            .map_err(|e| KernelError::InternalError {
                message: format!("{e}"),
                context: None,
            })?,
    ];

    let cell = build_convex_polyhedron(&planes, &BspConfig::default())?;
    let cfg = test_config();
    let result = build_halfedge_mesh(&cell, &cfg)?;
    let (topo, geom, _brep) = result.into_parts();
    Ok((topo, geom))
}

/// Build a cube at a random position with random half-size.
pub fn random_cube(rng: &mut Xorshift64) -> Result<(TopologyState, GeometryState), KernelError> {
    let cx = rng.next_f64_range(-5.0, 5.0);
    let cy = rng.next_f64_range(-5.0, 5.0);
    let cz = rng.next_f64_range(-5.0, 5.0);
    let half = rng.next_f64_range(0.5, 4.0);

    let planes = vec![
        Plane::from_point_normal([cx + half, cy, cz], [1.0, 0.0, 0.0]).map_err(|e| {
            KernelError::InternalError {
                message: format!("{e}"),
                context: None,
            }
        })?,
        Plane::from_point_normal([cx - half, cy, cz], [-1.0, 0.0, 0.0]).map_err(|e| {
            KernelError::InternalError {
                message: format!("{e}"),
                context: None,
            }
        })?,
        Plane::from_point_normal([cx, cy + half, cz], [0.0, 1.0, 0.0]).map_err(|e| {
            KernelError::InternalError {
                message: format!("{e}"),
                context: None,
            }
        })?,
        Plane::from_point_normal([cx, cy - half, cz], [0.0, -1.0, 0.0]).map_err(|e| {
            KernelError::InternalError {
                message: format!("{e}"),
                context: None,
            }
        })?,
        Plane::from_point_normal([cx, cy, cz + half], [0.0, 0.0, 1.0]).map_err(|e| {
            KernelError::InternalError {
                message: format!("{e}"),
                context: None,
            }
        })?,
        Plane::from_point_normal([cx, cy, cz - half], [0.0, 0.0, -1.0]).map_err(|e| {
            KernelError::InternalError {
                message: format!("{e}"),
                context: None,
            }
        })?,
    ];

    let cell = build_convex_polyhedron(&planes, &BspConfig::default())?;
    let cfg = test_config();
    let result = build_halfedge_mesh(&cell, &cfg)?;
    let (topo, geom, _brep) = result.into_parts();
    Ok((topo, geom))
}

/// Pick a random BooleanOp.
fn random_op(rng: &mut Xorshift64) -> BooleanOp {
    match rng.next_u64() % 3 {
        0 => BooleanOp::Union,
        1 => BooleanOp::Subtraction,
        _ => BooleanOp::Intersection,
    }
}

/// Generate a random Boolean pair from two cubes.
pub fn random_cube_pair(seed: u64) -> Result<BooleanInput, KernelError> {
    let mut rng = Xorshift64::new(seed);
    let (topo_a, geom_a) = random_cube(&mut rng)?;
    let (topo_b, geom_b) = random_cube(&mut rng)?;
    let op = random_op(&mut rng);
    Ok(BooleanInput::new(topo_a, geom_a, BrepState::new(), topo_b, geom_b, BrepState::new(), op))
}

/// Generate a random Boolean pair from two convex solids.
pub fn random_convex_pair(seed: u64) -> Result<BooleanInput, KernelError> {
    let mut rng = Xorshift64::new(seed);

    let seed_a = rng.next_u64();
    let seed_b = rng.next_u64();
    let op = random_op(&mut rng);

    let (topo_a, geom_a) = random_convex_solid(seed_a)?;
    let (topo_b, geom_b) = random_convex_solid(seed_b)?;

    Ok(BooleanInput::new(topo_a, geom_a, BrepState::new(), topo_b, geom_b, BrepState::new(), op))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift64_is_deterministic() {
        let mut a = Xorshift64::new(42);
        let mut b = Xorshift64::new(42);
        let vals_a: Vec<u64> = (0..10).map(|_| a.next_u64()).collect();
        let vals_b: Vec<u64> = (0..10).map(|_| b.next_u64()).collect();
        assert_eq!(vals_a, vals_b);
    }

    #[test]
    fn random_convex_solid_builds() {
        let result = random_convex_solid(12345);
        assert!(result.is_ok(), "Failed: {:?}", result.err());
    }

    #[test]
    fn random_cube_pair_builds() {
        let result = random_cube_pair(99);
        assert!(result.is_ok(), "Failed: {:?}", result.err());
    }

    #[test]
    fn random_convex_pair_builds() {
        let result = random_convex_pair(77);
        assert!(result.is_ok(), "Failed: {:?}", result.err());
    }
}
