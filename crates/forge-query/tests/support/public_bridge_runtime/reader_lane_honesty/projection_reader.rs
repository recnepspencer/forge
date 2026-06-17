use forge_query::facade::{
    public_bridge_hostile_title_projection_artifacts, ForgeQueryPublishedDerivedArtifactHandle,
    ForgeQueryPublishedProjectionConsumption, ProjectMaterializedFacts,
    ProjectionFactConsumptionAttempt,
};
use forge_query::{
    ForgeQueryPublicBridgeProjectionConsumptionEvidence,
    ForgeQueryPublicBridgePublishedProjectionReader as SealedPublicBridgeProjectionReader,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub struct PublicBridgePublishedProjectionReader<'a> {
    reader: SealedPublicBridgeProjectionReader<'a>,
}

impl<'a> PublicBridgePublishedProjectionReader<'a> {
    pub fn new(artifact: &'a ForgeQueryPublishedDerivedArtifactHandle) -> Self {
        Self {
            reader: SealedPublicBridgeProjectionReader::from_published_artifact(artifact),
        }
    }

    pub fn consume_title(
        &self,
        invocations: &Arc<AtomicUsize>,
    ) -> ForgeQueryPublicBridgeProjectionConsumptionEvidence {
        let before = invocations.load(Ordering::SeqCst);
        let (result_shape, authorized_projection) =
            public_bridge_hostile_title_projection_artifacts();
        let attempt = self
            .reader
            .consume_projection_facts(
                &result_shape,
                &authorized_projection,
                ProjectMaterializedFacts::declare().display_field("title.value"),
            )
            .expect("public bridge reader lane should consume typed projection facts");
        let evidence = read_evidence_from_attempt(attempt);
        let after = invocations.load(Ordering::SeqCst);
        assert_eq!(after, before, "reader path must not trigger reevaluation");
        evidence
    }
}

fn read_evidence_from_attempt(
    attempt: ForgeQueryPublishedProjectionConsumption,
) -> ForgeQueryPublicBridgeProjectionConsumptionEvidence {
    match attempt {
        ForgeQueryPublishedProjectionConsumption::Current(
            ProjectionFactConsumptionAttempt::Admitted(completed),
        ) => {
            let facts = completed.facts();
            let receipt = facts.issue_receipt();
            ForgeQueryPublicBridgeProjectionConsumptionEvidence::new(
                facts
                    .display_fields()
                    .first()
                    .and_then(|fact| fact.value().as_str())
                    .expect("projection reader lane should expose title.value")
                    .to_string(),
                receipt.receipt_digest(),
                receipt.fact_set_digest(),
                receipt.source_identity(),
                completed.extracted_fact_count(),
                "title.value",
            )
        }
        other => panic!("unexpected public bridge projection consumption posture: {other:?}"),
    }
}
