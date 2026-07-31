use super::{
    BankIdempotencyClaim, BankIdempotencyKey, BankOperationScopeBinding,
    BankOperationScopeEntityBinding, BankOperationScopeSchemaBinding, CanonicalProposalPayload,
};

#[test]
fn equal_typed_proposals_converge_on_the_same_claim() {
    let left = claim(
        7,
        "retry-1",
        CanonicalProposalPayload::new("send-money")
            .u64("principal", 11)
            .i64("amount-minor-units", 2500),
    );
    let right = claim(
        7,
        "retry-1",
        CanonicalProposalPayload::new("send-money")
            .u64("principal", 11)
            .i64("amount-minor-units", 2500),
    );

    assert_eq!(left, right);
}

#[test]
fn payload_change_preserves_retry_key_but_changes_intent() {
    let original = claim(
        7,
        "retry-1",
        CanonicalProposalPayload::new("send-money").i64("amount-minor-units", 2500),
    );
    let changed = claim(
        7,
        "retry-1",
        CanonicalProposalPayload::new("send-money").i64("amount-minor-units", 2600),
    );

    assert_eq!(original.key(), changed.key());
    assert_ne!(original.intent(), changed.intent());
}

#[test]
fn operation_scope_and_client_key_each_separate_retry_identity() {
    let baseline = claim(
        7,
        "retry-1",
        CanonicalProposalPayload::new("send-money").i64("amount-minor-units", 2500),
    );
    let other_operation = claim(
        7,
        "retry-1",
        CanonicalProposalPayload::new("withdraw-money").i64("amount-minor-units", 2500),
    );
    let other_scope = claim(
        8,
        "retry-1",
        CanonicalProposalPayload::new("send-money").i64("amount-minor-units", 2500),
    );
    let other_client_key = claim(
        7,
        "retry-2",
        CanonicalProposalPayload::new("send-money").i64("amount-minor-units", 2500),
    );

    assert_ne!(baseline.key(), other_operation.key());
    assert_ne!(baseline.key(), other_scope.key());
    assert_ne!(baseline.key(), other_client_key.key());
}

#[test]
fn field_locus_and_scalar_type_are_part_of_payload_identity() {
    let amount = claim(
        7,
        "retry-1",
        CanonicalProposalPayload::new("send-money").u64("amount", 1),
    );
    let principal = claim(
        7,
        "retry-1",
        CanonicalProposalPayload::new("send-money").u64("principal", 1),
    );
    let textual_amount = claim(
        7,
        "retry-1",
        CanonicalProposalPayload::new("send-money").text("amount", "1"),
    );

    assert_eq!(amount.key(), principal.key());
    assert_eq!(amount.key(), textual_amount.key());
    assert_ne!(amount.intent(), principal.intent());
    assert_ne!(amount.intent(), textual_amount.intent());
}

fn claim(scope_byte: u8, key: &str, payload: CanonicalProposalPayload) -> BankIdempotencyClaim {
    BankIdempotencyClaim::derive(
        BankOperationScopeBinding::new(
            1,
            BankOperationScopeSchemaBinding::new(2, 3, [4; 32], [5; 32]),
            "operation-authority",
            BankOperationScopeEntityBinding::new(0, 6, 1),
            BankOperationScopeEntityBinding::new(0, u64::from(scope_byte), 1),
        ),
        &BankIdempotencyKey::new(key).expect("test idempotency key is valid"),
        payload,
    )
}
