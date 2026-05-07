use crate::identity::hash_parts;
use forge_runtime_bridge::facade::{
    BridgeAuthoritativeMutationEvidenceCloseout, BridgeAuthoritativeMutationEvidenceSupport,
};

use super::authoritative_mutation_evidence_bridge_alignment::assert_bridge_support_alignment;
use super::{
    ForgeQueryAuthoritativeMutationEvidenceSupport, ForgeQueryMutationSurfaceReport,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimePublicApiNamingContract,
    ForgeQueryRuntimePublicSupportMatrix,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationEvidenceCloseout {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    support_matrix_digest: String,
    mutation_surface_digest: String,
    naming_contract_digest: String,
    query_support_digest: String,
    bridge_support_digest: String,
    bridge_closeout_digest: String,
    safe_to_build_now: Vec<String>,
    must_not_assume_yet: Vec<String>,
    migration_guidance: Vec<String>,
    required_verification_commands: Vec<String>,
    closeout_digest: String,
}

impl ForgeQueryAuthoritativeMutationEvidenceCloseout {
    pub fn derive(
        backend_posture: ForgeQueryRuntimeBackendPosture,
        support_matrix: &ForgeQueryRuntimePublicSupportMatrix,
        mutation_surface: &ForgeQueryMutationSurfaceReport,
        naming_contract: &ForgeQueryRuntimePublicApiNamingContract,
        query_support: &ForgeQueryAuthoritativeMutationEvidenceSupport,
        bridge_support: &BridgeAuthoritativeMutationEvidenceSupport,
        bridge_closeout: &BridgeAuthoritativeMutationEvidenceCloseout,
    ) -> Self {
        assert_bridge_support_alignment(query_support, bridge_support, bridge_closeout);
        let bridge_support_digest = bridge_support.support_digest().to_string();
        let bridge_closeout_digest = bridge_closeout.closeout_digest().to_string();
        let safe_to_build_now = vec![
            "workspace.insert/update/delete/batch receipts preserve declared-versus-resolved target evidence together with touched-aspect fallout".to_string(),
            "existing-truth binding, same-batch symbolic target reference, same-batch symbolic aspect reference, naming mutation, and continuity mutation evidence are part of the ordinary public receipt and inspection story".to_string(),
            "graph composition receipts and inspection now expose explicit symbolic-to-resolved mapping instead of forcing downstream domains to infer same-batch edge resolution from final rows alone".to_string(),
            "graph composition receipts and inspection now expose explicit entity-versus-relation breadth counters instead of forcing downstream domains to reconstruct graph breadth from generic batch rows".to_string(),
            "graph composition receipts and inspection now expose an explicit canonical lowered program ordering instead of forcing downstream domains to infer component meaning from generic batch families".to_string(),
            "graph composition receipts and inspection now expose explicit lifecycle outcome snapshots instead of forcing downstream domains to infer create-versus-update-versus-retire meaning from step kinds alone".to_string(),
            "graph composition now admits symbolic entity follow-up mutation and symbolic relation retirement as ordinary typed lifecycle steps instead of forcing downstream domains back onto scalar batch escape hatches".to_string(),
            "graph composition now admits existing-target update and retirement steps inside the same canonical program instead of forcing mixed created/existing workflows back onto generic batch orchestration".to_string(),
            "graph composition now admits existing-target retarget steps as explicit identity-preserved lifecycle lanes instead of flattening successor rewires back into generic update semantics".to_string(),
            "graph composition now admits existing-target supersession steps as explicit lineage-preserved lifecycle lanes instead of flattening split-or-merge successor semantics into retarget or retirement folklore".to_string(),
            "graph composition now admits bridge-verified existing-target update and retirement steps inside the same canonical program instead of forcing verified mixed-shape workflows back out into separate batch orchestration".to_string(),
            "graph composition now admits bridge-verified existing-target retarget steps inside the same canonical program instead of making verified relation rewires fall back to generic update folklore".to_string(),
            "graph composition now admits bridge-verified existing-target supersession steps inside the same canonical program instead of making lineage-preserved verified rewrites masquerade as plain updates".to_string(),
            "graph composition declaration and symbolic-edge failures now deny through a typed graph-composition lane instead of collapsing into generic workspace strings".to_string(),
            "graph composition denied paths now expose admission traces with explicit failure stages instead of forcing callers to infer where admission stopped from denial prose alone".to_string(),
            "graph composition invariant-pack rejection now denies through a distinct domain-invariant lane instead of collapsing domain invalidity into generic graph-composition support denial".to_string(),
            "graph composition domain-invariant denials now expose attempted-shape summaries with declared collections, declared symbols, capability families, and lifecycle families instead of forcing kernels to reconstruct rejected topology from builder folklore".to_string(),
            "graph composition support is now machine-readable by capability class and extension-hook boundary instead of forcing downstream domains to treat one flat family list as the whole support contract".to_string(),
            "verified graph composition lanes now expose aggregate assumption snapshot, verified precondition, and read-set-breadth summaries instead of forcing kernels to reconstruct operation preconditions from component rows one by one".to_string(),
            "lineage-carrying graph composition lanes now expose aggregate prior-versus-successor continuity summaries instead of forcing kernels to reconstruct edge-split lineage from scattered component continuity rows".to_string(),
            "existing-truth assertions now distinguish retained authoritative assertions from backend-verified assertions on the public receipt and inspection surface".to_string(),
            "backend-verified existing-truth lanes now expose verified assumption-set, assumption snapshot token/digest, verified precondition digest, and read-set-breadth evidence instead of collapsing all verification meaning into one opaque assertion digest".to_string(),
            "mixed existing-truth authority sessions now preserve aggregate mode evidence that distinguishes retained assertions, backend-verified assertions, verified updates, and verified deletes without reconstructing that story from component receipts".to_string(),
            "existing-truth probes now expose a typed backend-verified probe lane for current authoritative values without smuggling that truth through mutation receipts".to_string(),
            "existing-truth verified updates now expose a typed backend-verified update lane that proves current authoritative values before applying update-family mutation receipts".to_string(),
            "existing-target relation updates on admitted families preserve authoritative relation identity instead of disguising delete-plus-recreate as update vocabulary".to_string(),
            "existing-truth verified deletes now expose a typed backend-verified delete lane that proves current authoritative values before applying delete-family mutation receipts".to_string(),
            "existing-truth batch receipts, scalar inspection, and probe surfaces keep retained assertions, backend-verified assertions, backend-verified probes, verified updates, and verified deletes semantically distinct under mixed authority sessions".to_string(),
            "primary multi-command batches now commit atomically at the backend boundary instead of degrading into per-command commits, so invariant-complete closures can rely on one truth-change boundary".to_string(),
            "batch and import-style authority sessions preserve aggregate existing-binding, symbolic-target, symbolic-aspect, naming, continuity, causality, and provenance digests".to_string(),
            "bridge-backed verified-existing support rows are machine-readable by operation family and target-binding family instead of hiding behind one generic backend bool".to_string(),
            "downstream domains may rely on Query receipts and inspection instead of rebuilding target-recovery, naming, or continuity explanation glue locally".to_string(),
            "downstream domains may rely on `verify_existing(...)` only when the active backend actually supports backend verification; unsupported backends remain typed and fail-closed".to_string(),
            "downstream domains may rely on `update_existing_verified(...)` only when the active backend actually supports backend verification; unsupported backends remain typed and fail-closed".to_string(),
            "downstream domains may rely on `delete_existing_verified(...)` only when the active backend actually supports backend verification; unsupported backends remain typed and fail-closed".to_string(),
        ];
        let must_not_assume_yet = vec![
            "authority-mutation evidence closes durable restart, temporal, async, or store-backed mutation semantics".to_string(),
            "unsupported identity-binding, naming, or continuity families remain fail-closed until explicitly admitted".to_string(),
            "unsupported existing-truth binding, assertion, verified-mutation, and probe neighbors remain typed and fail-closed rather than degrading into best-effort fallback behavior".to_string(),
            "unsupported identity-preserving relation update families remain fail-closed until the lower runtime can preserve target identity honestly".to_string(),
            "bridge-backed verified-existing support rows that deny on the primary posture may not be treated as production-ready just because the scaffold posture admits them".to_string(),
            "downstream code may bypass Query receipts and inspect raw bridge/runtime provenance bags directly".to_string(),
        ];
        let migration_guidance = vec![
            "move authoritative mutation onto workspace.insert/update/delete/batch and consume receipts plus inspect output as the domain explanation contract".to_string(),
            "read bridge-backed verified-existing support rows before teaching `verify_existing(...)`, `probe_existing(...)`, `update_existing_verified(...)`, or `delete_existing_verified(...)` as ordinary production runtime flows".to_string(),
            "read graph-composition capability rows and extension-hook rows before teaching a new mixed-shape lifecycle or domain extension as ordinary stable runtime support".to_string(),
            "use `workspace.compose_graph(...)` or `workspace.compose_graph_with_invariant_pack(...)` when one logical mutation needs symbolic resolution, verified preconditions, lineage, or domain-invalidity evidence as part of the ordinary receipt story".to_string(),
            "use `workspace.assert_existing(...)` for retained assertion receipts and `workspace.verify_existing(...)` when the backend must prove current stored truth before returning an assertion receipt".to_string(),
            "use `workspace.probe_existing(...)` when the domain needs current authoritative aspect values as input rather than a retained assertion receipt".to_string(),
            "use `workspace.bind_existing_relation(...)` plus `workspace.update_existing(...)` when an admitted relation family must preserve authoritative target identity under ordinary update-family receipts".to_string(),
            "use `workspace.update_existing_verified(...)` when the backend must prove current stored truth immediately before an existing-target update-family mutation".to_string(),
            "use `workspace.delete_existing_verified(...)` when the backend must prove current stored truth immediately before an existing-target delete-family mutation".to_string(),
            "delete local existing-target rebinding, naming outcome reconstruction, and continuity breadcrumb glue once equivalent Query evidence is available".to_string(),
            "delete local graph-program rejection reconstruction once `admission_trace()` and `domain_invariant_summary()` cover the denied-path explanation contract".to_string(),
            "treat unsupported mutation-evidence neighbors as fail-closed support gates rather than alternate runtime seams".to_string(),
        ];
        let required_verification_commands = vec![
            "cargo fmt -p forge-query".to_string(),
            "cargo check -p forge-query --tests".to_string(),
            "cargo test --manifest-path crates/forge-query/Cargo.toml --test phase_boundaries_compile_fail".to_string(),
            "cargo test -p forge-query".to_string(),
            "cargo fmt -p forge-runtime-bridge".to_string(),
            "cargo check -p forge-runtime-bridge --tests".to_string(),
            "cargo test -p forge-runtime-bridge".to_string(),
            "cargo test --manifest-path crates/forge-runtime-bridge/Cargo.toml --test phase_boundaries_compile_fail".to_string(),
            "git diff --check".to_string(),
        ];
        let mut parts = vec![
            "forge_query_authoritative_mutation_evidence_closeout_v1".to_string(),
            format!("posture:{}", backend_posture.as_str()),
            format!("matrix:{}", support_matrix.matrix_digest()),
            format!("mutation:{}", mutation_surface.report_digest()),
            format!("naming:{}", naming_contract.contract_digest()),
            format!("query-support:{}", query_support.support_digest()),
            format!("bridge-support:{bridge_support_digest}"),
            format!("bridge-closeout:{bridge_closeout_digest}"),
        ];
        parts.extend(safe_to_build_now.iter().map(|item| format!("safe:{item}")));
        parts.extend(
            must_not_assume_yet
                .iter()
                .map(|item| format!("deferred:{item}")),
        );
        parts.extend(
            migration_guidance
                .iter()
                .map(|item| format!("migration:{item}")),
        );
        parts.extend(
            required_verification_commands
                .iter()
                .map(|item| format!("verify:{item}")),
        );
        let closeout_digest = hash_parts(&parts);
        Self {
            backend_posture,
            support_matrix_digest: support_matrix.matrix_digest().to_string(),
            mutation_surface_digest: mutation_surface.report_digest().to_string(),
            naming_contract_digest: naming_contract.contract_digest().to_string(),
            query_support_digest: query_support.support_digest().to_string(),
            bridge_support_digest,
            bridge_closeout_digest,
            safe_to_build_now,
            must_not_assume_yet,
            migration_guidance,
            required_verification_commands,
            closeout_digest,
        }
    }

    pub fn backend_posture(&self) -> ForgeQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn mutation_surface_digest(&self) -> &str {
        &self.mutation_surface_digest
    }

    pub fn naming_contract_digest(&self) -> &str {
        &self.naming_contract_digest
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn bridge_support_digest(&self) -> &str {
        &self.bridge_support_digest
    }

    pub fn bridge_closeout_digest(&self) -> &str {
        &self.bridge_closeout_digest
    }

    pub fn safe_to_build_now(&self) -> &[String] {
        &self.safe_to_build_now
    }

    pub fn must_not_assume_yet(&self) -> &[String] {
        &self.must_not_assume_yet
    }

    pub fn migration_guidance(&self) -> &[String] {
        &self.migration_guidance
    }

    pub fn required_verification_commands(&self) -> &[String] {
        &self.required_verification_commands
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}
