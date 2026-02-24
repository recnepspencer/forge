// Shared traits for geometry primitives

use forge_core::KernelError;

pub trait Eval {
    // TBD
}

pub trait Schema {
    // TBD
}

pub trait Bound {
    // TBD
}

pub trait Intersect {
    // TBD
}

/// Trait for geometry providers that can evaluate a surface normal at a world-space point.
pub trait EvaluateNormal {
    /// Compute a unit normal at `point` on the represented surface.
    fn normal_at(&self, point: &[f64; 3]) -> Result<[f64; 3], KernelError>;
}
