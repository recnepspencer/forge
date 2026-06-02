use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::query::{
    prepare_primitive_construction_query_no_local_runtime_workaround_audit,
    PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit,
};

const TOPOLOGY_CARGO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-topo/Cargo.toml"
));
const SPATIAL_CARGO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/Cargo.toml"
));
const SPATIAL_LIB: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/src/lib.rs"
));
const SPATIAL_STRUCTURE_GUARD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/src/structure_guard.rs"
));
const TOPOLOGY_STRUCTURE_GUARD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-topo/src/certification/structure_guard.rs"
));
const TOPOLOGY_PUBLIC_API: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-topo/src/certification/public_facade_contracts/contracts/public_api.rs"
));
const TOPOLOGY_BOUNDARY_TESTS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-topo/src/construction/boundary_tests.rs"
));
const KERNEL_ADMITTED_SCAFFOLD_ROOT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/phase_chain/admitted_scaffold/mod.rs"
));
const KERNEL_BIRTH_INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/phase_chain/admitted_scaffold/birth_input.rs"
));
const KERNEL_FAMILY_BIRTH_INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/phase_chain/admitted_scaffold/family_birth_input/mod.rs"
));
const KERNEL_BIRTH_SCAFFOLD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/phase_chain/admitted_scaffold/family_birth_input/birth_scaffold.rs"
));
const KERNEL_TOPOLOGY_READY_BIRTH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/phase_chain/admitted_scaffold/topology_ready_birth.rs"
));
const KERNEL_FACADE_ROOT: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/facade.rs"));
const KERNEL_FACADE_OUTCOME: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/facade/outcome.rs"
));
const KERNEL_FACADE_PRELUDE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/facade/prelude.rs"
));
const KERNEL_AUTHORING: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/authoring.rs"
));

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPhaseFiveBoundaryCloseoutKind {
    TopologyRejectsSpatialDependency,
    SpatialRejectsKernelDependency,
    SynopsisOwnedAdmittedHandoffPrecedent,
    KernelConsumesSynopsisOwnedAdmittedHandoff,
    PublicQuerylessHappyPathQuarantined,
    QueryRuntimeAuthoringHonesty,
    FamilyBirthInputBoundaryLocalized,
    TopologyReadyBirthBoundaryLocalized,
}

impl PrimitiveConstructionPhaseFiveBoundaryCloseoutKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TopologyRejectsSpatialDependency => "topology_rejects_spatial_dependency",
            Self::SpatialRejectsKernelDependency => "spatial_rejects_kernel_dependency",
            Self::SynopsisOwnedAdmittedHandoffPrecedent => {
                "synopsis_owned_admitted_handoff_precedent"
            }
            Self::KernelConsumesSynopsisOwnedAdmittedHandoff => {
                "kernel_consumes_synopsis_owned_admitted_handoff"
            }
            Self::PublicQuerylessHappyPathQuarantined => "public_queryless_happy_path_quarantined",
            Self::QueryRuntimeAuthoringHonesty => "query_runtime_authoring_honesty",
            Self::FamilyBirthInputBoundaryLocalized => "family_birth_input_boundary_localized",
            Self::TopologyReadyBirthBoundaryLocalized => "topology_ready_birth_boundary_localized",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionPhaseFiveBoundaryCloseoutRow {
    kind: PrimitiveConstructionPhaseFiveBoundaryCloseoutKind,
    verified: bool,
    evidence: Vec<String>,
    evidence_digest: String,
}

impl PrimitiveConstructionPhaseFiveBoundaryCloseoutRow {
    fn new(
        kind: PrimitiveConstructionPhaseFiveBoundaryCloseoutKind,
        verified: bool,
        evidence: Vec<String>,
    ) -> Self {
        let mut parts = vec![kind.as_str().to_string(), verified.to_string()];
        parts.extend(evidence.iter().cloned());
        let evidence_digest =
            digest_owned_parts_with_scope(ConstructionDigestScope::ArtifactIdentity, &parts);
        Self {
            kind,
            verified,
            evidence,
            evidence_digest,
        }
    }

    pub fn kind(&self) -> PrimitiveConstructionPhaseFiveBoundaryCloseoutKind {
        self.kind
    }

    pub fn verified(&self) -> bool {
        self.verified
    }

