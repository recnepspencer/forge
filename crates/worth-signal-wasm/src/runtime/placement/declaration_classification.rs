use std::convert::Infallible;

use worth_proof::TransitionOutcome;

use super::declaration_candidate::PlacementDeclarationCandidate;
use super::placement_category::WorkerPlacementCategory;
use super::raw_declaration_proof::{mint_raw_placement_proof, RawPlacementProof};

#[derive(Debug, PartialEq, Eq)]
pub struct PlacementClassifiedDeclaration {
    pub(in crate::runtime::placement) raw: RawPlacementProof,
    pub(in crate::runtime::placement) category: WorkerPlacementCategory,
    pub(in crate::runtime::placement) reason: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PlacementDenialArtifact {
    pub(in crate::runtime::placement) raw: RawPlacementProof,
    pub(in crate::runtime::placement) category: WorkerPlacementCategory,
    pub(in crate::runtime::placement) reason: String,
}

pub type PlacementClassificationOutcome = TransitionOutcome<
    PlacementClassifiedDeclaration,
    PlacementDenialArtifact,
    Infallible,
    Infallible,
    Infallible,
    Infallible,
>;

pub(crate) fn classify_declaration_placement(
    declaration: PlacementDeclarationCandidate,
) -> PlacementClassificationOutcome {
    let raw = mint_raw_placement_proof(&declaration);
    match resolve_declaration_placement_case(&declaration) {
        DeclarationPlacementCase::Unavailable => deny_unavailable_declaration(raw),
        DeclarationPlacementCase::WorkerExecutable => classify_worker_executable_declaration(raw),
        DeclarationPlacementCase::MainThreadHosted(reason) => {
            deny_main_thread_hosted_declaration(raw, reason)
        }
    }
}

enum DeclarationPlacementCase {
    WorkerExecutable,
    MainThreadHosted(&'static str),
    Unavailable,
}

fn resolve_declaration_placement_case(
    declaration: &PlacementDeclarationCandidate,
) -> DeclarationPlacementCase {
    if declaration.is_unavailable {
        DeclarationPlacementCase::Unavailable
    } else if !declaration.has_live_callback {
        DeclarationPlacementCase::WorkerExecutable
    } else {
        DeclarationPlacementCase::MainThreadHosted(main_thread_hosted_reason(declaration))
    }
}

fn classify_worker_executable_declaration(
    raw: RawPlacementProof,
) -> PlacementClassificationOutcome {
    TransitionOutcome::success(PlacementClassifiedDeclaration {
        raw,
        category: WorkerPlacementCategory::WorkerExecutable,
        reason: "expression-backed declaration has portable recipe data".to_owned(),
    })
}

fn deny_unavailable_declaration(raw: RawPlacementProof) -> PlacementClassificationOutcome {
    TransitionOutcome::denied(PlacementDenialArtifact {
        raw,
        category: WorkerPlacementCategory::Unavailable,
        reason: "declaration has no honest worker or main-thread-hosted placement".to_owned(),
    })
}

fn deny_main_thread_hosted_declaration(
    raw: RawPlacementProof,
    reason: &'static str,
) -> PlacementClassificationOutcome {
    TransitionOutcome::denied(PlacementDenialArtifact {
        raw,
        category: WorkerPlacementCategory::MainThreadHosted,
        reason: reason.to_owned(),
    })
}

fn main_thread_hosted_reason(declaration: &PlacementDeclarationCandidate) -> &'static str {
    if declaration.host_capability_read_count == 0 {
        "live callback remains process-local and needs main-thread-hosted execution"
    } else {
        "callback captured typed host capabilities and remains main-thread-hosted until worker lowering is explicit"
    }
}

#[cfg(test)]
mod tests {
    use worth_proof::TransitionOutcome;

    use super::*;
    use crate::runtime::core::WebSignalKind;
    use crate::runtime::placement::declaration_candidate::PlacementDeclarationOrigin;

    fn computed_declaration_candidate(
        origin: PlacementDeclarationOrigin,
    ) -> PlacementDeclarationCandidate {
        PlacementDeclarationCandidate {
            id: "derived".to_owned(),
            signal_kind: Some(WebSignalKind::Computed),
            origin,
            has_live_callback: false,
            callback_runtime_read_count: 0,
            host_capability_read_count: 0,
            is_unavailable: false,
        }
    }

    #[test]
    fn expression_recipe_classifies_as_worker_executable_success() {
        let outcome = classify_declaration_placement(computed_declaration_candidate(
            PlacementDeclarationOrigin::ExprSpec,
        ));

        let TransitionOutcome::Success(classified) = outcome else {
            panic!("expression recipe should classify successfully");
        };
        assert_eq!(
            classified.category,
            WorkerPlacementCategory::WorkerExecutable
        );
        assert_eq!(classified.raw.payload().id(), "derived");
        assert_eq!(classified.raw.payload().declaration_origin(), "exprSpec");
    }

    #[test]
    fn signal_tracked_callback_preserves_denied_main_thread_category() {
        let mut callback =
            computed_declaration_candidate(PlacementDeclarationOrigin::CallbackSignalTracked);
        callback.id = "callback".to_owned();
        callback.has_live_callback = true;

        let outcome = classify_declaration_placement(callback);

        let TransitionOutcome::Denied(denial) = outcome else {
            panic!("live callback should remain denied from worker execution");
        };
        assert_eq!(denial.category, WorkerPlacementCategory::MainThreadHosted);
        assert_eq!(denial.raw.payload().signal_kind(), "computed");
        assert_eq!(
            denial.raw.payload().declaration_origin(),
            "callbackSignalTracked"
        );
        assert!(denial.reason.contains("process-local"));
    }

    #[test]
    fn unavailable_declaration_preserves_unavailable_category() {
        let mut unavailable =
            computed_declaration_candidate(PlacementDeclarationOrigin::CallbackSignalTracked);
        unavailable.id = "missingCallback".to_owned();
        unavailable.is_unavailable = true;

        let outcome = classify_declaration_placement(unavailable);

        let TransitionOutcome::Denied(denial) = outcome else {
            panic!("unavailable declaration should deny");
        };
        assert_eq!(denial.category, WorkerPlacementCategory::Unavailable);
        assert_eq!(denial.raw.payload().id(), "missingCallback");
        assert!(denial.reason.contains("no honest worker"));
    }
}
