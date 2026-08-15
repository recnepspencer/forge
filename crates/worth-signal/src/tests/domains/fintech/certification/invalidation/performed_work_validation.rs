use std::collections::{BTreeMap, BTreeSet};

use crate::data::error::SignalError;
use crate::tests::domains::fintech::world::{
    FinancialPerformedCanonicalWork, FinancialPerformedWorkOrigin, LocalitySemanticOutputId,
};

use super::locality_expectation::ExpectedSealedOriginBinding;
use super::FinancialLocalityExpectationManifest;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum NormalizedWorkOrigin {
    Source(u64),
    Dependency {
        cause_set_generation: u64,
        producer_commit_ordinals: Vec<u64>,
    },
    Structural(u64),
}

type NormalizedWorkKey = (u64, LocalitySemanticOutputId, u64, u64, u32);
type NormalizedWork = BTreeMap<NormalizedWorkKey, BTreeSet<NormalizedWorkOrigin>>;

pub(super) fn require_expected_performed_work(
    manifest: &FinancialLocalityExpectationManifest,
    performed: &FinancialPerformedCanonicalWork,
) -> Result<(), SignalError> {
    let expected = manifest
        .executed_work()
        .iter()
        .map(|(work, origins)| {
            let key = (
                work.graph.graph_instance,
                work.target,
                work.dependency_revision,
                work.readiness_epoch,
                work.stage_order,
            );
            let origins = origins.iter().map(normalize_expected_origin).collect();
            (key, origins)
        })
        .collect::<NormalizedWork>();
    let mut actual = NormalizedWork::new();
    for (work, count) in performed {
        if *count != 1 {
            return Err(SignalError::internal(format!(
                "performed locality work repeated one exact binding {count} times: {work:?}"
            )));
        }
        let (graph, target, revision, origin, epoch, stage) = work.axes();
        actual
            .entry((graph, target, revision, epoch, stage))
            .or_default()
            .insert(normalize_performed_origin(origin));
    }
    if actual == expected {
        Ok(())
    } else {
        Err(SignalError::internal(format!(
            "performed locality U bindings differ from the independent manifest: actual={actual:?}, expected={expected:?}"
        )))
    }
}

fn normalize_expected_origin(origin: &ExpectedSealedOriginBinding) -> NormalizedWorkOrigin {
    match origin {
        ExpectedSealedOriginBinding::SourceRecompute {
            admission_generation,
        } => NormalizedWorkOrigin::Source(*admission_generation),
        ExpectedSealedOriginBinding::DependencyCommit {
            cause_set_generation,
            producer_commit_ordinals,
        } => NormalizedWorkOrigin::Dependency {
            cause_set_generation: *cause_set_generation,
            producer_commit_ordinals: producer_commit_ordinals.clone(),
        },
        ExpectedSealedOriginBinding::StructuralRecompute {
            structural_generation,
        } => NormalizedWorkOrigin::Structural(*structural_generation),
    }
}

fn normalize_performed_origin(origin: &FinancialPerformedWorkOrigin) -> NormalizedWorkOrigin {
    match origin {
        FinancialPerformedWorkOrigin::SourceAdmission { generation } => {
            NormalizedWorkOrigin::Source(*generation)
        }
        FinancialPerformedWorkOrigin::DependencyCommit {
            cause_set_generation,
            producer_commit_ordinals,
        } => NormalizedWorkOrigin::Dependency {
            cause_set_generation: u64::from(*cause_set_generation),
            producer_commit_ordinals: producer_commit_ordinals.clone(),
        },
        FinancialPerformedWorkOrigin::StructuralMutation { ordinal } => {
            NormalizedWorkOrigin::Structural(*ordinal)
        }
    }
}
