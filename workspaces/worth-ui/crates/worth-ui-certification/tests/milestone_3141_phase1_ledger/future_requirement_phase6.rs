use super::super::requirement_contract::RequirementContract;

macro_rules! contract {
    ($requirement:literal, $owner:literal, $boundary:literal, $world:literal,
     $proof:literal, $authority:literal, $mutation:literal, $counter:literal) => {
        RequirementContract {
            requirement: $requirement,
            owner: $owner,
            boundary: $boundary,
            world: $world,
            proof_kind: $proof,
            authority: $authority,
            mutation_family: $mutation,
            counter_family: $counter,
        }
    };
}

pub(super) const CONTRACTS: &[RequirementContract] = &[
    contract!(
        "P6-PREDECESSOR-01",
        "worth-ui-certification",
        "current Phase 1-5 source handoff",
        "phase-six-ledger-world",
        "operational-revalidation",
        "worth_ui_certification::phase_six_ledger",
        "stale-predecessor",
        "requirements"
    ),
    contract!(
        "P6-INPUT-AFFINITY-01",
        "worth-ui-host-native",
        "native last-completed-presentation affinity and bounded retention",
        "native-lifecycle-protocol-world",
        "runtime-model",
        "worth_ui_host_native::native::input::observation",
        "input-affinity",
        "observations"
    ),
    contract!(
        "P6-IME-01",
        "worth-ui-host-native",
        "native IME composition phases and canonical ranges",
        "native-lifecycle-protocol-world",
        "ime-conformance",
        "worth_ui_host_native::native::input::ime",
        "ime-semantic-phase",
        "ime-phases"
    ),
    contract!(
        "P6-POINTER-TIME-01",
        "worth-ui-host-native",
        "event-time pointer position witness",
        "windows-native-boundary-world",
        "event-time-input",
        "worth_ui_host_native::native::input::windows",
        "pointer-time",
        "pointer-witnesses"
    ),
    contract!(
        "P6-PROFILE-ORDER-01",
        "worth-ui-host-native",
        "event-time profile and resize ordering",
        "native-lifecycle-protocol-world",
        "runtime-model",
        "worth_ui_host_native::native::input::observation",
        "profile-order",
        "profile-transitions"
    ),
    contract!(
        "P6-READINESS-01",
        "worth-ui-host-native",
        "retained-observation readiness delivery",
        "native-lifecycle-protocol-world",
        "lifecycle-model",
        "worth_ui_host_native::native::readiness",
        "readiness-delivery",
        "readiness-generations"
    ),
    contract!(
        "P6-SETTLEMENT-01",
        "worth-ui-runtime",
        "typed native observation ingress settlement",
        "native-lifecycle-protocol-world",
        "integration-model",
        "worth_ui_runtime::facade::entry::native_observation_settlement",
        "typed-settlement",
        "settlement-outcomes"
    ),
    contract!(
        "P6-PROTOCOL-WORLD-01",
        "worth-ui-certification",
        "exhaustive native lifecycle schedule",
        "native-lifecycle-protocol-world",
        "exhaustive-oracle",
        "worth_ui_certification::application_contracts::phase6_native_lifecycle",
        "oracle",
        "protocol-schedules"
    ),
    contract!(
        "P6-WINDOWS-WORLD-01",
        "worth-ui-certification",
        "serialized Windows native input boundary",
        "windows-native-boundary-world",
        "external-world",
        "worth_ui_platform_pulse::native_phase6",
        "pointer-source",
        "pointer-witnesses"
    ),
    contract!(
        "P6-CLOSE-01",
        "worth-ui-certification",
        "phase six final source closure",
        "phase-six-ledger-world",
        "ledger-closure",
        "worth_ui_certification::phase_six_ledger",
        "ledger",
        "requirements"
    ),
];
