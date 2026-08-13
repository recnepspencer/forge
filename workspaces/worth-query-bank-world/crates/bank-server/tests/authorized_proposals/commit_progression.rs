use bank_domain::model::{BankPrincipalId, Money};
use bank_domain::proposals::BankProposedEffect;
use bank_domain::schema::SendMoney;
use bank_server::{
    BankMutationCommitOutcome, BankOperationProposals, BankPrincipalSeed, BankSendMoneyPreparation,
    BankWorldSeed,
};
use worth_query_host::facade::publication::application_aftermath::WorthQueryPublishedApplicationCommitKind;
use worth_query_host::facade::publication::domain_computation::WorthQueryPublishedApplicationCommitAttemptReleasePosture;

use super::fixture::{expect_send_proposal, funded_personal_world, id, key};
use crate::support::{block_on, request_scope, runtime, CausalCredential, DynamicIdentity};

#[test]
fn consumer_authenticates_authorizes_and_prepares_a_send_proposal() {
    let snapshot = funded_personal_world();
    let owner = DynamicIdentity::new("owner");
    let recipient = DynamicIdentity::new("recipient");
    let employee = DynamicIdentity::new("employee");
    let world = runtime(
        BankWorldSeed::new(snapshot.clone())
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 1),
                owner.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 2),
                recipient.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 3),
                employee.external(),
            )),
    );
    let request = request_scope();
    let actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&owner),
        &request,
    ))
    .unwrap();
    let source = snapshot
        .primary_account(id(BankPrincipalId::new, 1))
        .unwrap();
    let input = SendMoney {
        from: source,
        recipient: id(BankPrincipalId::new, 2),
        amount: Money::from_minor(2_500).unwrap(),
    };
    let admission = world
        .runtime
        .authorize_send_money(&actor, source, Default::default(), &request)
        .unwrap();
    let proposal = expect_send_proposal(
        BankOperationProposals::prepare_send_money(
            &world.runtime,
            admission,
            &key("consumer-send"),
            &input,
        )
        .unwrap(),
    );
    assert_eq!(proposal.admission().actor(), id(BankPrincipalId::new, 1));
    assert_eq!(proposal.admission().scope(), source);
    let [BankProposedEffect::AppendJournal(first_send)] = proposal.invariant().effects() else {
        panic!("send money must propose exactly one journal append");
    };
    let committed_journal = first_send.id();
    assert_eq!(first_send.postings().len(), 2);
    assert_eq!(proposal.invariant().proposed_snapshot().journal().len(), 1);
    let BankMutationCommitOutcome::Committed(receipt) =
        world.runtime.commit_send_money(proposal).unwrap()
    else {
        panic!("first exact send must commit");
    };

    let retry_admission = world
        .runtime
        .authorize_send_money(&actor, source, Default::default(), &request)
        .unwrap();
    let retry = BankOperationProposals::prepare_send_money(
        &world.runtime,
        retry_admission,
        &key("consumer-send"),
        &input,
    )
    .unwrap();
    match retry {
        BankSendMoneyPreparation::AlreadyCommitted {
            receipt: resolved, ..
        } => {
            assert_eq!(
                resolved.changed_record_count(),
                receipt.changed_record_count()
            );
            assert_eq!(
                resolved.emitted_effect_count(),
                receipt.emitted_effect_count()
            );
            assert_eq!(resolved.aftermath(), receipt.aftermath());
            assert_eq!(resolved.canonical_work(), receipt.canonical_work());
            let publication = resolved.publication().inspect();
            assert_eq!(
                publication.kind(),
                WorthQueryPublishedApplicationCommitKind::Recovered
            );
            assert_eq!(
                publication.attempt_release(),
                WorthQueryPublishedApplicationCommitAttemptReleasePosture::NotAttempted
            );
        }
        BankSendMoneyPreparation::Proposal(_) => {
            panic!("an exact retry must not prepare a second proposal")
        }
        BankSendMoneyPreparation::IntentDrift { .. } => {
            panic!("an exact retry must not be classified as intent drift")
        }
    }

    let drift_admission = world
        .runtime
        .authorize_send_money(&actor, source, Default::default(), &request)
        .unwrap();
    let drift = BankOperationProposals::prepare_send_money(
        &world.runtime,
        drift_admission,
        &key("consumer-send"),
        &SendMoney {
            amount: Money::from_minor(2_501).unwrap(),
            ..input
        },
    )
    .unwrap();
    assert!(matches!(
        drift,
        BankSendMoneyPreparation::IntentDrift { .. }
    ));

    let next_admission = world
        .runtime
        .authorize_send_money(&actor, source, Default::default(), &request)
        .unwrap();
    let next = expect_send_proposal(
        BankOperationProposals::prepare_send_money(
            &world.runtime,
            next_admission,
            &key("consumer-send-next"),
            &SendMoney {
                amount: Money::from_minor(500).unwrap(),
                ..input
            },
        )
        .unwrap(),
    );
    assert_eq!(next.invariant().proposed_snapshot().journal().len(), 1);
    assert!(
        next.invariant()
            .proposed_snapshot()
            .journal_entry(committed_journal)
            .is_none(),
        "a decision proposal must not copy committed history into its bounded basis"
    );
    assert_ne!(
        next.invariant().proposed_snapshot().journal()[0].id(),
        committed_journal
    );
}
