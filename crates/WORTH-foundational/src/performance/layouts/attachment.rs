use super::claim::FoundationalLayoutIntentClaim;
use crate::performance::claims::FoundationalPerformanceClaimSurface;
use crate::performance::primitives::FoundationalPerformanceAccessPatternPosture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalLayoutAnnotatedClaimConstructionDenial {
    AccessPatternMismatch {
        claim_access_pattern: FoundationalPerformanceAccessPatternPosture,
        layout_access_pattern: FoundationalPerformanceAccessPatternPosture,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalLayoutAnnotatedClaim<Claim> {
    claim: Claim,
    layout_intent_claim: FoundationalLayoutIntentClaim,
}

impl<Claim> FoundationalLayoutAnnotatedClaim<Claim>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    pub fn new(
        claim: Claim,
        layout_intent_claim: FoundationalLayoutIntentClaim,
    ) -> Result<Self, FoundationalLayoutAnnotatedClaimConstructionDenial> {
        if claim.access_pattern() != layout_intent_claim.access_pattern() {
            return Err(
                FoundationalLayoutAnnotatedClaimConstructionDenial::AccessPatternMismatch {
                    claim_access_pattern: claim.access_pattern(),
                    layout_access_pattern: layout_intent_claim.access_pattern(),
                },
            );
        }

        Ok(Self {
            claim,
            layout_intent_claim,
        })
    }

    pub const fn claim(&self) -> &Claim {
        &self.claim
    }

    pub const fn layout_intent_claim(&self) -> &FoundationalLayoutIntentClaim {
        &self.layout_intent_claim
    }

    pub const fn layout_intent(&self) -> crate::performance::FoundationalPerformanceLayoutIntent {
        self.layout_intent_claim.layout_intent()
    }

    pub const fn allocation_posture(
        &self,
    ) -> crate::performance::FoundationalPerformanceAllocationPosture {
        self.layout_intent_claim.allocation_posture()
    }
}
