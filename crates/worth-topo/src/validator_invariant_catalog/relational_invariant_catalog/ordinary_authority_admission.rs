use super::{
    WorthTopologyRelationalInvariantCatalogDenial,
    WorthTopologyRelationalInvariantCatalogDenialKind,
    WorthTopologyRelationalInvariantQueryRegistrationBundle,
};
use crate::validator_invariant_catalog::{
    WorthTopologySelectedLegalityObligationPlan,
    WorthTopologySelectedValidatorEnforcementPhaseFiveSeed,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopologyRelationalInvariantRejectedAuthorityKind {
    StaticInvariantPack,
    ManualGraphCompositionInvariantPack,
    ExplicitRelationalRuntimeAuthority,
    MixedQueryAndRelationalRuntimeAuthority,
}

impl WorthTopologyRelationalInvariantRejectedAuthorityKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaticInvariantPack => "static-invariant-pack",
            Self::ManualGraphCompositionInvariantPack => "manual-graph-composition-invariant-pack",
            Self::ExplicitRelationalRuntimeAuthority => "explicit-relational-runtime-authority",
            Self::MixedQueryAndRelationalRuntimeAuthority => {
                "mixed-query-and-relational-runtime-authority"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyRelationalInvariantOrdinaryAuthorityAdmission {
    selected_plan_digest: String,
    validator_phase_five_seed_digest: String,
    query_registration_bundle_digest: String,
    admission_digest: String,
}

impl WorthTopologyRelationalInvariantOrdinaryAuthorityAdmission {
    pub(in crate::validator_invariant_catalog) fn from_query_registered_catalog(
        selected_plan: &WorthTopologySelectedLegalityObligationPlan,
        validator_phase_five_seed: &WorthTopologySelectedValidatorEnforcementPhaseFiveSeed,
        query_registration_bundle: &WorthTopologyRelationalInvariantQueryRegistrationBundle,
    ) -> Result<Self, WorthTopologyRelationalInvariantCatalogDenial> {
        if validator_phase_five_seed.selected_plan_digest() != selected_plan.selected_plan_digest()
        {
            return Err(WorthTopologyRelationalInvariantCatalogDenial::new(
                WorthTopologyRelationalInvariantCatalogDenialKind::ValidatorSeedMismatch,
                validator_phase_five_seed.seed_digest(),
                "validator Phase 5 seed must be derived from the selected Query obligation plan",
            ));
        }
        if query_registration_bundle.graph_scoped_custom_invariant_count() == 0 {
            return Err(WorthTopologyRelationalInvariantCatalogDenial::new(
                WorthTopologyRelationalInvariantCatalogDenialKind::QueryRegistrationArtifactMissing,
                query_registration_bundle.bundle_digest(),
                "ordinary relational invariant authority requires graph-scoped Query registrations",
            ));
        }
        let selected_plan_digest = selected_plan.selected_plan_digest().to_string();
        let validator_phase_five_seed_digest = validator_phase_five_seed.seed_digest().to_string();
        let query_registration_bundle_digest =
            query_registration_bundle.bundle_digest().to_string();
        let admission_digest = [
            "worth-topo-relational-invariant-ordinary-authority-admission-v1",
            selected_plan_digest.as_str(),
            validator_phase_five_seed_digest.as_str(),
            query_registration_bundle_digest.as_str(),
        ]
        .join("|");
        Ok(Self {
            selected_plan_digest,
            validator_phase_five_seed_digest,
            query_registration_bundle_digest,
            admission_digest,
        })
    }

    pub fn reject_non_query_authority(
        kind: WorthTopologyRelationalInvariantRejectedAuthorityKind,
        subject_digest: impl Into<String>,
    ) -> WorthTopologyRelationalInvariantCatalogDenial {
        WorthTopologyRelationalInvariantCatalogDenial::new(
            WorthTopologyRelationalInvariantCatalogDenialKind::RejectedNonQueryAuthority,
            subject_digest,
            format!(
                "`{}` cannot satisfy ordinary relational invariant closeout; ordinary authority must enter through Query-selected graph-scoped registrations",
                kind.as_str()
            ),
        )
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn validator_phase_five_seed_digest(&self) -> &str {
        &self.validator_phase_five_seed_digest
    }

    pub fn query_registration_bundle_digest(&self) -> &str {
        &self.query_registration_bundle_digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }
}
