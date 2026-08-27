//! Descriptive redo intent derived only from a proved undo (R8.42 / R8.10).
//!
//! Binding fields record what was true at derivation. They authorize nothing.
//! Invalidation-on-divergence is **not** a method of this type; it is lane
//! policy checked against Relational-owned branch history (R8.45). A 9.18 rebasing lane must reuse
//! this type unchanged.

use worth_foundational::facade::{
    canonicalization, prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalDigestAlgorithmId,
    CanonicalDigestId, CanonicalDigestWorkBudget, CanonicalIntegerWidth,
    CanonicalizationRuleVersion,
};
use worth_proof::{
    prove_inversion, ActionMarker, AuthorityProves, InverseOf, Inverts, Performed, Proof,
    ProofMarker,
};
use worth_query_installation::facade::WorthQueryCanonicalWorkEvidence;
use worth_relational::facade::history::RelationalCommitReceipt;
#[cfg(test)]
use worth_relational::facade::{
    history::{BranchId, CommitId},
    identity::VersionId,
};

use super::undo_admission::WorthQueryUndoAdmission;
use super::{WorthQueryAftermathCausalRole, WorthQueryAftermathDerivationFailure};
use crate::domain_computation::primary_graph::WorthQueryApplicationCommitReceipt;

const DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-query.application-aftermath-redo-intent");
const RULE_VERSION: &str = "worth-query-application-aftermath-redo-intent-v1";
const BUDGET: CanonicalDigestWorkBudget = match CanonicalDigestWorkBudget::new(24, 8 * 1_024) {
    Some(budget) => budget,
    None => panic!("fixed redo-intent canonical-work budget is valid"),
};

worth_proof::authority_marker!(WorthQueryUndoCompletionAuthority);

#[derive(Debug, Eq, PartialEq)]
struct WorthQueryUndoCompleted;
impl ProofMarker for WorthQueryUndoCompleted {}
impl AuthorityProves<WorthQueryUndoCompleted> for WorthQueryUndoCompletionAuthority {}

struct WorthQueryOriginalCommitAction;
impl ActionMarker for WorthQueryOriginalCommitAction {}

struct WorthQueryCompletedUndoAction;
impl ActionMarker for WorthQueryCompletedUndoAction {}
impl InverseOf<WorthQueryOriginalCommitAction> for WorthQueryCompletedUndoAction {}

/// Evidence that an undo completed through ordinary progression.
///
/// Minted only from a committed undo outcome. Possession authorizes nothing
/// about whether redo is lawful now (R8.43).
#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryProvedUndo {
    _completion: Proof<WorthQueryUndoCompleted, WorthQueryUndoCompletionAuthority>,
    causal: Inverts<WorthQueryOriginalCommitAction, WorthQueryUndoCompletionAuthority>,
    original_operation: [u8; 32],
    undo_commit: RelationalCommitReceipt,
    principal_scope_digest: [u8; 32],
    compatibility_generation: u64,
    runtime_instance: u64,
    _private: (),
}

impl WorthQueryProvedUndo {
    /// Seal a proved undo from committed undo facts.
    ///
    /// Not an authority mint — possession authorizes nothing about redo now
    /// (R8.43). Call only after ordinary undo progression committed.
    pub(crate) fn seal_completed(
        admission: &WorthQueryUndoAdmission,
        undo_receipt: &WorthQueryApplicationCommitReceipt,
    ) -> Result<Self, WorthQueryAftermathDerivationFailure> {
        if admission.runtime_instance() != undo_receipt.provider_runtime_instance_id() {
            return Err(WorthQueryAftermathDerivationFailure::BasisRejected);
        }
        let causality = undo_receipt
            .aftermath_causality()
            .filter(|fact| fact.role() == WorthQueryAftermathCausalRole::Undo)
            .filter(|fact| fact.parent() == admission.original_commit())
            .filter(|fact| fact.child() == undo_receipt.commit_reference())
            .ok_or(WorthQueryAftermathDerivationFailure::BasisRejected)?;
        let performed =
            Performed::<WorthQueryCompletedUndoAction, WorthQueryUndoCompletionAuthority>::record(
                &WorthQueryUndoCompletionAuthority::witness(),
                (),
            );
        Ok(Self {
            _completion: Proof::from_authority_witness(
                &WorthQueryUndoCompletionAuthority::witness(),
            ),
            causal: prove_inversion(&performed),
            original_operation: *admission.original_operation(),
            undo_commit: causality.child().clone(),
            principal_scope_digest: *admission.principal_scope_digest(),
            compatibility_generation: admission.compatibility_generation(),
            runtime_instance: admission.runtime_instance(),
            _private: (),
        })
    }

