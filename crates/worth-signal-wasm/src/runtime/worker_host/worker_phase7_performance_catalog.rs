use super::worker_phase7_performance_contracts::{
    WorkerPhase7BridgeAllocationPosture, WorkerPhase7ComplexityContract,
    WorkerPhase7PerformanceFailureMode,
};

pub(crate) fn required_counter_names() -> Vec<&'static str> {
    vec![
        "workerTransactionSubmissionCount",
        "workerTransactionBatchWidth",
        "workerInvalidationBreadth",
        "workerRecomputationBreadth",
        "hostCapabilityIngressCount",
        "hostCapabilityIngressCoalescedWidth",
        "browserHistoryIngressCount",
        "hostEffectRequestCount",
        "hostEffectCompletionCount",
        "hostEffectDenialOrUnavailableCount",
        "outputDeliveryPacketCount",
        "outputDeliveryBreadth",
        "observationDeliveryPacketCount",
        "observationDeliveryBreadth",
        "diagnosticsSummaryReadCount",
        "diagnosticsRichReadCount",
        "diagnosticsColdReconstructionCount",
        "workerMainThreadRoundTripCount",
        "workerPlacementDenialCount",
        "workerFallbackCount",
        "mainThreadHostedCallbackExecutionCount",
        "replayCapabilityUnavailableCount",
        "restoreCapabilityReattachCount",
        "compatibilityModeParityCheckCount",
        "mainThreadBroadWorkDenialCount",
        "bridgeSerializationAllocationCount",
        "bridgeDeserializationAllocationCount",
    ]
}

pub(crate) fn required_complexity_contracts() -> Vec<WorkerPhase7ComplexityContract> {
    vec![
        contract(
            "transactionSubmissionBridging",
            "cost follows committed mutation batch width and bridged payload breadth",
            &["committedMutationBatchWidth", "bridgedPayloadBreadth"],
        ),
        contract(
            "hostCapabilityIngressAdmission",
            "cost follows changed capability frontier and coalesced update width",
            &["changedCapabilityFrontier", "coalescedUpdateWidth"],
        ),
        contract(
            "browserHistoryIngressAdmission",
            "cost follows typed route event width and admitted continuity artifacts",
            &["routeEventWidth", "continuityArtifactWidth"],
        ),
        contract(
            "hostEffectRequestRouting",
            "cost follows closed host-effect payload breadth and acknowledgement width",
            &[
                "closedHostEffectPayloadBreadth",
                "acknowledgementArtifactWidth",
            ],
        ),
        contract(
            "committedOutputDelivery",
            "cost follows committed public output delivery breadth and payload bytes",
            &["publicOutputDeliveryBreadth", "outputPayloadByteCount"],
        ),
        contract(
            "committedObservationDelivery",
            "cost follows committed observation delivery breadth",
            &["observationDeliveryBreadth"],
        ),
        contract(
            "diagnosticsSummaryReads",
            "cost remains summary lookup only with zero rich reconstruction",
            &["summaryLookup", "zeroRichReconstruction"],
        ),
        contract(
            "diagnosticsRichReads",
            "cost follows requested rich history span and explicit cold-work attribution",
            &["requestedHistorySpan", "coldWorkAttribution"],
        ),
        contract(
            "callbackPlacementClassification",
            "cost follows declaration classification work",
            &["declarationClassificationWork"],
        ),
        contract(
            "workerExecutableDeclarationLowering",
            "cost follows lowerable declaration count and lowered dependency frontier",
            &["lowerableDeclarationCount", "loweredDependencyFrontier"],
        ),
        contract(
            "mainThreadHostedCallbackExecutionRouting",
            "cost follows closed request input breadth and readmitted result width",
            &["closedRequestInputBreadth", "readmittedResultWidth"],
        ),
        contract(
            "replayRestoreCapabilityReconstruction",
            "cost follows retained capability artifacts and historical span",
            &["retainedCapabilityArtifacts", "historicalSpan"],
        ),
        contract(
            "importExportCapabilityClassification",
            "cost follows exported callback artifact count and reattachment width",
            &["exportedCallbackArtifactCount", "reattachmentWidth"],
        ),
        contract(
            "fallbackAndDenialClassification",
            "cost follows explicit denied artifact count and fallback artifact count",
            &["deniedArtifactCount", "fallbackArtifactCount"],
        ),
    ]
}

pub(crate) fn required_failure_modes() -> Vec<WorkerPhase7PerformanceFailureMode> {
    vec![
        failure("BridgeChatterStorm", "per-read or per-node bridge chatter"),
        failure(
            "CompatibilityTruthLeak",
            "compatibility mode owns semantics",
        ),
        failure(
            "MainThreadProjectionInflation",
            "main thread mirrors authority views",
        ),
        failure(
            "PlacementCollapse",
            "one ineligible node pins unrelated graph breadth",
        ),
        failure(
            "CallbackPortabilityLie",
            "live closures are treated as portable data",
        ),
        failure(
            "HistoryCapabilityAmnesia",
            "history preserves values but loses capability posture",
        ),
        failure(
            "UIFreezeBySerialization",
            "broad public delivery blocks the UI thread",
        ),
        failure(
            "AmbientHostReadRelapse",
            "browser facts regain meaning through ambient reads",
        ),
    ]
}

pub(crate) fn required_bridge_allocation_posture() -> WorkerPhase7BridgeAllocationPosture {
    WorkerPhase7BridgeAllocationPosture {
        posture: "explicitBoundaryAllocationAccounting",
        serialization_allocation_counter: "bridgeSerializationAllocationCount",
        deserialization_allocation_counter: "bridgeDeserializationAllocationCount",
        lifecycle_scope: "bridgeEnvelopeLifecycle",
        hidden_allocation_allowed: false,
    }
}

fn contract(
    operation: &'static str,
    contract: &'static str,
    cost_bases: &[&'static str],
) -> WorkerPhase7ComplexityContract {
    WorkerPhase7ComplexityContract {
        operation,
        contract,
        cost_bases: cost_bases.to_vec(),
        forbidden_cost_bases: vec!["totalGraphSize", "ambientMainThreadState"],
    }
}

fn failure(
    mode: &'static str,
    prohibited_behavior: &'static str,
) -> WorkerPhase7PerformanceFailureMode {
    WorkerPhase7PerformanceFailureMode {
        mode,
        prohibited_behavior,
    }
}
