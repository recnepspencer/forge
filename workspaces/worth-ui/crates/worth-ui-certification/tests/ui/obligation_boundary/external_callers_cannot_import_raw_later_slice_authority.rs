use worth_ui::facade::admission::{UiAdmissionReport, UiLegalityDecision, UiSupportSnapshot};
use worth_ui::facade::obligations::{
    UiObligationDispatchPlan, UiObligationEvidenceIndex, UiObligationEvidenceRecord,
    UiObligationVerdict, UiSelectedObligation, UiSelectedObligationIdentity,
    UiSelectedObligationSet,
};

fn main() {
    let _ = std::mem::size_of::<UiSelectedObligation>();
    let _ = std::mem::size_of::<UiSelectedObligationIdentity>();
    let _ = std::mem::size_of::<UiSelectedObligationSet>();
    let _ = std::mem::size_of::<UiObligationDispatchPlan>();
    let _ = std::mem::size_of::<UiObligationVerdict>();
    let _ = std::mem::size_of::<UiObligationEvidenceIndex>();
    let _ = std::mem::size_of::<UiObligationEvidenceRecord>();
    let _ = std::mem::size_of::<UiAdmissionReport>();
    let _ = std::mem::size_of::<UiLegalityDecision>();
    let _ = std::mem::size_of::<UiSupportSnapshot>();
}
