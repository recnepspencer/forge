use crate::planar_contracts::contract_bundle::PlanarContractBundleValidationReceipt;
use crate::planar_contracts::motion_posture::PlanarMotionPostureReceipt;
use crate::planar_contracts::structural_identity::PlanarStructuralIdentityReceipt;
use crate::planar_contracts::topology_contract_completeness::PlanarTopologyContractCompletenessReceipt;

use super::{RetainedPlanarFactsDenial, RetainedPlanarFactsDenialKind};

#[derive(Clone, Debug, PartialEq)]
pub struct RetainedPlanarFactsBasis {
    boolean_readiness_receipt: PlanarContractBundleValidationReceipt,
    structural_identity_receipt: PlanarStructuralIdentityReceipt,
    motion_posture_receipt: PlanarMotionPostureReceipt,
    topology_contract_receipt: PlanarTopologyContractCompletenessReceipt,
    retain_planar_classification: bool,
}

impl RetainedPlanarFactsBasis {
    pub fn builder() -> RetainedPlanarFactsBuilder {
        RetainedPlanarFactsBuilder::default()
    }

    pub fn boolean_readiness_receipt(&self) -> &PlanarContractBundleValidationReceipt {
        &self.boolean_readiness_receipt
    }

    pub fn structural_identity_receipt(&self) -> &PlanarStructuralIdentityReceipt {
        &self.structural_identity_receipt
    }

    pub fn motion_posture_receipt(&self) -> &PlanarMotionPostureReceipt {
        &self.motion_posture_receipt
    }

    pub fn topology_contract_receipt(&self) -> &PlanarTopologyContractCompletenessReceipt {
        &self.topology_contract_receipt
    }

    pub fn retains_planar_classification(&self) -> bool {
        self.retain_planar_classification
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RetainedPlanarFactsBuilder {
    boolean_readiness_receipt: Option<PlanarContractBundleValidationReceipt>,
    structural_identity_receipt: Option<PlanarStructuralIdentityReceipt>,
    motion_posture_receipt: Option<PlanarMotionPostureReceipt>,
    topology_contract_receipt: Option<PlanarTopologyContractCompletenessReceipt>,
    retain_planar_classification: bool,
}

impl RetainedPlanarFactsBuilder {
    pub fn boolean_readiness_receipt(
        mut self,
        receipt: PlanarContractBundleValidationReceipt,
    ) -> Self {
        self.boolean_readiness_receipt = Some(receipt);
        self
    }

    pub fn structural_identity_receipt(mut self, receipt: PlanarStructuralIdentityReceipt) -> Self {
        self.structural_identity_receipt = Some(receipt);
        self
    }

    pub fn motion_posture_receipt(mut self, receipt: PlanarMotionPostureReceipt) -> Self {
        self.motion_posture_receipt = Some(receipt);
        self
    }

    pub fn topology_contract_receipt(
        mut self,
        receipt: PlanarTopologyContractCompletenessReceipt,
    ) -> Self {
        self.topology_contract_receipt = Some(receipt);
        self
    }

    pub fn retain_planar_classification(mut self) -> Self {
        self.retain_planar_classification = true;
        self
    }

    pub fn build(self) -> Result<RetainedPlanarFactsBasis, RetainedPlanarFactsDenial> {
        let boolean_readiness_receipt =
            self.boolean_readiness_receipt.ok_or_else(|| {
                RetainedPlanarFactsDenial::new(
                    RetainedPlanarFactsDenialKind::MissingBooleanReadinessReceipt,
                    "retained planar facts require a boolean-readiness receipt as the frozen classification root",
                )
            })?;
        let structural_identity_receipt =
            self.structural_identity_receipt.ok_or_else(|| {
                RetainedPlanarFactsDenial::new(
                    RetainedPlanarFactsDenialKind::MissingStructuralIdentityReceipt,
                    "retained planar facts require structural identity so replay is not repaired from live names",
                )
            })?;
        let motion_posture_receipt = self.motion_posture_receipt.ok_or_else(|| {
            RetainedPlanarFactsDenial::new(
                RetainedPlanarFactsDenialKind::MissingMotionPostureReceipt,
                "retained planar facts require explicit movement and rotation posture before replay",
            )
        })?;
        let topology_contract_receipt = self.topology_contract_receipt.ok_or_else(|| {
            RetainedPlanarFactsDenial::new(
                RetainedPlanarFactsDenialKind::MissingTopologyContractReceipt,
                "retained planar facts require topology contract completeness before replay",
            )
        })?;
        if !self.retain_planar_classification {
            return Err(RetainedPlanarFactsDenial::new(
                RetainedPlanarFactsDenialKind::MissingPlanarClassificationRetention,
                "retained planar facts must explicitly retain planar classification",
            ));
        }
        let basis = RetainedPlanarFactsBasis {
            boolean_readiness_receipt,
            structural_identity_receipt,
            motion_posture_receipt,
            topology_contract_receipt,
            retain_planar_classification: self.retain_planar_classification,
        };
        super::validation::validate_retained_planar_facts_basis(&basis)?;
        Ok(basis)
    }
}