    #[cfg(test)]
    pub(crate) fn axis_probe(parts: WorthQueryProvedUndoAxisProbe) -> Self {
        let performed =
            Performed::<WorthQueryCompletedUndoAction, WorthQueryUndoCompletionAuthority>::record(
                &WorthQueryUndoCompletionAuthority::witness(),
                (),
            );
        Self {
            _completion: Proof::from_authority_witness(
                &WorthQueryUndoCompletionAuthority::witness(),
            ),
            causal: prove_inversion(&performed),
            original_operation: parts.original_operation,
            undo_commit: super::redo_intent_tests::probe_commit(parts.undo_commit_id),
            principal_scope_digest: parts.principal_scope_digest,
            compatibility_generation: parts.compatibility_generation,
            runtime_instance: parts.runtime_instance,
            _private: (),
        }
    }

    pub const fn original_operation(&self) -> &[u8; 32] {
        &self.original_operation
    }

    pub const fn undo_commit_id(&self) -> u64 {
        self.undo_commit.commit_id.0
    }

    pub const fn undo_commit(&self) -> &RelationalCommitReceipt {
        &self.undo_commit
    }

    pub const fn principal_scope_digest(&self) -> &[u8; 32] {
        &self.principal_scope_digest
    }

    pub const fn compatibility_generation(&self) -> u64 {
        self.compatibility_generation
    }

    pub const fn runtime_instance(&self) -> u64 {
        self.runtime_instance
    }
}

#[cfg(test)]
pub(crate) struct WorthQueryProvedUndoAxisProbe {
    pub original_operation: [u8; 32],
    pub undo_commit_id: u64,
    pub principal_scope_digest: [u8; 32],
    pub compatibility_generation: u64,
    pub runtime_instance: u64,
}

/// One bounded redo intent identity. Fan-out must not scale this derivation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRedoIntentIdentity {
    digest: CanonicalDigestId,
    work: WorthQueryCanonicalWorkEvidence,
}

impl WorthQueryRedoIntentIdentity {
    pub const fn digest(&self) -> &CanonicalDigestId {
        &self.digest
    }

    pub const fn work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.work
    }
}

/// Descriptive redo intent. Embeds no runtime authority and no replay state.
///
/// # Reuse for 9.18
///
/// This type records a bound linear head as descriptive data. It never asks
/// whether that head is still current. A rebasing lane can therefore reuse
/// [`WorthQueryRedoIntent`] unchanged and apply a different lane policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRedoIntent {
    identity: WorthQueryRedoIntentIdentity,
    original_operation: [u8; 32],
    undo_commit: RelationalCommitReceipt,
    bound_relational_head: RelationalCommitReceipt,
    principal_scope_digest: [u8; 32],
    compatibility_generation: u64,
    runtime_instance: u64,
    _private: (),
}

impl WorthQueryRedoIntent {
    /// Derive a descriptive intent from a proved undo and the head bound at
    /// derivation time. Does not consult live chain state for validity.
    pub(crate) fn derive(
        proved: &WorthQueryProvedUndo,
        bound_relational_head: RelationalCommitReceipt,
    ) -> Result<Self, WorthQueryAftermathDerivationFailure> {
        let identity = derive_identity(proved, &bound_relational_head)?;
        Ok(Self {
            identity,
            original_operation: *proved.original_operation(),
            undo_commit: proved.undo_commit().clone(),
            bound_relational_head,
            principal_scope_digest: *proved.principal_scope_digest(),
            compatibility_generation: proved.compatibility_generation(),
            runtime_instance: proved.runtime_instance(),
            _private: (),
        })
    }

    pub const fn identity(&self) -> &WorthQueryRedoIntentIdentity {
        &self.identity
    }

    pub const fn original_operation(&self) -> &[u8; 32] {
        &self.original_operation
    }

    pub const fn undo_commit_id(&self) -> u64 {
        self.undo_commit.commit_id.0
    }

    /// Head recorded at derivation — descriptive binding, not a live check.
    pub const fn undo_commit(&self) -> &RelationalCommitReceipt {
        &self.undo_commit
    }

    /// Exact Relational head recorded at derivation; descriptive, not authority.
    pub const fn bound_relational_head(&self) -> &RelationalCommitReceipt {
        &self.bound_relational_head
    }

    pub const fn principal_scope_digest(&self) -> &[u8; 32] {
        &self.principal_scope_digest
    }

    pub const fn compatibility_generation(&self) -> u64 {
        self.compatibility_generation
    }

    pub const fn runtime_instance(&self) -> u64 {
        self.runtime_instance
    }

    pub const fn work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.identity.work
    }
}

