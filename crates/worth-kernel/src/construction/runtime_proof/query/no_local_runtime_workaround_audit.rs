use crate::construction::digest::digest_owned_parts;

const AUDITED_FILES: [(&str, &str); 27] = [
    (
        "worth-kernel.authoring",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/authoring.rs"
        )),
    ),
    (
        "worth-kernel.runtime-basis",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/runtime_basis.rs"
        )),
    ),
    (
        "worth-kernel.motion-branch-runtime",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/motion/branch_runtime.rs"
        )),
    ),
    (
        "worth-kernel.motion-replay",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/motion/replay.rs"
        )),
    ),
    (
        "worth-kernel.arbitration-replay",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/arbitration/replay.rs"
        )),
    ),
    (
        "worth-kernel.continuity-branch-runtime",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/continuity_branch_runtime.rs"
        )),
    ),
    (
        "worth-kernel.continuity-replay",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/continuity_replay.rs"
        )),
    ),
    (
        "worth-kernel.preview-branch-runtime",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/preview_branch_runtime.rs"
        )),
    ),
    (
        "worth-kernel.preview-replay",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/preview_replay.rs"
        )),
    ),
    (
        "worth-kernel.profile-branch-runtime",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/profile_branch_runtime.rs"
        )),
    ),
    (
        "worth-kernel.profile-replay",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/profile_replay.rs"
        )),
    ),
    (
        "worth-kernel.query-basis-preview-parity",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/query/basis_preview_parity.rs"
        )),
    ),
    (
        "worth-kernel.query-continuity",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/query/continuity.rs"
        )),
    ),
    (
        "worth-kernel.query-graph-composition-parity",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/query/graph_composition_parity.rs"
        )),
    ),
    (
        "worth-kernel.query-inspection-parity",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/query/inspection_parity.rs"
        )),
    ),
    (
        "worth-kernel.query-intent-arbitration",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/query/intent_arbitration.rs"
        )),
    ),
    (
        "worth-kernel.query-motion-parity",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/query/motion_parity.rs"
        )),
    ),
    (
        "worth-kernel.query-preview-parity",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/query/preview.rs"
        )),
    ),
    (
        "worth-kernel.query-policy-profile",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/query/profile.rs"
        )),
    ),
    (
        "worth-kernel.query-projection-consumption-receipt",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/query/projection_consumption_receipt.rs"
        )),
    ),
    (
        "worth-kernel.outcome",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/result_surface/outcome.rs"
        )),
    ),
    (
        "worth-spatial.primitive-birth",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-spatial/src/bindings/primitive_birth.rs"
        )),
    ),
    (
        "worth-geom.realization-conditioning",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-geom/src/primitives/shape_realization/conditioning.rs"
        )),
    ),
    (
        "worth-geom.realization-support",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-geom/src/primitives/shape_realization/support.rs"
        )),
    ),
    (
        "worth-topo.construction-boundary-mod",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-topo/src/construction/mod.rs"
        )),
    ),
    (
        "worth-topo.query-native-boundary",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-topo/src/construction/query_native_boundary.rs"
        )),
    ),
    (
        "worth-topo.boundary-tests",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-topo/src/construction/boundary_tests.rs"
        )),
    ),
];

const FORBIDDEN_RUNTIME_PATTERNS: [&str; 9] = [
    ".batch(",
    ".write(",
    "bind_existing_entity(",
    "bind_existing_relation(",
    "update_existing(",
    "verify_existing(",
    "update_existing_verified(",
    "delete_existing(",
    "probe_existing(",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit {
    audited_file_count: usize,
    violation_count: usize,
    report_digest: String,
}

impl PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit {
    pub fn audited_file_count(&self) -> usize {
        self.audited_file_count
    }

    pub fn violation_count(&self) -> usize {
        self.violation_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_query_no_local_runtime_workaround_audit(
) -> PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit {
    let violation_count = AUDITED_FILES
        .iter()
        .flat_map(|(_, source)| {
            FORBIDDEN_RUNTIME_PATTERNS
                .iter()
                .map(|pattern| source.contains(pattern))
        })
        .filter(|found| *found)
        .count();
    let report_digest =
        digest_owned_parts(&[AUDITED_FILES.len().to_string(), violation_count.to_string()]);
    PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit {
        audited_file_count: AUDITED_FILES.len(),
        violation_count,
        report_digest,
    }
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_query_no_local_runtime_workaround_audit;

    const AUDITED_QUERY_READY_ENTRY_FILES: [(&str, &str); 5] = [
        (
            "worth-kernel.runtime-basis",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/construction/runtime_proof/runtime_basis.rs"
            )),
        ),
        (
            "worth-kernel.query-graph-composition-parity",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/construction/runtime_proof/query/graph_composition_parity.rs"
            )),
        ),
        (
            "worth-kernel.query-inspection-parity",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/construction/runtime_proof/query/inspection_parity.rs"
            )),
        ),
        (
            "worth-kernel.query-projection-consumption-receipt",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/construction/runtime_proof/query/projection_consumption_receipt.rs"
            )),
        ),
        (
            "worth-kernel.public-api-construction-contract",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/certification/public_facade_contracts/contracts/public_api_construction.rs"
            )),
        ),
    ];

    const FORBIDDEN_DIRECT_LOCAL_PREPARATION_PATTERNS: [&str; 2] = [
        "prepare_primitive_construction_result(",
        "prepare_primitive_construction_outcome(",
    ];

    #[test]
    fn query_no_local_runtime_workaround_audit_proves_the_current_path_avoids_local_bypasses() {
        let report = prepare_primitive_construction_query_no_local_runtime_workaround_audit();

        assert_eq!(report.audited_file_count(), 27);
        assert_eq!(report.violation_count(), 0);
        assert!(!report.report_digest().is_empty());
    }

    #[test]
    fn query_ready_runtime_surfaces_enter_through_query_authoring_session() {
        let violations = AUDITED_QUERY_READY_ENTRY_FILES
            .iter()
            .flat_map(|(label, source)| {
                FORBIDDEN_DIRECT_LOCAL_PREPARATION_PATTERNS
                    .iter()
                    .filter(move |pattern| source.contains(**pattern))
                    .map(move |pattern| format!("{label}:{pattern}"))
            })
            .collect::<Vec<_>>();

        assert_eq!(
            violations,
            Vec::<String>::new(),
            "workspace-backed query/runtime construction surfaces still call direct local preparation helpers instead of crossing the query authoring-session entry lane: {violations:?}"
        );
    }
}
