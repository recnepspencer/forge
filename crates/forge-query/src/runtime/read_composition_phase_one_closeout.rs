use crate::identity::hash_parts;

use super::{
    ForgeQueryReadCompositionSupportReport, ForgeQueryRuntime, ForgeQueryRuntimeBackendPosture,
    ForgeQueryRuntimePublicSupportMatrix, ForgeQueryRuntimeSupportProfile,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadCompositionPhaseOneCloseout {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    support_matrix_digest: String,
    read_support_digest: String,
    safe_to_build_now: Vec<String>,
    must_not_assume_yet: Vec<String>,
    migration_guidance: Vec<String>,
    required_verification_commands: Vec<String>,
    closeout_digest: String,
}

impl ForgeQueryReadCompositionPhaseOneCloseout {
    pub fn derive(
        backend_posture: ForgeQueryRuntimeBackendPosture,
        support_matrix: &ForgeQueryRuntimePublicSupportMatrix,
        read_support: &ForgeQueryReadCompositionSupportReport,
    ) -> Self {
        let safe_to_build_now = vec![
            "compose_read, compose_read_with_invariant_pack, define_read_family, define_read_family_with_invariant_pack, execute_read_family, and execute_read_family_in_basis_context form one public read-composition product instead of separate helper stories".to_string(),
            "the canonical read artifact is ReadGraph and every admitted execution returns a ReadReceipt with scope class, graph family, breadth, fallback posture, and relationship-proof admission identity".to_string(),
            "the Phase 1 runtime taxonomy now freezes query_runtime_current, query_runtime_historical, and fallback classes as public read-kernel posture".to_string(),
            "scope classes are kernel-owned and freeze local_neighborhood, anchored_expansion, and explicit_broad_search at the shared boundary instead of letting callers relabel the same lowered read".to_string(),
            "operator-owned graph lanes now cover direct_edge, successor_walk, shared_endpoint, shared_attachment, bounded_ancestor, bounded_descendant, anchored_frontier, and frontier_search".to_string(),
            "traversal-bearing reads now admit descriptor-backed synthetic runtime relationship proof before execution instead of reporting only a receipt heuristic".to_string(),
            "invariant packs can narrow an admitted read graph before execution and deny through a typed domain-invariant lane with an attached rejected-graph summary".to_string(),
            "reusable ReadFamily artifacts are part of kernel completeness and keep admission history in their digest so invariant-admitted families do not collapse into plain reusable reads".to_string(),
            "operator-owned builders keep traversal ownership mechanical because the exported operator-builder boundary hides traverse and is compile-fail certified".to_string(),
            "later domain adoption must extend through the frozen read-composition hooks for domain_read_family_lowering, domain_invariant_pack, domain_decoder, and domain_result_certification instead of rebuilding a second local read stack".to_string(),
        ];
        let must_not_assume_yet = vec![
            "do not assume this Phase 1 kernel artifact by itself certifies Worth topology migration; that closure lives in the Worth topology-domain closeout surfaces".to_string(),
            "do not assume this generic gate certifies future non-topology Worth domains; later trim, carrier, NURBS, fillet, and branch-history vocabularies still need domain-owned adoption on top of this kernel".to_string(),
            "do not assume all future domain families already exist; later topology, trim, carrier, NURBS, fillet, and branch-history vocabularies still need domain-owned adoption on top of this kernel".to_string(),
        ];
        let migration_guidance = vec![
            "for new domain adoption, start by moving one bounded read family onto compose_read plus a domain-owned decoded view before widening the family set".to_string(),
            "bind each new domain family through the frozen lowering, invariant-pack, decoder, and certification hook boundaries instead of inventing local extension seams".to_string(),
            "prefer an operator-owned read lane whenever the domain shape matches one of the admitted built-in operators instead of open-coding traverse in the Worth facade".to_string(),
            "for topology snapshot read-only posture, use the admitted historical basis-aware read-family path instead of carrying stale fallback debt wording".to_string(),
            "for later Worth domains, do not resume domain-specific widening until that domain has an aggregate closeout proof naming any remaining fallback consumers as debt rows".to_string(),
        ];
        let required_verification_commands = vec![
            "cargo fmt --package forge-query".to_string(),
            "cargo test -p forge-query runtime::tests::read_composition --quiet".to_string(),
            "cargo test -p forge-query --test phase_boundaries_compile_fail --quiet".to_string(),
            "cargo test -p forge-query --quiet".to_string(),
            "git diff --check".to_string(),
        ];
        let mut parts = vec![
            "forge_query_read_composition_phase_one_closeout_v1".to_string(),
            format!("posture:{}", backend_posture.as_str()),
            format!("matrix:{}", support_matrix.matrix_digest()),
            format!("read-support:{}", read_support.support_digest()),
        ];
        parts.extend(safe_to_build_now.iter().map(|line| format!("safe:{line}")));
        parts.extend(
            must_not_assume_yet
                .iter()
                .map(|line| format!("deferred:{line}")),
        );
        parts.extend(
            migration_guidance
                .iter()
                .map(|line| format!("migration:{line}")),
        );
        parts.extend(
            required_verification_commands
                .iter()
                .map(|line| format!("verify:{line}")),
        );
        let closeout_digest = hash_parts(&parts);
        Self {
            backend_posture,
            support_matrix_digest: support_matrix.matrix_digest().to_string(),
            read_support_digest: read_support.support_digest().to_string(),
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

    pub fn read_support_digest(&self) -> &str {
        &self.read_support_digest
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

impl ForgeQueryRuntime {
    pub fn public_read_composition_phase_one_closeout_for_support_profile(
        support_profile: &ForgeQueryRuntimeSupportProfile,
    ) -> ForgeQueryReadCompositionPhaseOneCloseout {
        let public_api_contract =
            super::ForgeQueryRuntimePublicApiContract::from_support_profile(support_profile);
        let support_matrix =
            ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&public_api_contract);
        let read_support =
            Self::public_read_composition_support_report_for_support_profile(support_profile);
        ForgeQueryReadCompositionPhaseOneCloseout::derive(
            public_api_contract.backend_posture(),
            &support_matrix,
            &read_support,
        )
    }

    pub fn public_read_composition_phase_one_closeout(
        &self,
    ) -> ForgeQueryReadCompositionPhaseOneCloseout {
        Self::public_read_composition_phase_one_closeout_for_support_profile(
            &self.backend.support_profile(),
        )
    }
}
