use super::{PlanarStructuralIdentityDenial, PlanarStructuralIdentityDenialKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarOrientationPolicy {
    Preserve,
    ReverseDenied,
}

impl PlanarOrientationPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserve => "preserve",
            Self::ReverseDenied => "reverse-denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalPlanarTransformBasis {
    local_frame_identity: String,
    movement_rotation_posture_identity: String,
    transform_chain_digest: String,
    orientation_policy: PlanarOrientationPolicy,
}

impl CanonicalPlanarTransformBasis {
    pub fn builder() -> CanonicalPlanarTransformBasisBuilder {
        CanonicalPlanarTransformBasisBuilder::default()
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }

    pub fn transform_chain_digest(&self) -> &str {
        &self.transform_chain_digest
    }

    pub fn orientation_policy(&self) -> PlanarOrientationPolicy {
        self.orientation_policy
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CanonicalPlanarTransformBasisBuilder {
    local_frame_identity: Option<String>,
    movement_rotation_posture_identity: Option<String>,
    transform_chain_digest: Option<String>,
    orientation_policy: Option<PlanarOrientationPolicy>,
}

impl CanonicalPlanarTransformBasisBuilder {
    pub fn local_frame(mut self, identity: impl Into<String>) -> Self {
        self.local_frame_identity = Some(identity.into());
        self
    }

    pub fn movement_rotation_posture(mut self, identity: impl Into<String>) -> Self {
        self.movement_rotation_posture_identity = Some(identity.into());
        self
    }

    pub fn transform_chain_digest(mut self, digest: impl Into<String>) -> Self {
        self.transform_chain_digest = Some(digest.into());
        self
    }

    pub fn orientation_policy(mut self, policy: PlanarOrientationPolicy) -> Self {
        self.orientation_policy = Some(policy);
        self
    }

    pub fn build(self) -> Result<CanonicalPlanarTransformBasis, PlanarStructuralIdentityDenial> {
        let basis = CanonicalPlanarTransformBasis {
            local_frame_identity: required(self.local_frame_identity, "local frame")?,
            movement_rotation_posture_identity: required(
                self.movement_rotation_posture_identity,
                "movement/rotation posture",
            )?,
            transform_chain_digest: required(self.transform_chain_digest, "transform chain")?,
            orientation_policy: self
                .orientation_policy
                .ok_or_else(|| missing("orientation policy"))?,
        };
        Ok(basis)
    }
}

fn required(
    value: Option<String>,
    label: &'static str,
) -> Result<String, PlanarStructuralIdentityDenial> {
    match value {
        Some(value) if !value.is_empty() => Ok(value),
        _ => Err(missing(label)),
    }
}

fn missing(label: &'static str) -> PlanarStructuralIdentityDenial {
    PlanarStructuralIdentityDenial::new(
        PlanarStructuralIdentityDenialKind::MissingCanonicalTransformBasis,
        format!("planar structural identity requires {label} in the canonical transform basis"),
    )
}
