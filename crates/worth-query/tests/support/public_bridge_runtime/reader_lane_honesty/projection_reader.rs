use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};
use worth_query::facade::certification::public_bridge_hostile_title_projection_artifacts;
use worth_query::facade::certification::{
    WorthQueryPublicBridgeProjectionConsumptionEvidence,
    WorthQueryPublicBridgePublishedProjectionReader as SealedPublicBridgeProjectionReader,
};
use worth_query::facade::foundation::{ProjectionAuthorityContract, ProjectionFactFieldPath};
use worth_query::facade::runtime::{
    WorthQueryPublishedDerivedArtifactHandle, WorthQueryPublishedProjectionAuthorityOutcome,
};

pub struct PublicBridgePublishedProjectionReader<'a> {
    reader: SealedPublicBridgeProjectionReader<'a>,
}

impl<'a> PublicBridgePublishedProjectionReader<'a> {
    pub fn new(artifact: &'a WorthQueryPublishedDerivedArtifactHandle) -> Self {
        Self {
            reader: SealedPublicBridgeProjectionReader::from_published_artifact(artifact),
        }
    }

    pub fn consume_title(
        &self,
        invocations: &Arc<AtomicUsize>,
    ) -> WorthQueryPublicBridgeProjectionConsumptionEvidence {
        let before = invocations.load(Ordering::SeqCst);
        let (result_shape, authorized_projection) =
            public_bridge_hostile_title_projection_artifacts();
        let outcome = self
            .reader
            .consume_projection_authority(
                &result_shape,
                &authorized_projection,
                ProjectionAuthorityContract::declare()
                    .require_settled_consumption()
                    .require_source_authority()
                    .require_display_field(title_value_field_path()),
            )
            .expect("public bridge reader lane should consume typed projection authority");
        let evidence = read_evidence_from_outcome(outcome);
        let after = invocations.load(Ordering::SeqCst);
        assert_eq!(after, before, "reader path must not trigger reevaluation");
        evidence
    }
}

fn title_value_field_path() -> ProjectionFactFieldPath {
    ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(vec![
            FieldKey::new("title").expect("test field segment must be valid"),
            FieldKey::new("value").expect("test field segment must be valid"),
        ])
        .expect("test field path must be valid"),
    )
}

fn read_evidence_from_outcome(
    outcome: WorthQueryPublishedProjectionAuthorityOutcome,
) -> WorthQueryPublicBridgeProjectionConsumptionEvidence {
    if let Some(authority) = outcome.authority() {
        let facts = authority.facts();
        let receipt = authority.receipt();
        WorthQueryPublicBridgeProjectionConsumptionEvidence::new(
            facts
                .display_fields()
                .first()
                .and_then(|fact| match fact.native_value().scalar()? {
                    AspectValue::String(value) => match value {
                        worth_foundational::facade::InternedString::Raw(value) => {
                            Some(value.as_str())
                        }
                        worth_foundational::facade::InternedString::Symbol(_) => None,
                    },
                    _ => None,
                })
                .expect("projection reader lane should expose title.value")
                .to_string(),
            receipt.receipt_digest(),
            receipt.fact_set_digest(),
            receipt.source_identity(),
            authority.counters().consumed_fact_visits(),
            "title.value",
        )
    } else {
        panic!("unexpected public bridge projection authority posture: {outcome:?}")
    }
}
