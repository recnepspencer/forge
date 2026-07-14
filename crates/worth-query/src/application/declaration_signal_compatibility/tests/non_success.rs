use crate::application::WorthQueryDeclarationSignalCompatibilityChecked;

use super::support::{
    domain::{handle, DeferredFamily, IncompatibleFamily, Input},
    proof::envelope_checked_for,
};

#[test]
fn deferred_and_incompatible_families_stay_typed() {
    let handle = handle("non-success");

    match handle.signal_compatibility_checked(
        crate::application::WorthQueryDeclarationSignalCompatibilityInput::envelope_checked(
            envelope_checked_for(&handle, Input::<DeferredFamily>::new("edge:42")),
        ),
    ) {
        WorthQueryDeclarationSignalCompatibilityChecked::Deferred(_) => {}
        _ => panic!("deferred signal posture should remain deferred"),
    }

    match handle.signal_compatibility_checked(
        crate::application::WorthQueryDeclarationSignalCompatibilityInput::envelope_checked(
            envelope_checked_for(&handle, Input::<IncompatibleFamily>::new("edge:42")),
        ),
    ) {
        WorthQueryDeclarationSignalCompatibilityChecked::Denied(_) => {}
        _ => panic!("incompatible signal posture should deny"),
    }
}
