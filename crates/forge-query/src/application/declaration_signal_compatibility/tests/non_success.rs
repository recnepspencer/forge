use crate::application::ForgeQueryDeclarationSignalCompatibilityChecked;

use super::support::{
    domain::{handle, DeferredFamily, IncompatibleFamily, Input},
    proof::envelope_checked_for,
};

#[test]
fn deferred_and_incompatible_families_stay_typed() {
    let handle = handle("non-success");

    match handle.signal_compatibility_checked(
        crate::application::ForgeQueryDeclarationSignalCompatibilityInput::envelope_checked(
            envelope_checked_for(&handle, Input::<DeferredFamily>::new("edge:42")),
        ),
    ) {
        ForgeQueryDeclarationSignalCompatibilityChecked::Deferred(_) => {}
        _ => panic!("deferred signal posture should remain deferred"),
    }

    match handle.signal_compatibility_checked(
        crate::application::ForgeQueryDeclarationSignalCompatibilityInput::envelope_checked(
            envelope_checked_for(&handle, Input::<IncompatibleFamily>::new("edge:42")),
        ),
    ) {
        ForgeQueryDeclarationSignalCompatibilityChecked::Denied(_) => {}
        _ => panic!("incompatible signal posture should deny"),
    }
}
