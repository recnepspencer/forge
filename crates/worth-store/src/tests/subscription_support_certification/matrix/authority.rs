use super::super::{
    publish_exact, raw_materialized, RawSubscriptionSupportDeclaration,
    SubscriptionResumeClassification, SubscriptionSupportAllocationScope,
    SubscriptionSupportAuthority, SubscriptionSupportCatalog,
    SubscriptionSupportCertificationLaneKind, SubscriptionSupportCertificationLaneOutcome,
    SubscriptionSupportClassificationPlan, SubscriptionSupportCounterSnapshot,
    SubscriptionSupportDensityClass, SubscriptionSupportDriftCause, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportFetchRequest,
    SubscriptionSupportMissingSupportRecoveryRequest, SubscriptionSupportPayloadBudget,
    SubscriptionSupportPayloadDigest, SubscriptionSupportPlanFamily,
    SubscriptionSupportResumeEvidence, SubscriptionSupportResumeRequest, SubscriptionSupportRole,
    SubscriptionSupportScope, WORTHStoreBuilder,
};
use super::evidence::CertificationMatrixEvidence;

pub(super) fn record_authority(evidence: &mut CertificationMatrixEvidence) {
    let unknown_authority_error = {
        let raw = RawSubscriptionSupportDeclaration::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportAuthority::Unadmitted("phase-5b-hostile".into()),
            "worth-query-live-v1",
            SubscriptionSupportScope::from_canonical(vec!["feed:2".into()]).unwrap(),
            SubscriptionSupportPayloadDigest::new("payload:abc").unwrap(),
        );
        SubscriptionSupportCatalog::first_ship()
            .admit(raw)
            .expect_err("unknown upstream authority must reject before publication")
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::UnknownUpstreamAuthorityRejected,
            unknown_authority_error.kind().clone(),
            SubscriptionSupportCounterSnapshot::default(),
        )
        .unwrap(),
    );

    let non_canonical_error =
        SubscriptionSupportScope::from_canonical(vec!["z".into(), "a".into()])
            .expect_err("non-canonical support scope must reject before identity calculation");
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::NonCanonicalScopeRejected,
            non_canonical_error.kind().clone(),
            SubscriptionSupportCounterSnapshot::default(),
        )
        .unwrap(),
    );

    let unsupported_family_error = {
        let raw = RawSubscriptionSupportDeclaration::new(
            SubscriptionSupportFamilyId::new("unsupported-subscription-support-family").unwrap(),
            SubscriptionSupportFamilyKind::ExtensionDefinedSupport,
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportAuthority::WorthQuery,
            "worth-query-live-v1",
            SubscriptionSupportScope::from_canonical(vec!["feed:2".into()]).unwrap(),
            SubscriptionSupportPayloadDigest::new("payload:abc").unwrap(),
        );
        SubscriptionSupportCatalog::first_ship()
            .admit(raw)
            .expect_err("unsupported family identity must reject before publication")
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::UnsupportedFamilyKindRejected,
            unsupported_family_error.kind().clone(),
            SubscriptionSupportCounterSnapshot::default(),
        )
        .unwrap(),
    );

    let compatibility_precedence_report = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:compat", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_compatibility_digest("compatibility:drift")
            .unwrap()
            .with_support_artifact_digest("artifact:drift")
            .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        compatibility_precedence_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportCompatibilityDrift)
    );
    assert_eq!(
        compatibility_precedence_report.suppressed_causes(),
        &[SubscriptionSupportDriftCause::SubscriptionSupportDigestMismatch]
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::MultiDriftCompatibilityPrecedence,
            &compatibility_precedence_report,
        )
        .unwrap(),
    );
    evidence.record_classification_report(compatibility_precedence_report);

    let missing_rebuild_basis_report = {
        let missing_artifact_id = {
            let mut source = WORTHStoreBuilder::new().in_memory().build().unwrap();
            let admitted = source
                .admit_subscription_support_declaration(raw_materialized())
                .unwrap();
            let publishable = source
                .subscription_support_pipeline()
                .prepare_exact(
                    admitted,
                    "basis:missing",
                    "cursor:missing",
                    "checkpoint:missing",
                    "schema:1",
                    "compatibility:1",
                )
                .unwrap();
            source
                .publish_subscription_support(publishable)
                .unwrap()
                .artifact_id()
                .clone()
        };
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let report = store
            .classify_missing_subscription_support(
                SubscriptionSupportMissingSupportRecoveryRequest::new(
                    SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
                    SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
                    SubscriptionSupportRole::NarrowingMaterialization,
                    missing_artifact_id,
                    "basis:missing",
                    "cursor:missing",
                    "checkpoint:missing",
                    "compatibility:1",
                    "portability:1",
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(
            report.classification(),
            SubscriptionResumeClassification::NotResumable
        );
        (report, store.subscription_support_counters())
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_missing_support_recovery(
            SubscriptionSupportCertificationLaneKind::RebuildBasisMissingNotResumable,
            &missing_rebuild_basis_report.0,
            missing_rebuild_basis_report.1,
        )
        .unwrap(),
    );

    let batch_debt_report = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_materialized())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:batch",
                "cursor:batch",
                "checkpoint:batch",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        let published = store.publish_subscription_support(publishable).unwrap();
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
                SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
                published.artifact_id().clone(),
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_support_artifact_digest("artifact:drift")
            .unwrap();
        let plan = SubscriptionSupportClassificationPlan::new(
            SubscriptionSupportPlanFamily::DeniedResumeClassificationPlan,
            SubscriptionSupportPayloadBudget::new(16 * 1024, 64).unwrap(),
            SubscriptionSupportAllocationScope::FamilyLocalScratch,
            SubscriptionSupportDensityClass::FamilyBatchClassificationDebt,
            None,
        )
        .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched, evidence, plan,
            ))
            .unwrap()
    };
    assert_eq!(
        batch_debt_report.cost_surface().density_class(),
        SubscriptionSupportDensityClass::FamilyBatchClassificationDebt
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::BatchClassificationDebt,
            &batch_debt_report,
        )
        .unwrap(),
    );
    evidence.record_classification_report(batch_debt_report);
}