    pub fn evidence(&self) -> &[String] {
        &self.evidence
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionPhaseFiveBoundaryCloseoutReport {
    rows: Vec<PrimitiveConstructionPhaseFiveBoundaryCloseoutRow>,
    query_runtime_audit: PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit,
    closeout_gate_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPhaseFiveBoundaryCloseoutReport {
    pub fn rows(&self) -> &[PrimitiveConstructionPhaseFiveBoundaryCloseoutRow] {
        &self.rows
    }

    pub fn row_for(
        &self,
        kind: PrimitiveConstructionPhaseFiveBoundaryCloseoutKind,
    ) -> Option<&PrimitiveConstructionPhaseFiveBoundaryCloseoutRow> {
        self.rows.iter().find(|row| row.kind() == kind)
    }

    pub fn query_runtime_audit(&self) -> &PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit {
        &self.query_runtime_audit
    }

    pub fn closeout_gate_verified(&self) -> bool {
        self.closeout_gate_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_phase_five_boundary_closeout_report(
) -> PrimitiveConstructionPhaseFiveBoundaryCloseoutReport {
    let query_runtime_audit =
        prepare_primitive_construction_query_no_local_runtime_workaround_audit();
    let rows = vec![
        topology_rejects_spatial_dependency_row(),
        spatial_rejects_kernel_dependency_row(),
        synopsis_owned_admitted_handoff_precedent_row(),
        kernel_consumes_synopsis_owned_admitted_handoff_row(),
        public_queryless_happy_path_quarantined_row(),
        query_runtime_authoring_honesty_row(&query_runtime_audit),
        family_birth_input_boundary_localized_row(),
        topology_ready_birth_boundary_localized_row(),
    ];
    let closeout_gate_verified = rows.iter().all(|row| row.verified());
    let mut parts = vec![
        closeout_gate_verified.to_string(),
        query_runtime_audit.report_digest().to_string(),
    ];
    parts.extend(rows.iter().map(|row| row.evidence_digest().to_string()));
    let report_digest =
        digest_owned_parts_with_scope(ConstructionDigestScope::ArtifactIdentity, &parts);
    PrimitiveConstructionPhaseFiveBoundaryCloseoutReport {
        rows,
        query_runtime_audit,
        closeout_gate_verified,
        report_digest,
    }
}

fn topology_rejects_spatial_dependency_row() -> PrimitiveConstructionPhaseFiveBoundaryCloseoutRow {
    let verified = !TOPOLOGY_CARGO.contains("worth-spatial.workspace = true")
        && !TOPOLOGY_CARGO.contains("worth-geom.workspace = true")
        && TOPOLOGY_STRUCTURE_GUARD.contains("\"worth-spatial\"")
        && TOPOLOGY_STRUCTURE_GUARD.contains("\"worth-geom\"");
    PrimitiveConstructionPhaseFiveBoundaryCloseoutRow::new(
        PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::TopologyRejectsSpatialDependency,
        verified,
        vec![
            "worth-topo.Cargo.toml: no worth-spatial or worth-geom production dependency"
                .to_string(),
            "worth-topo.structure_guard: rejects worth-spatial and worth-geom".to_string(),
        ],
    )
}

fn synopsis_owned_admitted_handoff_precedent_row(
) -> PrimitiveConstructionPhaseFiveBoundaryCloseoutRow {
    let verified = TOPOLOGY_PUBLIC_API
        .contains("prepare_primitive_construction_query_admitted_handoff_from_synopsis")
        && TOPOLOGY_BOUNDARY_TESTS
            .contains("prepare_primitive_construction_query_admitted_handoff_from_synopsis");
    PrimitiveConstructionPhaseFiveBoundaryCloseoutRow::new(
        PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::SynopsisOwnedAdmittedHandoffPrecedent,
        verified,
        vec![
            "worth-topo.public_api: certifies synopsis-owned admitted-handoff seam".to_string(),
            "worth-topo.boundary_tests: guards synopsis-owned admitted-handoff seam".to_string(),
        ],
    )
}

fn spatial_rejects_kernel_dependency_row() -> PrimitiveConstructionPhaseFiveBoundaryCloseoutRow {
    let verified = !SPATIAL_CARGO.contains("worth-kernel")
        && !SPATIAL_CARGO.contains("worth_kernel")
        && SPATIAL_LIB.contains("mod structure_guard;")
        && SPATIAL_STRUCTURE_GUARD.contains("worth-kernel")
        && SPATIAL_STRUCTURE_GUARD.contains("worth_kernel::");
    PrimitiveConstructionPhaseFiveBoundaryCloseoutRow::new(
        PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::SpatialRejectsKernelDependency,
        verified,
        vec![
            "worth-spatial.Cargo.toml: no worth-kernel dependency".to_string(),
            "worth-spatial.structure_guard: rejects worth-kernel dependency and imports"
                .to_string(),
        ],
    )
}

fn kernel_consumes_synopsis_owned_admitted_handoff_row(
) -> PrimitiveConstructionPhaseFiveBoundaryCloseoutRow {
    let verified = KERNEL_ADMITTED_SCAFFOLD_ROOT
        .contains("prepare_primitive_construction_topology_ready_birth(")
        && KERNEL_TOPOLOGY_READY_BIRTH
            .contains("prepare_primitive_construction_query_admitted_handoff_from_synopsis(")
        && !KERNEL_TOPOLOGY_READY_BIRTH.contains("prepare_primitive_construction_query_handoff(");
    PrimitiveConstructionPhaseFiveBoundaryCloseoutRow::new(
        PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::KernelConsumesSynopsisOwnedAdmittedHandoff,
        verified,
        vec![
            "worth-kernel.admitted_scaffold: delegates admitted-handoff consumption through topology-ready birth"
                .to_string(),
            "worth-kernel.topology_ready_birth: uses synopsis-owned admitted-handoff helper without raw topology handoff sequencing"
                .to_string(),
        ],
    )
}

fn public_queryless_happy_path_quarantined_row() -> PrimitiveConstructionPhaseFiveBoundaryCloseoutRow
{
    let verified = !KERNEL_FACADE_ROOT.contains("prepare_primitive_construction_result")
        && !KERNEL_FACADE_ROOT.contains("prepare_primitive_construction_outcome")
        && !KERNEL_FACADE_OUTCOME.contains("prepare_primitive_construction_result")
        && !KERNEL_FACADE_OUTCOME.contains("prepare_primitive_construction_outcome")
        && !KERNEL_FACADE_PRELUDE.contains("prepare_primitive_construction_result")
        && !KERNEL_FACADE_PRELUDE.contains("prepare_primitive_construction_outcome");
    PrimitiveConstructionPhaseFiveBoundaryCloseoutRow::new(
        PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::PublicQuerylessHappyPathQuarantined,
        verified,
        vec![
            "worth-kernel.facade: no public queryless happy-path helpers".to_string(),
            "worth-kernel.facade.prelude/outcome: no queryless happy-path exports".to_string(),
        ],
    )
}

fn query_runtime_authoring_honesty_row(
    query_runtime_audit: &PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit,
) -> PrimitiveConstructionPhaseFiveBoundaryCloseoutRow {
    let verified = query_runtime_audit.violation_count() == 0
        && KERNEL_AUTHORING.contains("fn prepare_result<")
        && KERNEL_AUTHORING.contains("fn prepare_outcome<")
        && KERNEL_AUTHORING.contains("ForgeQueryWorkspace");
    PrimitiveConstructionPhaseFiveBoundaryCloseoutRow::new(
        PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::QueryRuntimeAuthoringHonesty,
        verified,
        vec![
            format!(
                "query runtime workaround audit violations: {}",
                query_runtime_audit.violation_count()
            ),
            "worth-kernel.authoring: session exposes the query-backed construction entry lane"
                .to_string(),
        ],
    )
}

fn family_birth_input_boundary_localized_row() -> PrimitiveConstructionPhaseFiveBoundaryCloseoutRow
{
    let verified = KERNEL_BIRTH_INPUT.contains("build_family_birth_input(")
        && !KERNEL_BIRTH_INPUT.contains("match request.geometry()")
        && KERNEL_FAMILY_BIRTH_INPUT.contains("match request.geometry()")
        && KERNEL_BIRTH_SCAFFOLD.contains("PrimitiveConstructionBirthScaffoldPlan");
    PrimitiveConstructionPhaseFiveBoundaryCloseoutRow::new(
        PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::FamilyBirthInputBoundaryLocalized,
        verified,
        vec![
            "birth_input.rs: delegates to family-owned birth-input lane".to_string(),
            "family_birth_input/mod.rs: owns geometry-family dispatch".to_string(),
            "family_birth_input/birth_scaffold.rs: owns shared birth-scaffold plan lowering"
                .to_string(),
        ],
    )
}

fn topology_ready_birth_boundary_localized_row() -> PrimitiveConstructionPhaseFiveBoundaryCloseoutRow
{
    let verified = KERNEL_ADMITTED_SCAFFOLD_ROOT
        .contains("prepare_primitive_construction_topology_ready_birth(")
        && !KERNEL_ADMITTED_SCAFFOLD_ROOT.contains("plan_primitive_construction_birth(")
        && !KERNEL_ADMITTED_SCAFFOLD_ROOT
            .contains("TopologyPrimitiveConstructionQueryBirthSynopsis::new(")
        && !KERNEL_ADMITTED_SCAFFOLD_ROOT.contains("topology_family_from_spatial_family(")
        && KERNEL_TOPOLOGY_READY_BIRTH.contains("plan_primitive_construction_birth(")
        && KERNEL_TOPOLOGY_READY_BIRTH
            .contains("TopologyPrimitiveConstructionQueryBirthSynopsis::new(")
        && KERNEL_TOPOLOGY_READY_BIRTH.contains("topology_family_from_spatial_family(");
    PrimitiveConstructionPhaseFiveBoundaryCloseoutRow::new(
        PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::TopologyReadyBirthBoundaryLocalized,
        verified,
        vec![
            "admitted_scaffold/mod.rs: delegates post-birth topology bridge".to_string(),
            "topology_ready_birth.rs: owns birth-plan, synopsis, and admitted-handoff bridge"
                .to_string(),
        ],
    )
}
