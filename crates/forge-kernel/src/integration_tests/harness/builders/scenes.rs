//! SceneBuilder — fluent multi-solid composition.
//!
//! DOMAIN: Equivalent to Laravel's nested `Factory::has(...)` calls.
//! Builds multiple real solids in a single call, optionally with a
//! shared config. Returns `Vec<SolidEnvelope>`.
//!
//! ```rust,ignore
//! let solids = SceneBuilder::new()
//!     .cube([0.0, 0.0, 0.0], 2.0)
//!     .cube([1.5, 0.0, 0.0], 2.0)
//!     .tetrahedron([5.0, 0.0, 0.0], 1.0)
//!     .with_config(config_tight())
//!     .build()?;
//! ```

use crate::configuration::facade::ResolvedConfig;
use crate::context::ModelingContext;
use crate::context::scope::OperationScope;
use crate::engine::facade::SolidEnvelope;
use crate::operations::primitives;
use forge_core::KernelError;

use super::configs::test_config;

/// A pending shape to build.
enum PendingShape {
    Cube { center: [f64; 3], size: f64 },
    Block { center: [f64; 3], half_extents: [f64; 3] },
    Tetrahedron { center: [f64; 3], scale: f64 },
    Dodecahedron { center: [f64; 3], radius: f64 },
    Prism { center: [f64; 3], sides: u32, radius: f64, height: f64 },
    Pyramid { center: [f64; 3], sides: u32, radius: f64, height: f64 },
    Wedge { center: [f64; 3], half_extents: [f64; 3] },
}

/// Fluent builder for composing multi-solid test scenes.
pub struct SceneBuilder {
    shapes: Vec<PendingShape>,
    config: Option<ResolvedConfig>,
}

impl SceneBuilder {
    /// Create a new empty scene.
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            config: None,
        }
    }

    /// Set a custom config for all shapes in this scene.
    pub fn with_config(mut self, config: ResolvedConfig) -> Self {
        self.config = Some(config);
        self
    }

    // ── Shape methods ────────────────────────────────────────────────────

    /// Add a cube to the scene.
    pub fn cube(mut self, center: [f64; 3], size: f64) -> Self {
        self.shapes.push(PendingShape::Cube { center, size });
        self
    }

    /// Add a block to the scene.
    pub fn block(mut self, center: [f64; 3], half_extents: [f64; 3]) -> Self {
        self.shapes.push(PendingShape::Block { center, half_extents });
        self
    }

    /// Add a tetrahedron to the scene.
    pub fn tetrahedron(mut self, center: [f64; 3], scale: f64) -> Self {
        self.shapes.push(PendingShape::Tetrahedron { center, scale });
        self
    }

    /// Add a dodecahedron to the scene.
    pub fn dodecahedron(mut self, center: [f64; 3], radius: f64) -> Self {
        self.shapes.push(PendingShape::Dodecahedron { center, radius });
        self
    }

    /// Add a prism to the scene.
    pub fn prism(mut self, center: [f64; 3], sides: u32, radius: f64, height: f64) -> Self {
        self.shapes.push(PendingShape::Prism { center, sides, radius, height });
        self
    }

    /// Add a pyramid to the scene.
    pub fn pyramid(mut self, center: [f64; 3], sides: u32, radius: f64, height: f64) -> Self {
        self.shapes.push(PendingShape::Pyramid { center, sides, radius, height });
        self
    }

    /// Add a wedge to the scene.
    pub fn wedge(mut self, center: [f64; 3], half_extents: [f64; 3]) -> Self {
        self.shapes.push(PendingShape::Wedge { center, half_extents });
        self
    }

    // ── Build ────────────────────────────────────────────────────────────

    /// Build all shapes and return them as a vector.
    pub fn build(self) -> Result<Vec<SolidEnvelope>, KernelError> {
        let config = self.config.unwrap_or_else(test_config);
        let mut results = Vec::with_capacity(self.shapes.len());

        for shape in self.shapes {
            let mut ctx = ModelingContext::new();
            let mut scope = OperationScope::new(&config, &mut ctx);
            let envelope = match shape {
                PendingShape::Cube { center, size } => {
                    primitives::make_cube(center, size, &mut scope)?
                }
                PendingShape::Block { center, half_extents } => {
                    primitives::make_block(center, half_extents, &mut scope)?
                }
                PendingShape::Tetrahedron { center, scale } => {
                    primitives::make_tetrahedron(center, scale, &mut scope)?
                }
                PendingShape::Dodecahedron { center, radius } => {
                    primitives::make_dodecahedron(center, radius, &mut scope)?
                }
                PendingShape::Prism { center, sides, radius, height } => {
                    primitives::make_prism(center, sides, radius, height, &mut scope)?
                }
                PendingShape::Pyramid { center, sides, radius, height } => {
                    primitives::make_pyramid(center, sides, radius, height, &mut scope)?
                }
                PendingShape::Wedge { center, half_extents } => {
                    primitives::make_wedge(center, half_extents, &mut scope)?
                }
            };
            results.push(envelope);
        }

        Ok(results)
    }

    /// Build all shapes and return them as a pair (convenience for 2-solid scenes).
    pub fn build_pair(self) -> Result<(SolidEnvelope, SolidEnvelope), KernelError> {
        let mut solids = self.build()?;
        if solids.len() != 2 {
            return Err(KernelError::InvalidInput {
                message: format!("build_pair() requires exactly 2 shapes, got {}", solids.len()),
                context: None,
            });
        }
        let b = solids.pop().unwrap();
        let a = solids.pop().unwrap();
        Ok((a, b))
    }
}
