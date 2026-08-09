use worth_query::facade::installed::collection::WorthQueryCollectionDeliveryDenialKind as Denial;
use worth_ui_query_binding::certification::{
    certify_collection_patch_attack, WorthUiCollectionPatchAttack as Attack,
};

#[test]
pub(crate) fn hostile_query_patches_return_exact_denials_and_mint_no_ui_effect() {
    let cases = [
        (Attack::Duplicate, Denial::DuplicateOrReorderedDelivery, 1),
        (Attack::Reordered, Denial::DuplicateOrReorderedDelivery, 2),
        (Attack::Superseded, Denial::SupersededPatch, 1),
        (Attack::ForeignLease, Denial::WrongLease, 2),
        (Attack::WrongWindow, Denial::WindowContractMismatch, 1),
    ];
    for (attack, expected_denial, successful_facts) in cases {
        let report = certify_collection_patch_attack(attack);
        assert_eq!(report.attack(), attack);
        assert_eq!(report.denial(), expected_denial);
        assert!(
            report.state_preserved(),
            "{attack:?} mutated UI-visible Query row state"
        );
        assert!(
            report.follow_up_delivery_succeeded(),
            "{attack:?} poisoned the next valid Query delivery"
        );
        assert_eq!(
            report.successful_facts().len(),
            successful_facts,
            "{attack:?} minted a fact without an applied Query receipt"
        );
        let expected_resources = if matches!(attack, Attack::ForeignLease | Attack::WrongWindow) {
            2
        } else {
            1
        };
        assert_eq!(report.closed_resources(), expected_resources);
        assert_eq!(report.terminal_owners(), expected_resources);
    }
}
