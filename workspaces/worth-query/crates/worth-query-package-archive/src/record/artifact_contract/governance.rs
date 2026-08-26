use worth_foundational::facade::RetentionDeliveryProfile as Retention;
use worth_query_installation::facade::{
    WorthQueryArtifactClassification as Classification,
    WorthQueryArtifactCompatibilityContract as Compatibility,
    WorthQueryArtifactCompatibilityWindow as CompatibilityWindow,
    WorthQueryArtifactDeletionPosture as Deletion, WorthQueryArtifactDowngradePosture as Downgrade,
    WorthQueryArtifactGovernanceContract as Governance,
    WorthQueryArtifactLegalHoldPosture as LegalHold,
    WorthQueryArtifactProtocolVersion as ProtocolVersion,
    WorthQueryArtifactRedactionPosture as Redaction,
    WorthQueryArtifactRetirementRule as Retirement,
    WorthQueryArtifactSchemaVersion as SchemaVersion,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::record::decode_budget::RecordDecodeAttempt;
use crate::record::sequence::{decode_sequence, require_canonical_sequence, write_sequence};

pub(super) fn write_governance(
    output: &mut dyn BinaryEncodingSink,
    contract: &Governance,
) -> Result<(), Denial> {
    write_sequence(output, contract.audiences(), |output, audience| {
        output.text(audience)
    })?;
    write_classification(output, contract.classification())?;
    output.u16(redaction_tag(contract.redaction()))?;
    write_retention(output, contract.retention())?;
    output.u16(deletion_tag(contract.deletion()))?;
    output.u16(legal_hold_tag(contract.legal_hold()))
}

pub(super) fn decode_governance(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<Governance, Denial> {
    let audiences = decode_sequence(input, budget, 4, |input, _| Ok(input.text()?.to_owned()))?;
    require_canonical_sequence(&audiences)?;
    Ok(Governance::new(
        audiences,
        decode_classification(input)?,
        redaction_from_tag(input.u16()?)?,
        decode_retention(input)?,
        deletion_from_tag(input.u16()?)?,
        legal_hold_from_tag(input.u16()?)?,
    ))
}

pub(super) fn write_compatibility(
    output: &mut dyn BinaryEncodingSink,
    contract: &Compatibility,
) -> Result<(), Denial> {
    output.u32(contract.minimum_schema().get())?;
    output.u32(contract.maximum_schema().get())?;
    output.u32(contract.minimum_protocol().get())?;
    output.u32(contract.maximum_protocol().get())?;
    write_sequence(output, contract.migration_owners(), |output, owner| {
        output.text(owner)
    })?;
    write_retirement(output, contract.retirement())?;
    write_downgrade(output, contract.downgrade())
}

pub(super) fn decode_compatibility(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<Compatibility, Denial> {
    let window = CompatibilityWindow::new(
        SchemaVersion::new(input.u32()?),
        SchemaVersion::new(input.u32()?),
        ProtocolVersion::new(input.u32()?),
        ProtocolVersion::new(input.u32()?),
    );
    let migration_owners =
        decode_sequence(input, budget, 4, |input, _| Ok(input.text()?.to_owned()))?;
    require_canonical_sequence(&migration_owners)?;
    let retirement = decode_retirement(input)?;
    let downgrade = decode_downgrade(input)?;
    let mut owners = migration_owners.into_iter();
    let first = owners
        .next()
        .ok_or_else(|| Denial::new(Kind::InvalidRecordShape))?;
    let mut contract = Compatibility::new(window, first, retirement, downgrade);
    for owner in owners {
        contract = contract.migration_owner(owner);
    }
    Ok(contract)
}

pub(super) fn write_classification(
    output: &mut dyn BinaryEncodingSink,
    value: Classification,
) -> Result<(), Denial> {
    output.u16(classification_tag(value))
}

pub(super) fn decode_classification(input: &mut BinaryInput<'_>) -> Result<Classification, Denial> {
    match input.u16()? {
        1 => Ok(Classification::Public),
        2 => Ok(Classification::Internal),
        3 => Ok(Classification::Confidential),
        4 => Ok(Classification::Restricted),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

pub(super) fn write_retention(
    output: &mut dyn BinaryEncodingSink,
    value: Retention,
) -> Result<(), Denial> {
    output.u16(retention_tag(value))
}

pub(super) fn decode_retention(input: &mut BinaryInput<'_>) -> Result<Retention, Denial> {
    match input.u16()? {
        1 => Ok(Retention::Ephemeral),
        2 => Ok(Retention::Retained),
        3 => Ok(Retention::Durable),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn write_retirement(
    output: &mut dyn BinaryEncodingSink,
    retirement: &Retirement,
) -> Result<(), Denial> {
    match retirement {
        Retirement::Active => output.u16(1),
        Retirement::Retired => output.u16(2),
        Retirement::RetiredThroughSchema(version) => {
            output.u16(3)?;
            output.u32(version.get())
        }
    }
}

fn decode_retirement(input: &mut BinaryInput<'_>) -> Result<Retirement, Denial> {
    match input.u16()? {
        1 => Ok(Retirement::Active),
        2 => Ok(Retirement::Retired),
        3 => Ok(Retirement::RetiredThroughSchema(SchemaVersion::new(
            input.u32()?,
        ))),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn write_downgrade(
    output: &mut dyn BinaryEncodingSink,
    downgrade: &Downgrade,
) -> Result<(), Denial> {
    match downgrade {
        Downgrade::Denied => output.u16(1),
        Downgrade::SupportedBy { family } => {
            output.u16(2)?;
            output.text(family)
        }
    }
}

fn decode_downgrade(input: &mut BinaryInput<'_>) -> Result<Downgrade, Denial> {
    match input.u16()? {
        1 => Ok(Downgrade::Denied),
        2 => Ok(Downgrade::SupportedBy {
            family: input.text()?.to_owned(),
        }),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn classification_tag(value: Classification) -> u16 {
    match value {
        Classification::Public => 1,
        Classification::Internal => 2,
        Classification::Confidential => 3,
        Classification::Restricted => 4,
    }
}

const fn redaction_tag(value: Redaction) -> u16 {
    match value {
        Redaction::NotRequired => 1,
        Redaction::CanonicalProjectionOnly => 2,
        Redaction::DomainRedactorRequired => 3,
        Redaction::NeverDisclose => 4,
    }
}

fn redaction_from_tag(tag: u16) -> Result<Redaction, Denial> {
    match tag {
        1 => Ok(Redaction::NotRequired),
        2 => Ok(Redaction::CanonicalProjectionOnly),
        3 => Ok(Redaction::DomainRedactorRequired),
        4 => Ok(Redaction::NeverDisclose),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn retention_tag(value: Retention) -> u16 {
    match value {
        Retention::Ephemeral => 1,
        Retention::Retained => 2,
        Retention::Durable => 3,
    }
}

const fn deletion_tag(value: Deletion) -> u16 {
    match value {
        Deletion::DeleteWithRun => 1,
        Deletion::DeleteAfterRetention => 2,
        Deletion::DomainControlled => 3,
        Deletion::ExternallyControlled => 4,
    }
}

fn deletion_from_tag(tag: u16) -> Result<Deletion, Denial> {
    match tag {
        1 => Ok(Deletion::DeleteWithRun),
        2 => Ok(Deletion::DeleteAfterRetention),
        3 => Ok(Deletion::DomainControlled),
        4 => Ok(Deletion::ExternallyControlled),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn legal_hold_tag(value: LegalHold) -> u16 {
    match value {
        LegalHold::NotEligible => 1,
        LegalHold::DomainControlled => 2,
        LegalHold::RequiredWhenDirected => 3,
    }
}

fn legal_hold_from_tag(tag: u16) -> Result<LegalHold, Denial> {
    match tag {
        1 => Ok(LegalHold::NotEligible),
        2 => Ok(LegalHold::DomainControlled),
        3 => Ok(LegalHold::RequiredWhenDirected),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}
