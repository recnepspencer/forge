use super::registry::{AUTHORED_VALUE_EXPORTS, MUTATION_ASPECT};
use super::registry_helpers::{coarse, proof, unvalidated};
use super::WorthQueryNativeValueAuthorityRow as Row;

pub(super) const ROWS: &[Row] = &[
    coarse(
        "DeclarativeWritebackValue",
        "src/declarative_live.rs",
        &["src/facade/exports_foundation.rs"],
        &["declarative live writeback"],
        "phase-26-native-mutation-authoring",
    ),
    coarse(
        "WorthQueryAdmittedAspectValueTemplate",
        "src/program.rs",
        &[],
        &["program mutation templates"],
        "phase-26-native-mutation-authoring",
    ),
    coarse(
        "WorthQueryWriteCommandTemplate",
        "src/program.rs",
        &["src/facade/exports_policy.rs"],
        &["program mutation templates"],
        "phase-26-native-mutation-authoring",
    ),
    coarse(
        "WorthQueryAspectMutationBuilder",
        MUTATION_ASPECT,
        AUTHORED_VALUE_EXPORTS,
        &["ordinary mutation authoring"],
        "phase-26-native-mutation-authoring",
    ),
    proof(
        "WorthQueryParsedDesiredAspect",
        "src/runtime/mutation/native_intent/proof_states.rs",
        &[],
        &["parsed mutation intent"],
        "phase-26-contract-validated-successor",
    ),
    unvalidated(
        "WorthQueryExistingTruthProbeField",
        "src/runtime/mutation/probe.rs",
        &["src/facade/exports_runtime.rs"],
        &["existing-truth mutation probes"],
        "phase-26-contract-validated-successor",
    ),
    coarse(
        "WorthQueryWriteCommand",
        "src/runtime/surface/mutation/command.rs",
        &["src/facade/exports_runtime_core.rs"],
        &["lower-level mutation command"],
        "phase-26-native-mutation-authoring",
    ),
];
