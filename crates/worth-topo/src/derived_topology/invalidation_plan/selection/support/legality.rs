use serde::Serialize;

use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyLegalityReceiptPosture;
use crate::validator_invariant_catalog::WorthTopologySelectedLegalityObligationPlan;
use crate::validator_invariant_catalog::WorthTopologySelectedValidatorEnforcementReceipt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationLegalitySupportEvidence {
    selected_legality_plan_digest: Option<String>,
    selected_validator_receipt_digest: Option<String>,
    support_digest: String,
}

impl DerivedInvalidationLegalitySupportEvidence {
    pub fn missing() -> Self {
        Self::from_parts(None, None)
    }

    pub fn from_selected_legality_plan(plan: &WorthTopologySelectedLegalityObligationPlan) -> Self {
        Self::from_parts(Some(plan.selected_plan_digest().to_string()), None)
    }

    pub fn from_selected_validator_receipt(
        receipt: &WorthTopologySelectedValidatorEnforcementReceipt,
    ) -> Self {
        Self::from_parts(None, Some(receipt.enforcement_receipt_digest().to_string()))
    }

    pub fn from_legality_products(
        selected_legality_plan: Option<&WorthTopologySelectedLegalityObligationPlan>,
        selected_validator_receipt: Option<&WorthTopologySelectedValidatorEnforcementReceipt>,
    ) -> Self {
        Self::from_parts(
            selected_legality_plan.map(|plan| plan.selected_plan_digest().to_string()),
            selected_validator_receipt
                .map(|receipt| receipt.enforcement_receipt_digest().to_string()),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_digests_for_tests(
        selected_legality_plan_digest: Option<String>,
        selected_validator_receipt_digest: Option<String>,
    ) -> Self {
        Self::from_parts(
            selected_legality_plan_digest,
            selected_validator_receipt_digest,
        )
    }

    fn from_parts(
        selected_legality_plan_digest: Option<String>,
        selected_validator_receipt_digest: Option<String>,
    ) -> Self {
        let support_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-legality-support:v1".to_string(),
            digest_part(
                "selected-legality-plan",
                selected_legality_plan_digest.as_deref(),
            ),
            digest_part(
                "selected-validator-receipt",
                selected_validator_receipt_digest.as_deref(),
            ),
        ]);
        Self {
            selected_legality_plan_digest,
            selected_validator_receipt_digest,
            support_digest,
        }
    }

    pub fn supports(&self, posture: DerivedTopologyLegalityReceiptPosture) -> bool {
        self.required_receipt_digest(posture).is_some()
            || posture == DerivedTopologyLegalityReceiptPosture::NotRequiredForFamilyDeclaration
    }

    pub fn required_receipt_digest(
        &self,
        posture: DerivedTopologyLegalityReceiptPosture,
    ) -> Option<&str> {
        match posture {
            DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired => {
                self.selected_legality_plan_digest.as_deref()
            }
            DerivedTopologyLegalityReceiptPosture::SelectedValidatorReceiptRequired => {
                self.selected_validator_receipt_digest.as_deref()
            }
            DerivedTopologyLegalityReceiptPosture::NotRequiredForFamilyDeclaration => None,
        }
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }
}

fn digest_part(label: &str, digest: Option<&str>) -> String {
    format!("{label}:{}", digest.unwrap_or("missing"))
}
