use crate::facade::{
    admit_historical_evaluation_path, resolve_historical_materialization_path,
    HistoricalCapabilityDescriptor, HistoricalEvaluationRequest,
    HistoricalMaterializationDescriptor, HistoricalPathResolved, HistoricalPathReuseDescriptor,
    ResolvedHistoricalPathClass,
};

pub(crate) fn retained_resolved(basis: &str) -> HistoricalPathResolved {
    let request = HistoricalEvaluationRequest::retained_snapshot(
        basis.to_string(),
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::new(
        basis.to_string(),
        Some(crate::historical::AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath),
        false,
        false,
        true,
        false,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("admission should succeed");
    resolve_historical_materialization_path(
        admission,
        HistoricalMaterializationDescriptor::new(
            basis.to_string(),
            ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath,
        ),
    )
    .expect("resolution should succeed")
}

pub(crate) fn replay_resolved(basis: &str) -> HistoricalPathResolved {
    let request = HistoricalEvaluationRequest::delta_replay(
        basis.to_string(),
        4,
        8,
        HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::new(
        basis.to_string(),
        Some(crate::historical::AdmittedHistoricalPathClass::AdmittedDeltaReplayPath),
        true,
        false,
        false,
        true,
        HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("admission should succeed");
    resolve_historical_materialization_path(
        admission,
        HistoricalMaterializationDescriptor::new(
            basis.to_string(),
            ResolvedHistoricalPathClass::ResolvedDeltaReplayPath,
        ),
    )
    .expect("resolution should succeed")
}

pub(crate) fn reconstruction_resolved(basis: &str) -> HistoricalPathResolved {
    let request = HistoricalEvaluationRequest::full_reconstruction(
        basis.to_string(),
        4,
        8,
        HistoricalPathReuseDescriptor::no_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::new(
        basis.to_string(),
        Some(crate::historical::AdmittedHistoricalPathClass::AdmittedFullReconstructionPath),
        true,
        false,
        false,
        true,
        HistoricalPathReuseDescriptor::no_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("admission should succeed");
    resolve_historical_materialization_path(
        admission,
        HistoricalMaterializationDescriptor::new(
            basis.to_string(),
            ResolvedHistoricalPathClass::ResolvedFullReconstructionPath,
        ),
    )
    .expect("resolution should succeed")
}
