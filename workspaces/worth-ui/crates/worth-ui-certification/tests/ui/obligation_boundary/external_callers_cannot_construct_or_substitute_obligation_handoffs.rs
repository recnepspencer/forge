use worth_ui::facade::obligations::{
    UiAdmissionAuthorityHandoff, UiObligationCloseoutReport, UiObligationSelectionHandoff,
};

struct ForgedLaterSliceState;

fn require_selection_handoff(_: UiObligationSelectionHandoff<'_>) {}
fn require_admission_handoff(_: UiAdmissionAuthorityHandoff<'_>) {}

fn main() {
    let _ = UiObligationSelectionHandoff::new;
    let _ = UiAdmissionAuthorityHandoff::new;
    let _ = UiObligationCloseoutReport::new;

    let fake_closeout =
        unsafe { std::mem::MaybeUninit::<UiObligationCloseoutReport>::zeroed().assume_init() };

    require_admission_handoff(fake_closeout);
    require_selection_handoff(ForgedLaterSliceState);
}
