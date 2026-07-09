use worth_store::{
    CompletedSupportProgramAction, SupportActionPublicationWitness, SupportConsequenceEnvelope,
};

fn main() {
    let envelope: SupportConsequenceEnvelope =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let witness: SupportActionPublicationWitness =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let _ = CompletedSupportProgramAction::new(envelope, witness);
}
