use crate::planar_contracts::predicate_authority::PlanarPredicateFactReceipt;

use super::validation::validate_planar_precision_basis;
use super::PlanarPrecisionBasisDenial;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarPrecisionBasis {
    local_frame_identity: String,
    topology_basis_identity: String,
    movement_rotation_posture_identity: String,
    tolerance_policy_identity: String,
    predicate_fact_digest: String,
    predicate_declaration_digest: String,
    predicate_envelope_digest: String,
    local_feature_scale_order: i32,
    world_magnitude_order: i32,
    normalization_scale: f64,
}

impl PlanarPrecisionBasis {
    pub fn builder() -> PlanarPrecisionBasisBuilder {
        PlanarPrecisionBasisBuilder::default()
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn topology_basis_identity(&self) -> &str {
        &self.topology_basis_identity
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }

    pub fn tolerance_policy_identity(&self) -> &str {
        &self.tolerance_policy_identity
    }

    pub fn predicate_fact_digest(&self) -> &str {
        &self.predicate_fact_digest
    }

    pub fn predicate_declaration_digest(&self) -> &str {
        &self.predicate_declaration_digest
    }

    pub fn predicate_envelope_digest(&self) -> &str {
        &self.predicate_envelope_digest
    }

    pub fn local_feature_scale_order(&self) -> i32 {
        self.local_feature_scale_order
    }

    pub fn world_magnitude_order(&self) -> i32 {
        self.world_magnitude_order
    }

    pub fn normalization_scale(&self) -> f64 {
        self.normalization_scale
    }

    pub fn scale_separation_orders(&self) -> i32 {
        self.world_magnitude_order - self.local_feature_scale_order
    }

    pub(crate) fn from_builder(
        builder: PlanarPrecisionBasisBuilder,
    ) -> Result<Self, PlanarPrecisionBasisDenial> {
        let local_feature_scale_order = builder.local_feature_scale_order.ok_or_else(|| {
            PlanarPrecisionBasisDenial::new(
                super::PlanarPrecisionBasisDenialKind::MissingLocalFeatureScaleOrder,
                "local feature scale order is required",
            )
        })?;
        let world_magnitude_order = builder.world_magnitude_order.ok_or_else(|| {
            PlanarPrecisionBasisDenial::new(
                super::PlanarPrecisionBasisDenialKind::MissingWorldMagnitudeOrder,
                "world magnitude order is required",
            )
        })?;
        let basis = Self {
            local_frame_identity: builder.local_frame_identity.unwrap_or_default(),
            topology_basis_identity: builder.topology_basis_identity.unwrap_or_default(),
            movement_rotation_posture_identity: builder
                .movement_rotation_posture_identity
                .unwrap_or_default(),
            tolerance_policy_identity: builder.tolerance_policy_identity.unwrap_or_default(),
            predicate_fact_digest: builder.predicate_fact_digest.unwrap_or_default(),
            predicate_declaration_digest: builder.predicate_declaration_digest.unwrap_or_default(),
            predicate_envelope_digest: builder.predicate_envelope_digest.unwrap_or_default(),
            local_feature_scale_order,
            world_magnitude_order,
            normalization_scale: builder.normalization_scale.unwrap_or(f64::NAN),
        };
        validate_planar_precision_basis(&basis, builder.predicate_basis)?;
        Ok(basis)
    }
}

#[derive(Clone, Debug, Default)]
pub struct PlanarPrecisionBasisBuilder {
    local_frame_identity: Option<String>,
    topology_basis_identity: Option<String>,
    movement_rotation_posture_identity: Option<String>,
    tolerance_policy_identity: Option<String>,
    predicate_fact_digest: Option<String>,
    predicate_declaration_digest: Option<String>,
    predicate_envelope_digest: Option<String>,
    local_feature_scale_order: Option<i32>,
    world_magnitude_order: Option<i32>,
    normalization_scale: Option<f64>,
    predicate_basis: Option<PredicateBasisSnapshot>,
}

#[derive(Clone, Debug)]
pub(crate) struct PredicateBasisSnapshot {
    pub(crate) local_frame_identity: String,
    pub(crate) topology_basis_identity: String,
    pub(crate) movement_rotation_posture_identity: String,
    pub(crate) tolerance_policy_identity: String,
}

impl PlanarPrecisionBasisBuilder {
    pub fn local_frame_identity(mut self, identity: impl Into<String>) -> Self {
        self.local_frame_identity = Some(identity.into());
        self
    }

    pub fn topology_basis_identity(mut self, identity: impl Into<String>) -> Self {
        self.topology_basis_identity = Some(identity.into());
        self
    }

    pub fn movement_rotation_posture_identity(mut self, identity: impl Into<String>) -> Self {
        self.movement_rotation_posture_identity = Some(identity.into());
        self
    }

    pub fn tolerance_policy_identity(mut self, identity: impl Into<String>) -> Self {
        self.tolerance_policy_identity = Some(identity.into());
        self
    }

    pub fn local_feature_scale_order(mut self, order: i32) -> Self {
        self.local_feature_scale_order = Some(order);
        self
    }

    pub fn world_magnitude_order(mut self, order: i32) -> Self {
        self.world_magnitude_order = Some(order);
        self
    }

    pub fn normalization_scale(mut self, scale: f64) -> Self {
        self.normalization_scale = Some(scale);
        self
    }

    pub fn predicate_receipt(mut self, receipt: &PlanarPredicateFactReceipt) -> Self {
        self.predicate_fact_digest = Some(receipt.fact_digest().to_string());
        self.predicate_declaration_digest = Some(receipt.declaration_digest().to_string());
        self.predicate_envelope_digest = Some(receipt.envelope_digest().to_string());
        self.predicate_basis = Some(PredicateBasisSnapshot {
            local_frame_identity: receipt.input_basis().local_frame_identity().to_string(),
            topology_basis_identity: receipt.input_basis().topology_basis_identity().to_string(),
            movement_rotation_posture_identity: receipt
                .input_basis()
                .movement_rotation_posture_identity()
                .to_string(),
            tolerance_policy_identity: receipt
                .input_basis()
                .tolerance_policy_identity()
                .to_string(),
        });
        self
    }

    pub fn build(self) -> Result<PlanarPrecisionBasis, PlanarPrecisionBasisDenial> {
        PlanarPrecisionBasis::from_builder(self)
    }
}
