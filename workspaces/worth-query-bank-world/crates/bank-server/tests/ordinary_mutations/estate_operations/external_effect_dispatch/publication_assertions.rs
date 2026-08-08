//! Public-consumer assertions for closed external-effect publication.

use bank_server::BankCommitReceipt;
use worth_query_host::facade::publication::application_aftermath::WorthQueryPublishedExternalEffectPostureKind;

pub(super) fn assert_dispatch_publication_and_work(receipt: &BankCommitReceipt, scenario: &str) {
    let posture = receipt
        .external_dispatch_posture()
        .expect("a dispatched effect has a published posture");
    let event_count = match posture.kind() {
        WorthQueryPublishedExternalEffectPostureKind::Completed
        | WorthQueryPublishedExternalEffectPostureKind::Acknowledged => 4,
        WorthQueryPublishedExternalEffectPostureKind::Unresolved => 3,
        WorthQueryPublishedExternalEffectPostureKind::NotDeclared
        | WorthQueryPublishedExternalEffectPostureKind::PendingDispatch => {
            panic!("{scenario}: a dispatched effect cannot publish as undispatched")
        }
    };
    let work = receipt.canonical_work().external_dispatch();
    assert_eq!(work.basis_preparations(), event_count);
    assert_eq!(work.digest_derivations(), event_count);
    assert_eq!(work.digest_text_materializations(), 0);
    assert!(
        work.canonical_encoded_bytes() > 0,
        "{scenario}: causal identities have canonical bytes"
    );
}
