use crate::harness::certification::digest_parts;
use crate::runtime::{
    ForgeQueryHandleContract, ForgeQueryRuntimeFamilySupportStatus,
    ForgeQueryRuntimePublicApiContract, ForgeQueryRuntimePublicApiNamingContract,
    ForgeQueryRuntimePublicSupportMatrix, ForgeQueryRuntimeSupportProfile,
};

use super::RuntimeApiStabilizationCertificationMatrix;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeApiStabilizationCloseout {
    pub stable_runtime_surfaces: Vec<String>,
    pub deferred_runtime_surfaces: Vec<String>,
    pub unsupported_runtime_surfaces: Vec<String>,
    pub alternate_names: Vec<String>,
    pub safe_to_build_now: Vec<String>,
    pub must_not_assume_yet: Vec<String>,
    pub migration_guidance: Vec<String>,
    pub required_verification_commands: Vec<String>,
    pub golden_transcript_count: usize,
    pub hostile_rejection_count: usize,
    pub lower_runtime_plumbing_count: usize,
    pub closeout_self_check_answers: Vec<String>,
    pub closeout_digest: String,
}

impl RuntimeApiStabilizationCloseout {
    pub(super) fn from_matrix(matrix: &RuntimeApiStabilizationCertificationMatrix) -> Self {
        let support_contract = ForgeQueryRuntimePublicApiContract::from_support_profile(
            &ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
        );
        let support_matrix =
            ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&support_contract);
        let handle_contract = ForgeQueryHandleContract::from_public_api_contract(&support_contract);
        let naming_contract = ForgeQueryRuntimePublicApiNamingContract::standard();

        let stable_runtime_surfaces = support_matrix
            .rows()
            .iter()
            .filter(|row| row.status() == ForgeQueryRuntimeFamilySupportStatus::Supported)
            .map(|row| row.surface().to_string())
            .collect::<Vec<_>>();
        let deferred_runtime_surfaces = support_matrix
            .rows()
            .iter()
            .filter(|row| row.status() == ForgeQueryRuntimeFamilySupportStatus::DeferredDebt)
            .map(|row| format!("{}:{}", row.surface(), row.owner_milestone()))
            .collect::<Vec<_>>();
        let unsupported_runtime_surfaces = support_matrix
            .rows()
            .iter()
            .filter(|row| row.status() == ForgeQueryRuntimeFamilySupportStatus::Unsupported)
            .map(|row| row.surface().to_string())
            .collect::<Vec<_>>();
        let alternate_names = naming_contract
            .rows()
            .iter()
            .flat_map(|row| {
                row.alternate_names()
                    .iter()
                    .map(|name| format!("{}=>{}", name, row.preferred_name()))
            })
            .collect::<Vec<_>>();
        let safe_to_build_now = vec![
            "workspace-scoped live_view/computed/effect declarations through the public facade"
                .to_string(),
            "runtime-backed handles as durable inspectable app surfaces".to_string(),
            "sync read/observe/materialize/state/inspect access for retained handles".to_string(),
            "branch and preview sessions with explicit effect policy and residue inspection"
                .to_string(),
            "typed early support admission via public support matrix rows".to_string(),
        ];
        let must_not_assume_yet = vec![
            "store-backed parity or durable restart/reload semantics are admitted".to_string(),
            "domain-specific geometry/workflow/table semantics live inside forge-query".to_string(),
        ];
        let migration_guidance = vec![
            "enter through ForgeQueryRuntime::workspace and keep downstream APIs handle-shaped"
                .to_string(),
            "prefer canonical names from the naming contract; keep alternate names as adapters"
                .to_string(),
            "use workspace.public_support_matrix() for family admission and workspace.public_mutation_surface_report() for lower-level mutation posture"
                .to_string(),
            "use aspects, lanes, state snapshots, support rows, and inspect output as the extension points"
                .to_string(),
            "extend existing handle/state/inspection contracts in 9.4 rather than adding sibling facades"
                .to_string(),
        ];
        let required_verification_commands = vec![
            "cargo fmt -p forge-query".to_string(),
            "cargo check -p forge-query --tests".to_string(),
            "cargo test --manifest-path crates/forge-query/Cargo.toml --test phase_boundaries_compile_fail".to_string(),
            "cargo test -p forge-query".to_string(),
            "cargo test -p forge-query runtime_api_stabilization".to_string(),
            "cargo test -p forge-query runtime_public_support".to_string(),
            "git diff --check".to_string(),
        ];
        let golden_transcript_count = matrix.rows.len();
        let hostile_rejection_count = matrix.rejection_rows.len();
        let lower_runtime_plumbing_count = matrix
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .map(|lane| lane.lower_runtime_plumbing_count)
            .sum();
        let closeout_self_check_answers = vec![
            format!(
                "golden transcripts are executable through the public facade: {} rows",
                golden_transcript_count
            ),
            format!(
                "support-gated future neighbors fail typed and early: {} rejection rows",
                hostile_rejection_count
            ),
            format!(
                "ordinary DX uses no lower-runtime plumbing: {} lower-runtime calls",
                lower_runtime_plumbing_count
            ),
            format!(
                "support metadata is synchronized with admission gates: {} fail-closed rows",
                support_matrix.fail_closed_row_count()
            ),
            format!(
                "handle/state/inspection contract is extension-ready: {} handle rows",
                handle_contract.rows().len()
            ),
            "store/durable behavior remains explicitly deferred".to_string(),
            "downstream examples are pressure tests, not forge-query domain semantics".to_string(),
        ];
        let mut parts = vec![
            "runtime_api_stabilization_closeout_v1".to_string(),
            format!("certification_rows:{golden_transcript_count}"),
            format!("rejection_rows:{hostile_rejection_count}"),
            format!("lower_runtime_plumbing:{lower_runtime_plumbing_count}"),
            format!("support:{}", support_matrix.matrix_digest()),
            format!("handles:{}", handle_contract.contract_digest()),
            format!("naming:{}", naming_contract.contract_digest()),
        ];
        parts.extend(
            stable_runtime_surfaces
                .iter()
                .map(|item| format!("stable:{item}")),
        );
        parts.extend(
            deferred_runtime_surfaces
                .iter()
                .map(|item| format!("deferred:{item}")),
        );
        parts.extend(
            unsupported_runtime_surfaces
                .iter()
                .map(|item| format!("unsupported:{item}")),
        );
        parts.extend(
            closeout_self_check_answers
                .iter()
                .map(|item| format!("self_check:{item}")),
        );
        let closeout_digest = digest_parts(&parts);

        Self {
            stable_runtime_surfaces,
            deferred_runtime_surfaces,
            unsupported_runtime_surfaces,
            alternate_names,
            safe_to_build_now,
            must_not_assume_yet,
            migration_guidance,
            required_verification_commands,
            golden_transcript_count,
            hostile_rejection_count,
            lower_runtime_plumbing_count,
            closeout_self_check_answers,
            closeout_digest,
        }
    }
}
