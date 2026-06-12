use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BoundaryOwnershipPolicy {
    kind: BoundaryOwnershipKind,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BoundaryOwnershipKind {
    OwnedClosed,
    OpenUnowned,
    OwnedHalfOpen { convention: String },
}

impl BoundaryOwnershipPolicy {
    pub fn owned_closed() -> Self {
        Self {
            kind: BoundaryOwnershipKind::OwnedClosed,
        }
    }

    pub fn open_unowned() -> Self {
        Self {
            kind: BoundaryOwnershipKind::OpenUnowned,
        }
    }

    pub fn owned_half_open(
        convention: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let convention = canonical_half_open_convention(convention)?;
        Ok(Self {
            kind: BoundaryOwnershipKind::OwnedHalfOpen { convention },
        })
    }

    pub fn kind(&self) -> &BoundaryOwnershipKind {
        &self.kind
    }

    pub fn owns_boundary(&self) -> bool {
        matches!(
            self.kind,
            BoundaryOwnershipKind::OwnedClosed | BoundaryOwnershipKind::OwnedHalfOpen { .. }
        )
    }

    pub fn stable_token(&self) -> String {
        match &self.kind {
            BoundaryOwnershipKind::OwnedClosed => "owned_closed".to_string(),
            BoundaryOwnershipKind::OpenUnowned => "open_unowned".to_string(),
            BoundaryOwnershipKind::OwnedHalfOpen { convention } => {
                format!("owned_half_open:{convention}")
            }
        }
    }
}

fn canonical_half_open_convention(
    convention: impl Into<String>,
) -> Result<String, HadwigerArtifactShapeError> {
    let convention = require_non_empty(convention, "boundary_half_open_convention")?;
    let mut left = false;
    let mut right = false;
    let mut bottom = false;
    let mut top = false;
    for raw_side in convention.split(',') {
        match raw_side.trim() {
            "left" if !left => left = true,
            "right" if !right => right = true,
            "bottom" if !bottom => bottom = true,
            "top" if !top => top = true,
            _ => {
                return Err(HadwigerArtifactShapeError::EmptyField {
                    field: "boundary_half_open_side",
                });
            }
        }
    }
    let mut sides = Vec::new();
    if left {
        sides.push("left");
    }
    if right {
        sides.push("right");
    }
    if bottom {
        sides.push("bottom");
    }
    if top {
        sides.push("top");
    }
    if sides.is_empty() {
        return Err(HadwigerArtifactShapeError::EmptyField {
            field: "boundary_half_open_side",
        });
    }
    Ok(sides.join(","))
}
