use bank_domain::{
    estate::{EstateAction, EstateDisbursement},
    proposals::BankIdempotencyKey,
    schema::{BankSchema, EstateCase},
};
use worth_foundational::facade::{
    canonicalization, prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalDigestAlgorithmId,
    CanonicalDigestId, CanonicalIntegerWidth, CanonicalizationRuleVersion,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryAdmittedApplicationOperation, WorthQueryApplicationIdempotencyBinding,
    WorthQueryApplicationIdempotencyResolution,
};

const NOTIFICATION_KEY_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-bank.estate-notification-key");
const NOTIFICATION_INTENT_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-bank.estate-notification-intent");
const DISBURSEMENT_KEY_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-bank.estate-disbursement-key");
const DISBURSEMENT_INTENT_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-bank.estate-disbursement-intent");
mod elevation;

pub(super) use elevation::{elevation_binding, EstateElevationTransition};

pub(super) fn notification_binding(
    key: &BankIdempotencyKey,
    action: EstateAction,
) -> Result<WorthQueryApplicationIdempotencyBinding, BankEstateProgressionDenial> {
    let EstateAction::NotifyDeath {
        estate,
        notice,
        subject,
    } = action
    else {
        return Err(BankEstateProgressionDenial::CommandInput(
            "NotifyDeathEstateOperation",
        ));
    };
    let key_identity = derive_identity(
        NOTIFICATION_KEY_DOMAIN,
        "worth-bank-estate-notification-key-v1",
        [text_entry(
            NOTIFICATION_KEY_DOMAIN,
            "client-key",
            key.as_str(),
        )],
    );
    let intent_identity = derive_identity(
        NOTIFICATION_INTENT_DOMAIN,
        "worth-bank-estate-notification-intent-v1",
        [
            digest_entry(NOTIFICATION_INTENT_DOMAIN, "key", key_identity),
            unsigned_entry(NOTIFICATION_INTENT_DOMAIN, "estate", estate.get()),
            unsigned_entry(NOTIFICATION_INTENT_DOMAIN, "notice", notice.get()),
            unsigned_entry(NOTIFICATION_INTENT_DOMAIN, "subject", subject.get()),
        ],
    );
    Ok(WorthQueryApplicationIdempotencyBinding::new(
        *key_identity.bytes(),
        *intent_identity.bytes(),
    ))
}

pub(super) fn disbursement_binding(
    key: &BankIdempotencyKey,
    action: EstateAction,
) -> Result<WorthQueryApplicationIdempotencyBinding, BankEstateProgressionDenial> {
    let EstateAction::DisburseEstate(disbursement) = action else {
        return Err(BankEstateProgressionDenial::CommandInput(
            "DisburseEstateOperation",
        ));
    };
    let key_identity = derive_client_key(
        DISBURSEMENT_KEY_DOMAIN,
        "worth-bank-estate-disbursement-key-v1",
        key,
    );
    let intent_identity = disbursement_intent(key_identity, disbursement);
    Ok(WorthQueryApplicationIdempotencyBinding::new(
        *key_identity.bytes(),
        *intent_identity.bytes(),
    ))
}

fn disbursement_intent(
    key_identity: CanonicalDigestId,
    disbursement: EstateDisbursement,
) -> CanonicalDigestId {
    derive_identity(
        DISBURSEMENT_INTENT_DOMAIN,
        "worth-bank-estate-disbursement-intent-v1",
        [
            digest_entry(DISBURSEMENT_INTENT_DOMAIN, "key", key_identity),
            unsigned_entry(
                DISBURSEMENT_INTENT_DOMAIN,
                "estate",
                disbursement.estate.get(),
            ),
            text_entry(
                DISBURSEMENT_INTENT_DOMAIN,
                "source-account",
                &disbursement.source_account.canonical_text(),
            ),
            text_entry(
                DISBURSEMENT_INTENT_DOMAIN,
                "destination-account",
                &disbursement.destination_account.canonical_text(),
            ),
            unsigned_entry(
                DISBURSEMENT_INTENT_DOMAIN,
                "beneficiary",
                disbursement.beneficiary.get(),
            ),
            unsigned_entry(
                DISBURSEMENT_INTENT_DOMAIN,
                "amount-minor",
                disbursement.amount.minor_units() as u64,
            ),
        ],
    )
}

fn derive_client_key(
    domain: CanonicalBasisDomain,
    rule: &'static str,
    key: &BankIdempotencyKey,
) -> CanonicalDigestId {
    derive_identity(
        domain,
        rule,
        [text_entry(domain, "client-key", key.as_str())],
    )
}

fn derive_identity(
    domain: CanonicalBasisDomain,
    rule: &'static str,
    entries: impl IntoIterator<Item = CanonicalBasisEntry>,
) -> CanonicalDigestId {
    let version = CanonicalizationRuleVersion::new(rule).expect("fixed Bank rule is valid");
    let basis = prepare_canonical_basis_sequence(version, domain, entries)
        .into_result()
        .expect("Bank notification loci are unique");
    let ready = canonicalization()
        .digest()
        .for_sequence(basis, CanonicalDigestAlgorithmId::sha256())
        .into_result()
        .expect("SHA-256 admits notification basis");
    CanonicalDigestId::new(*canonicalization().digest().derive(ready).value().bytes())
}

fn text_entry(
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: &str,
) -> CanonicalBasisEntry {
    entry(
        domain,
        locus,
        CanonicalBasisValue::ExactText(value.to_owned().into()),
    )
}

fn digest_entry(
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: CanonicalDigestId,
) -> CanonicalBasisEntry {
    entry(domain, locus, CanonicalBasisValue::BytesDigest(value))
}

fn unsigned_entry(
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: u64,
) -> CanonicalBasisEntry {
    entry(
        domain,
        locus,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: u128::from(value),
        },
    )
}

fn entry(
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: CanonicalBasisValue,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Identity,
        value,
    )
}

use super::BankEstateProgressionDenial;
use crate::operation_commit::commit_receipt;
use crate::{
    BankCommitDenialKind, BankCommitDenialStage, BankIdentityRuntime, BankMutationCommitOutcome,
};

pub(super) fn resolve_admitted_idempotency<Operation>(
    runtime: &BankIdentityRuntime,
    admission: &WorthQueryAdmittedApplicationOperation<
        BankSchema,
        Operation,
        EstateAction,
        EstateCase,
    >,
    idempotency: WorthQueryApplicationIdempotencyBinding,
) -> Result<Option<BankMutationCommitOutcome>, BankEstateProgressionDenial> {
    match runtime
        .application_runtime()
        .resolve_admitted_application_idempotency(admission, idempotency)
        .map_err(BankEstateProgressionDenial::from_idempotency)?
        .into_resolution()
    {
        WorthQueryApplicationIdempotencyResolution::Unseen => Ok(None),
        WorthQueryApplicationIdempotencyResolution::AlreadyCommitted(receipt) => Ok(Some(
            BankMutationCommitOutcome::AlreadyCommitted(commit_receipt(receipt)),
        )),
        WorthQueryApplicationIdempotencyResolution::IntentDrift => {
            Ok(Some(BankMutationCommitOutcome::Denied {
                kind: BankCommitDenialKind::IdempotencyIntentDrift,
                stage: BankCommitDenialStage::Idempotency,
            }))
        }
    }
}