fn derive_identity(
    proved: &WorthQueryProvedUndo,
    bound_relational_head: &RelationalCommitReceipt,
) -> Result<WorthQueryRedoIntentIdentity, WorthQueryAftermathDerivationFailure> {
    let version =
        CanonicalizationRuleVersion::new(RULE_VERSION).expect("the redo-intent rule is valid");
    let entries = redo_intent_basis_entries(proved, bound_relational_head);
    let prepared = prepare_canonical_basis_sequence(version, DOMAIN, entries)
        .into_result()
        .map_err(|_| WorthQueryAftermathDerivationFailure::BasisRejected)?;
    let ready = canonicalization()
        .digest()
        .for_sequence_with_budget(prepared, CanonicalDigestAlgorithmId::sha256(), BUDGET)
        .into_result()
        .map_err(|_| WorthQueryAftermathDerivationFailure::DigestRejected)?;
    let derived = canonicalization().digest().derive(ready);
    Ok(WorthQueryRedoIntentIdentity {
        digest: CanonicalDigestId::new(*derived.value().bytes()),
        work: WorthQueryCanonicalWorkEvidence::one_digest(derived.metadata().work()),
    })
}

fn redo_intent_basis_entries(
    proved: &WorthQueryProvedUndo,
    bound_relational_head: &RelationalCommitReceipt,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        entry(
            "family",
            CanonicalBasisValue::ExactText("redo-intent".into()),
        ),
        entry(
            "original-operation",
            CanonicalBasisValue::BytesDigest(CanonicalDigestId::new(*proved.original_operation())),
        ),
        entry(
            "principal-scope",
            CanonicalBasisValue::BytesDigest(CanonicalDigestId::new(
                *proved.principal_scope_digest(),
            )),
        ),
        entry(
            "compatibility-generation",
            CanonicalBasisValue::UnsignedInteger {
                width: CanonicalIntegerWidth::Bits64,
                value: proved.compatibility_generation().into(),
            },
        ),
        entry(
            "runtime-instance",
            CanonicalBasisValue::UnsignedInteger {
                width: CanonicalIntegerWidth::Bits64,
                value: proved.runtime_instance().into(),
            },
        ),
    ];
    append_commit_reference_entries(&mut entries, "undo", proved.undo_commit());
    append_commit_reference_entries(&mut entries, "bound-head", bound_relational_head);
    entries
}

fn append_commit_reference_entries(
    entries: &mut Vec<CanonicalBasisEntry>,
    prefix: &str,
    commit: &RelationalCommitReceipt,
) {
    entries.push(unsigned_entry(
        &format!("{prefix}-commit"),
        commit.commit_id.0,
    ));
    entries.push(unsigned_entry(
        &format!("{prefix}-version"),
        commit.version_id.0,
    ));
    entries.push(entry(
        &format!("{prefix}-branch"),
        CanonicalBasisValue::ExactText(commit.branch_id.0.clone().into()),
    ));
    entries.push(unsigned_entry(
        &format!("{prefix}-parent-count"),
        commit.parents.len() as u64,
    ));
    entries.extend(
        commit
            .parents
            .iter()
            .enumerate()
            .map(|(index, parent)| unsigned_entry(&format!("{prefix}-parent-{index}"), parent.0)),
    );
}

fn unsigned_entry(locus: &str, value: u64) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: value.into(),
        },
    )
}

fn entry(locus: &str, value: CanonicalBasisValue) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        DOMAIN,
        CanonicalBasisLocus::Named(locus.to_owned().into()),
        CanonicalBasisEntryKind::Identity,
        value,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intent_does_not_decide_divergence() {
        // R8.45 — intent remains constructible and unchanged when a caller
        // later observes divergence. Validity is not a method of this type.
        let proved = WorthQueryProvedUndo::axis_probe(WorthQueryProvedUndoAxisProbe {
            original_operation: [1; 32],
            undo_commit_id: 20,
            principal_scope_digest: [2; 32],
            compatibility_generation: 1,
            runtime_instance: 7,
        });
        let bound = RelationalCommitReceipt {
            commit_id: CommitId(20),
            version_id: VersionId(20),
            branch_id: BranchId("main".to_owned()),
            parents: vec![CommitId(19)],
        };
        let intent = WorthQueryRedoIntent::derive(&proved, bound.clone()).expect("derive");
        assert_eq!(intent.bound_relational_head(), &bound);
        assert_eq!(intent.work().basis_preparations(), 1);
        assert_eq!(intent.work().digest_derivations(), 1);
        assert_eq!(intent.work().digest_text_materializations(), 0);
        // No API exists that takes a live head and returns invalidation.
        let _ = intent.original_operation();
        let _ = intent.undo_commit_id();
        let _ = intent.compatibility_generation();
    }
}
