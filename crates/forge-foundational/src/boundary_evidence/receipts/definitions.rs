use super::super::primitives::{definition, FoundationalBoundaryEvidencePrimitiveDefinition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceReceiptKind {
    Admission,
    Planning,
    Execution,
    Publication,
    Restoration,
    SupportPublication,
    CheckpointResume,
    Closeout,
}

pub const fn foundational_boundary_evidence_receipt_kind_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidenceReceiptKind>; 8] {
    [
        definition(
            FoundationalBoundaryEvidenceReceiptKind::Admission,
            "admission",
            "a receipt that attests an admitted authority-bearing boundary",
            "mere planning intent, support explanation, or lineage truth",
        ),
        definition(
            FoundationalBoundaryEvidenceReceiptKind::Planning,
            "planning",
            "a receipt that attests a completed planning boundary without claiming execution",
            "completed execution truth or a blocked closeout result",
        ),
        definition(
            FoundationalBoundaryEvidenceReceiptKind::Execution,
            "execution",
            "a receipt that attests a completed effectful or authority-bearing execution boundary",
            "planning-only intent or support-grade parity evidence",
        ),
        definition(
            FoundationalBoundaryEvidenceReceiptKind::Publication,
            "publication",
            "a receipt that attests a completed publication boundary",
            "execution truth in general or support-only diagnostics",
        ),
        definition(
            FoundationalBoundaryEvidenceReceiptKind::Restoration,
            "restoration",
            "a receipt that attests a completed restoration or reentry boundary",
            "fresh continuity truth or replay reconstruction by itself",
        ),
        definition(
            FoundationalBoundaryEvidenceReceiptKind::SupportPublication,
            "support_publication",
            "a receipt that attests a completed support publication boundary",
            "stronger authority truth or general lineage meaning",
        ),
        definition(
            FoundationalBoundaryEvidenceReceiptKind::CheckpointResume,
            "checkpoint_resume",
            "a receipt that attests a completed checkpoint or resume boundary",
            "ordinary current-fresh execution with no checkpoint seam",
        ),
        definition(
            FoundationalBoundaryEvidenceReceiptKind::Closeout,
            "closeout",
            "a receipt that attests a completed blocked or denied closeout boundary",
            "a successful executed boundary or mere planning intent",
        ),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceCloseoutDisposition {
    Blocked,
    Denied,
}

pub const fn foundational_boundary_evidence_closeout_disposition_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
    FoundationalBoundaryEvidenceCloseoutDisposition,
>; 2] {
    [
        definition(
            FoundationalBoundaryEvidenceCloseoutDisposition::Blocked,
            "blocked",
            "a completed closeout saying execution did not proceed because a blocker held",
            "a successful execution, publication, or restored continuity claim",
        ),
        definition(
            FoundationalBoundaryEvidenceCloseoutDisposition::Denied,
            "denied",
            "a completed closeout saying execution did not proceed because admission or policy denied it",
            "a successful execution or a merely missing receipt",
        ),
    ]
}
