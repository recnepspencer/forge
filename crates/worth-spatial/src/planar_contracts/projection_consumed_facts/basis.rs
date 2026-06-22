use crate::planar_contracts::projection_2d::ProjectPointToCertifiedPlane2DReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsReceipt;

use super::{
    validate_projection_consumed_planar_facts_basis, ProjectionConsumedPlanarFactsDenial,
    ProjectionConsumedPlanarFactsDenialKind,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectionConsumedPlanarFactsBasis {
    retained_planar_facts_receipt: RetainedPlanarFactsReceipt,
    projection_receipts: Vec<ProjectPointToCertifiedPlane2DReceipt>,
    materialization_basis_identity: String,
}

impl ProjectionConsumedPlanarFactsBasis {
    pub fn builder() -> ProjectionConsumedPlanarFactsBuilder {
        ProjectionConsumedPlanarFactsBuilder::default()
    }

    pub(crate) fn from_builder(
        builder: ProjectionConsumedPlanarFactsBuilder,
    ) -> Result<Self, ProjectionConsumedPlanarFactsDenial> {
        let materialization_basis_identity = builder
            .materialization_basis_identity
            .unwrap_or_else(|| "materialization:planar-projection-consumption".to_string());
        if materialization_basis_identity.trim().is_empty() {
            return Err(ProjectionConsumedPlanarFactsDenial::new(
                ProjectionConsumedPlanarFactsDenialKind::InvalidMaterializationBasis,
                "projection-consumed planar facts require a non-empty materialization basis identity",
            ));
        }
        let basis = Self {
            retained_planar_facts_receipt: builder.retained_planar_facts_receipt.ok_or_else(
                || {
                    ProjectionConsumedPlanarFactsDenial::new(
                        ProjectionConsumedPlanarFactsDenialKind::MissingRetainedPlanarFactsReceipt,
                        "projection-consumed planar facts require a retained planar facts receipt as source truth",
                    )
                },
            )?,
            projection_receipts: canonical_projection_receipt_order(builder.projection_receipts),
            materialization_basis_identity,
        };
        validate_projection_consumed_planar_facts_basis(&basis)?;
        Ok(basis)
    }

    pub fn retained_planar_facts_receipt(&self) -> &RetainedPlanarFactsReceipt {
        &self.retained_planar_facts_receipt
    }

    pub fn projection_receipts(&self) -> &[ProjectPointToCertifiedPlane2DReceipt] {
        &self.projection_receipts
    }

    pub fn materialization_basis_identity(&self) -> &str {
        &self.materialization_basis_identity
    }

    pub fn retained_planar_fact_digest(&self) -> &str {
        self.retained_planar_facts_receipt.retained_fact_digest()
    }

    pub fn structural_identity_digest(&self) -> &str {
        self.retained_planar_facts_receipt
            .basis()
            .structural_identity_receipt()
            .structural_identity_digest()
    }

    pub fn motion_posture_digest(&self) -> &str {
        self.retained_planar_facts_receipt
            .basis()
            .motion_posture_receipt()
            .retained_motion_digest()
    }

    pub fn topology_contract_digest(&self) -> &str {
        self.retained_planar_facts_receipt
            .basis()
            .topology_contract_receipt()
            .fact_digest()
    }
}

fn canonical_projection_receipt_order(
    mut receipts: Vec<ProjectPointToCertifiedPlane2DReceipt>,
) -> Vec<ProjectPointToCertifiedPlane2DReceipt> {
    receipts.sort_by(|left, right| left.fact_digest().cmp(right.fact_digest()));
    receipts
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProjectionConsumedPlanarFactsBuilder {
    retained_planar_facts_receipt: Option<RetainedPlanarFactsReceipt>,
    projection_receipts: Vec<ProjectPointToCertifiedPlane2DReceipt>,
    materialization_basis_identity: Option<String>,
}

impl ProjectionConsumedPlanarFactsBuilder {
    pub fn retained_planar_facts(mut self, receipt: RetainedPlanarFactsReceipt) -> Self {
        self.retained_planar_facts_receipt = Some(receipt);
        self
    }

    pub fn projection_receipts<I>(mut self, receipts: I) -> Self
    where
        I: IntoIterator<Item = ProjectPointToCertifiedPlane2DReceipt>,
    {
        self.projection_receipts = receipts.into_iter().collect();
        self
    }

    pub fn materialization_basis(mut self, identity: impl Into<String>) -> Self {
        self.materialization_basis_identity = Some(identity.into());
        self
    }

    pub fn build(
        self,
    ) -> Result<ProjectionConsumedPlanarFactsBasis, ProjectionConsumedPlanarFactsDenial> {
        ProjectionConsumedPlanarFactsBasis::from_builder(self)
    }
}
