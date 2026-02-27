//! Classification data shapes.
//!
//! DOMAIN: Face classification labels and classified face structs used
//! by both the parametric and ember boolean pipelines.

use forge_topo::handles::FaceId;
use serde::{Deserialize, Serialize};

/// Classification of a face relative to another solid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FaceClassification {
    /// Face is strictly inside the other solid.
    Inside,
    /// Face is strictly outside the other solid.
    Outside,
    /// Face classification is ambiguous and requires resolver policy.
    ///
    /// Used for split fragments where point-sample classification is
    /// topologically unsafe to consume directly in selection.
    Ambiguous,
    /// Face is on the boundary (coplanar) with same normal alignment.
    OnBoundary,
    /// Face is on the boundary (coplanar) with opposite normal alignment.
    OppositeBoundary,
}

/// Which input solid a face originated from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaceOrigin {
    /// Face came from the target solid.
    Target,
    /// Face came from the tool solid.
    Tool,
}

/// A classified face with its origin and classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedFace {
    /// The face handle.
    face: FaceId,
    /// Classification relative to the other solid.
    classification: FaceClassification,
}

impl ClassifiedFace {
    /// Create a new classified face.
    pub fn new(face: FaceId, classification: FaceClassification) -> Self {
        Self {
            face,
            classification,
        }
    }

    /// The face handle.
    pub fn face(&self) -> FaceId {
        self.face
    }

    /// Classification relative to the other solid.
    pub fn classification(&self) -> FaceClassification {
        self.classification
    }

    /// Override the classification (used by coplanar resolution).
    pub fn set_classification(&mut self, c: FaceClassification) {
        self.classification = c;
    }
}
