//! Effect and linear-resource evidence from downstream-owned markers.

pub(crate) struct WitnessAction;
impl worth_proof::ActionMarker for WitnessAction {}

pub(crate) struct UndoAction;
impl worth_proof::ActionMarker for UndoAction {}
impl worth_proof::InverseOf<WitnessAction> for UndoAction {}

worth_proof::authority_marker!(pub(crate) WitnessAuthority);

fn authority() -> worth_proof::AuthorityWitness<WitnessAuthority> {
    WitnessAuthority::witness()
}

pub(crate) fn performed() -> worth_proof::Performed<WitnessAction, WitnessAuthority, u8> {
    worth_proof::Performed::record(&authority(), 1)
}

pub(crate) fn derived_from() -> worth_proof::DerivedFrom<WitnessAction, WitnessAuthority> {
    worth_proof::prove_derivation(&performed())
}

pub(crate) fn inverts() -> worth_proof::Inverts<WitnessAction, WitnessAuthority> {
    let undo = worth_proof::Performed::<UndoAction, WitnessAuthority>::record(&authority(), ());
    worth_proof::prove_inversion(&undo)
}

pub(crate) enum WitnessTerminal {
    Complete,
}

impl worth_proof::TerminalState for WitnessTerminal {
    fn label(&self) -> &'static str {
        "complete"
    }
}

pub(crate) fn linear_resource(
) -> worth_proof::LinearResource<u8, WitnessTerminal, WitnessAuthority> {
    worth_proof::LinearResource::mint(1, &authority())
}

pub(crate) fn terminal_receipt(
) -> worth_proof::TerminalReceipt<u8, WitnessTerminal, WitnessAuthority> {
    linear_resource().terminate(WitnessTerminal::Complete)
}
