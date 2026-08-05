use bank_domain::{
    accounting::{BankJournalEntry, BankPosting},
    estate::{
        BankEstateWorld, CapabilityGrantId, DeathNoticeStatus, EstateCapabilityOperation,
        EstateDeathNotice, EstateLegalAuthority, LegalAuthorityKind,
    },
    model::{BankSnapshotVersion, JournalEntryId, PostingId, SignedMoney},
    proposals::{BankSnapshot, BankSnapshotBuilder},
    schema::{AccountStatus, PostingPurpose},
};

use super::super::{
    grant, GrantSpec, ACCOUNT, APPROVER, AUTHORITY, DECEASED, ESTATE, EXECUTOR, INSTITUTION,
    NOTICE, OTHER_ACCOUNT, REVIEWER, SPECIALIST,
};

pub(super) fn install_truth(estate: BankEstateWorld) -> BankEstateWorld {
    let mut notification = GrantSpec::view();
    notification.operation = EstateCapabilityOperation::NotifyDeath;
    notification.field = None;
    estate
        .with_death_notice(EstateDeathNotice {
            id: NOTICE,
            subject: DECEASED,
            status: DeathNoticeStatus::Reported,
        })
        .with_legal_authority(EstateLegalAuthority {
            id: AUTHORITY,
            estate: ESTATE,
            holder: EXECUTOR,
            kind: LegalAuthorityKind::CourtAppointment,
            recognized: true,
        })
        .with_executor(ESTATE, EXECUTOR)
        .with_beneficiary(ESTATE, APPROVER)
        .with_joint_owner(OTHER_ACCOUNT, APPROVER)
        .with_grant(grant(
            CapabilityGrantId::new(94).unwrap(),
            SPECIALIST,
            notification,
        ))
}

pub(super) fn snapshot() -> BankSnapshot {
    BankSnapshotBuilder::new(BankSnapshotVersion::new(1).unwrap())
        .institution(INSTITUTION)
        .principal(DECEASED)
        .principal(SPECIALIST)
        .principal(EXECUTOR)
        .principal(APPROVER)
        .principal(REVIEWER)
        .personal_account(
            ACCOUNT,
            INSTITUTION,
            DECEASED,
            bank_domain::model::AccountName::new("Estate Operating").unwrap(),
            AccountStatus::Open,
        )
        .personal_account(
            OTHER_ACCOUNT,
            INSTITUTION,
            APPROVER,
            bank_domain::model::AccountName::new("Beneficiary Settlement").unwrap(),
            AccountStatus::Open,
        )
        .projected_journal(BankJournalEntry::from_projection(
            JournalEntryId::new(91).unwrap(),
            PostingPurpose::OpeningFunding,
            vec![
                BankPosting::from_projection(
                    PostingId::new(92).unwrap(),
                    OTHER_ACCOUNT,
                    SignedMoney::from_minor(-10_000),
                ),
                BankPosting::from_projection(
                    PostingId::new(93).unwrap(),
                    ACCOUNT,
                    SignedMoney::from_minor(10_000),
                ),
            ],
            None,
        ))
        .build()
        .expect("the disbursement currentness snapshot is balanced")
}
