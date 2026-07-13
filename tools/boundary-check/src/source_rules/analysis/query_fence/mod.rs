//! Query source fence: import shape, public signature, and public re-export law.
//!
//! Detection is deliberately spelling based. The pass recognizes crate roots
//! from the configured Query audience contract and item names explicitly
//! exported by the configured audience facades. It does not perform full Rust
//! type resolution or follow arbitrary local aliases and renames. Phase 6's
//! facade-manifest ratchet owns the deferred alias/surface-drift gap; this pass
//! must not be interpreted as a resolved-type guarantee.

mod public_reexport;
mod public_signature;
mod source_path;
mod vocabulary;

use super::crate_modules::ModuleNode;
use super::crate_modules::{GovernedCrate, ModuleGraph};
use super::public_reachability::Reachability;
use crate::diagnostics::Diagnostic;

pub(super) use vocabulary::QueryVocabulary;

pub(super) fn enforce_query_fence(
    governed: &GovernedCrate,
    graph: &ModuleGraph,
    reachable: &Reachability,
    vocabulary: &QueryVocabulary,
) -> Vec<Diagnostic> {
    let mut diagnostics = source_path::enforce(governed, graph, reachable, vocabulary);
    diagnostics.extend(public_signature::enforce(
        governed, graph, reachable, vocabulary,
    ));
    diagnostics.extend(public_reexport::enforce(
        governed, graph, reachable, vocabulary,
    ));
    diagnostics.sort_by(Diagnostic::compare_code_subject_message);
    diagnostics.dedup_by(|left, right| left.has_same_code_subject_message(right));
    diagnostics
}

pub(super) fn enforce_query_target_paths(
    governed: &GovernedCrate,
    targets: &[ModuleNode],
    vocabulary: &QueryVocabulary,
) -> Vec<Diagnostic> {
    let mut diagnostics = source_path::enforce_nodes(governed, targets, vocabulary, true);
    diagnostics.sort_by(Diagnostic::compare_subject_message);
    diagnostics.dedup_by(|left, right| left.has_same_code_subject_message(right));
    diagnostics
}
