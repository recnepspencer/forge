use worth_store_budgets::CounterEvidenceStrength;

use crate::BlobReachabilityCounterSnapshot;

use super::super::BlobPublicationCounterSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationCounterReceiptIdentity {
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlobPublicationRecoveryOperationDigest {
    value: String,
}

impl BlobPublicationCounterReceiptIdentity {
    pub(crate) fn from_reachability_staging(
        publication_counters: BlobPublicationCounterSnapshot,
        reachability_counters: BlobReachabilityCounterSnapshot,
    ) -> Self {
        Self {
            value: format!(
                "blob-publication-counter-receipt:v1:publication={}:reachability={}",
                publication_counter_basis(publication_counters),
                reachability_counter_basis(reachability_counters)
            ),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl BlobPublicationRecoveryOperationDigest {
    pub(crate) fn from_stable_parts(phase: &str, stable_basis: &str) -> Self {
        Self {
            value: format!("blob-publication-recovery:v1:{phase}:{stable_basis}"),
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.value
    }
}

pub(crate) fn recovery_evidence_digest(
    crash_phase: &str,
    replay_digest: &str,
    stable_basis: &str,
) -> String {
    format!("blob-publication-recovery-evidence:v1:{crash_phase}:{replay_digest}:{stable_basis}")
}

fn publication_counter_basis(counters: BlobPublicationCounterSnapshot) -> String {
    format!(
        "root_candidates={}:reachability_staged={}:wal_records={}:session_closeouts={}:committed_publications={}:recovered_states={}:denied_promotions={}:visible_observations={}",
        counters.root_candidates(),
        counters.reachability_staged(),
        counters.wal_records(),
        counters.session_closeouts(),
        counters.committed_publications(),
        counters.recovered_states(),
        counters.denied_promotions(),
        counters.visible_observations()
    )
}

fn reachability_counter_basis(counters: BlobReachabilityCounterSnapshot) -> String {
    format!(
        "strength={}:reachable_chunks={}:reference_edges={}:dedupe_edges={}:protected_holds={}:orphan_candidates={}:stale_reference_denials={}:copied_row_denials={}:wrong_authority_denials={}:empty_proof_denials={}:reclaim_denials={}:replay_convergence_checks={}",
        counter_strength_basis(counters.strength()),
        counters.reachable_chunks(),
        counters.reference_edges(),
        counters.dedupe_reference_edges(),
        counters.protected_holds(),
        counters.orphan_candidates(),
        counters.stale_reference_denials(),
        counters.copied_row_denials(),
        counters.wrong_authority_denials(),
        counters.empty_proof_denials(),
        counters.reclaim_denials(),
        counters.replay_convergence_checks()
    )
}

const fn counter_strength_basis(strength: CounterEvidenceStrength) -> &'static str {
    match strength {
        CounterEvidenceStrength::Exact => "exact",
        CounterEvidenceStrength::Bounded => "bounded",
        CounterEvidenceStrength::Sampled => "sampled",
        CounterEvidenceStrength::Derived => "derived",
        CounterEvidenceStrength::CertificationOnly => "certification-only",
        CounterEvidenceStrength::Unavailable => "unavailable",
    }
}
