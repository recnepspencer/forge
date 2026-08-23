use worth_ui_inspection::{
    UiEvidenceExpansionOutcome, UiEvidenceRichness, UiInspectionForeignEvidenceRef,
    UiInspectionQuery,
};

use super::{
    evidence_materialized_detail::UiEvidenceMaterializedDetail, evidence_reference::UiEvidenceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEvidenceExpansion {
    evidence_ref: UiEvidenceRef,
    requested_richness: UiEvidenceRichness,
    outcome: UiEvidenceExpansionOutcome,
    materialized_detail: Option<UiEvidenceMaterializedDetail>,
    foreign_evidence_refs: Box<[UiInspectionForeignEvidenceRef]>,
    followup_query: Option<UiInspectionQuery>,
}

impl UiEvidenceExpansion {
    pub(crate) fn new(
        evidence_ref: UiEvidenceRef,
        requested_richness: UiEvidenceRichness,
        outcome: UiEvidenceExpansionOutcome,
        materialized_detail: Option<UiEvidenceMaterializedDetail>,
        foreign_evidence_refs: Box<[UiInspectionForeignEvidenceRef]>,
        followup_query: Option<UiInspectionQuery>,
    ) -> Self {
        Self {
            evidence_ref,
            requested_richness,
            outcome,
            materialized_detail,
            foreign_evidence_refs,
            followup_query,
        }
    }

    pub fn evidence_ref(&self) -> UiEvidenceRef {
        self.evidence_ref
    }

    pub fn requested_richness(&self) -> UiEvidenceRichness {
        self.requested_richness
    }

    pub fn outcome(&self) -> UiEvidenceExpansionOutcome {
        self.outcome
    }

    pub fn materialized_detail(&self) -> Option<&UiEvidenceMaterializedDetail> {
        self.materialized_detail.as_ref()
    }

    pub fn foreign_evidence_refs(&self) -> &[UiInspectionForeignEvidenceRef] {
        &self.foreign_evidence_refs
    }

    pub fn followup_query(&self) -> Option<&UiInspectionQuery> {
        self.followup_query.as_ref()
    }
}

#[cfg(test)]
mod evidence_expansion_tests {
    use worth_ui_inspection::{
        UiEvidenceAuthorityGeneration, UiEvidenceExpansionOutcome, UiEvidenceFamily,
        UiEvidenceMaterializationPosture, UiEvidenceRetentionPosture, UiEvidenceRichness,
    };

    use crate::evidence::{
        evidence_authority_binding, evidence_handle, evidence_identity, evidence_ref,
        UiEvidenceAuthorityKind,
    };
    use crate::facade::WorthUi;

    #[test]
    fn expand_evidence_ref_returns_wrong_generation_for_stale_generation_bound_refs() {
        let app = WorthUi::app()
            .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
            .freeze()
            .map(
                crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host,
            )
            .expect("application preparation should succeed");
        let stale_generation =
            UiEvidenceAuthorityGeneration::new(app.graph().generation().as_u64() + 1);
        let identity = evidence_identity(UiEvidenceFamily::Obligation, 17);
        let evidence_ref = evidence_ref(
            UiEvidenceFamily::Obligation,
            identity,
            evidence_authority_binding(
                UiEvidenceAuthorityKind::ObligationAuthority,
                19,
                stale_generation,
                None,
            ),
            UiEvidenceMaterializationPosture::DetailAvailable,
            UiEvidenceRetentionPosture::CurrentGenerationOnly,
            evidence_handle(UiEvidenceFamily::Obligation, identity, 17),
        );

        let expansion = app.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());

        assert_eq!(
            expansion.outcome(),
            UiEvidenceExpansionOutcome::WrongGeneration {
                requested_generation: stale_generation,
                current_generation: UiEvidenceAuthorityGeneration::new(
                    app.graph().generation().as_u64(),
                ),
            }
        );
        assert!(expansion.materialized_detail().is_none());
    }

    #[test]
    fn unknown_current_generation_obligation_ref_is_not_reported_as_materialized() {
        let app = WorthUi::app()
            .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
            .freeze()
            .map(
                crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host,
            )
            .expect("application preparation should succeed");
        let current_generation =
            UiEvidenceAuthorityGeneration::new(app.graph().generation().as_u64());
        let identity = evidence_identity(UiEvidenceFamily::Obligation, 31);
        let evidence_ref = evidence_ref(
            UiEvidenceFamily::Obligation,
            identity,
            evidence_authority_binding(
                UiEvidenceAuthorityKind::ObligationAuthority,
                37,
                current_generation,
                None,
            ),
            UiEvidenceMaterializationPosture::DetailAvailable,
            UiEvidenceRetentionPosture::CurrentGenerationOnly,
            evidence_handle(UiEvidenceFamily::Obligation, identity, 31),
        );

        let expansion = app.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());

        assert_eq!(expansion.outcome(), UiEvidenceExpansionOutcome::Unsupported);
        assert!(expansion.materialized_detail().is_none());
    }
}
