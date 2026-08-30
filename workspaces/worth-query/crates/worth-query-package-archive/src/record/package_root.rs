use worth_query_installation::facade::{
    WorthQueryInstallationCapabilityFamily, WorthQueryInstallationConfigSectionFamily,
    WorthQueryInstallationContributionCategory, WorthQueryInstallationOperatingRequirement,
    WorthQueryPortableDefinition, WorthQueryPortableDefinitionKind,
    WorthQueryPortableDomainIdentity, WorthQueryPortablePackageRecord,
    WorthQueryPortablePackageRecordFamily as Family,
};

use crate::binary_input::BinaryInput;
use crate::binary_output::BinaryOutput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
const LENGTH_PREFIX_BYTES: u64 = 4;

pub(super) const fn is_package_root_family(family: Family) -> bool {
    matches!(
        family,
        Family::DomainIdentity
            | Family::CapabilityRequirement
            | Family::ConfigurationRequirement
            | Family::OperatingRequirement
            | Family::Definition
            | Family::ContributionPolicy
    )
}

pub(super) fn write_package_root_payload(
    record: &WorthQueryPortablePackageRecord,
    output: &mut BinaryOutput,
) -> Result<(), Denial> {
    match record {
        WorthQueryPortablePackageRecord::DomainIdentity(identity) => {
            output.text(identity.owner());
            output.u32(identity.major());
            output.u32(identity.minor());
        }
        WorthQueryPortablePackageRecord::CapabilityRequirement(value) => {
            output.text(value.as_str());
        }
        WorthQueryPortablePackageRecord::ConfigurationRequirement(value) => {
            output.text(value.as_str());
        }
        WorthQueryPortablePackageRecord::OperatingRequirement(value) => {
            output.text(value.as_str());
        }
        WorthQueryPortablePackageRecord::Definition(definition) => {
            output.u16(definition_kind_tag(definition.kind())?);
            output.text(definition.slot());
            output.text(definition.semantics());
        }
        WorthQueryPortablePackageRecord::ContributionPolicy(value) => {
            output.text(value.as_str());
        }
        _ => return Err(Denial::new(Kind::PackageRootRecordFamilyRequired)),
    }
    Ok(())
}

pub(super) fn decode_package_root_payload(
    family: Family,
    payload: &[u8],
) -> Result<WorthQueryPortablePackageRecord, Denial> {
    let mut input = BinaryInput::new(payload);
    let record = match family {
        Family::DomainIdentity => WorthQueryPortablePackageRecord::DomainIdentity(
            WorthQueryPortableDomainIdentity::new(input.text()?, input.u32()?, input.u32()?),
        ),
        Family::CapabilityRequirement => WorthQueryPortablePackageRecord::CapabilityRequirement(
            WorthQueryInstallationCapabilityFamily::new(input.text()?),
        ),
        Family::ConfigurationRequirement => {
            WorthQueryPortablePackageRecord::ConfigurationRequirement(
                WorthQueryInstallationConfigSectionFamily::new(input.text()?),
            )
        }
        Family::OperatingRequirement => WorthQueryPortablePackageRecord::OperatingRequirement(
            WorthQueryInstallationOperatingRequirement::new(input.text()?),
        ),
        Family::Definition => {
            let kind = definition_kind_from_tag(input.u16()?)?;
            let slot = input.text()?;
            let semantics = input.text()?;
            WorthQueryPortablePackageRecord::Definition(definition(kind, slot, semantics)?)
        }
        Family::ContributionPolicy => WorthQueryPortablePackageRecord::ContributionPolicy(
            WorthQueryInstallationContributionCategory::new(input.text()?),
        ),
        _ => return Err(Denial::new(Kind::PackageRootRecordFamilyRequired)),
    };
    if !input.is_finished() {
        return Err(Denial::new(Kind::TrailingBytes));
    }
    Ok(record)
}

pub(super) fn package_root_payload_byte_length(
    record: &WorthQueryPortablePackageRecord,
) -> Result<u64, Denial> {
    match record {
        WorthQueryPortablePackageRecord::DomainIdentity(identity) => {
            text_byte_length(identity.owner())?
                .checked_add(8)
                .ok_or_else(|| Denial::new(Kind::InvalidRecordLength))
        }
        WorthQueryPortablePackageRecord::CapabilityRequirement(value) => {
            text_byte_length(value.as_str())
        }
        WorthQueryPortablePackageRecord::ConfigurationRequirement(value) => {
            text_byte_length(value.as_str())
        }
        WorthQueryPortablePackageRecord::OperatingRequirement(value) => {
            text_byte_length(value.as_str())
        }
        WorthQueryPortablePackageRecord::Definition(definition) => {
            definition_kind_tag(definition.kind())?;
            text_byte_length(definition.slot())?
                .checked_add(text_byte_length(definition.semantics())?)
                .and_then(|length| length.checked_add(2))
                .ok_or_else(|| Denial::new(Kind::InvalidRecordLength))
        }
        WorthQueryPortablePackageRecord::ContributionPolicy(value) => {
            text_byte_length(value.as_str())
        }
        _ => Err(Denial::new(Kind::PackageRootRecordFamilyRequired)),
    }
}

fn text_byte_length(value: &str) -> Result<u64, Denial> {
    let length = u32::try_from(value.len()).map_err(|_| Denial::new(Kind::InvalidRecordLength))?;
    LENGTH_PREFIX_BYTES
        .checked_add(u64::from(length))
        .ok_or_else(|| Denial::new(Kind::InvalidRecordLength))
}

const fn definition_kind_tag(kind: WorthQueryPortableDefinitionKind) -> Result<u16, Denial> {
    match kind {
        WorthQueryPortableDefinitionKind::Invariant => Ok(1),
        WorthQueryPortableDefinitionKind::GraphReadOperation => Ok(2),
        WorthQueryPortableDefinitionKind::DeclarationFamily => Ok(3),
        WorthQueryPortableDefinitionKind::DomainOperation => {
            Err(Denial::new(Kind::UnsupportedDefinitionKind))
        }
    }
}

const fn definition_kind_from_tag(tag: u16) -> Result<WorthQueryPortableDefinitionKind, Denial> {
    match tag {
        1 => Ok(WorthQueryPortableDefinitionKind::Invariant),
        2 => Ok(WorthQueryPortableDefinitionKind::GraphReadOperation),
        3 => Ok(WorthQueryPortableDefinitionKind::DeclarationFamily),
        _ => Err(Denial::new(Kind::UnsupportedDefinitionKind)),
    }
}

fn definition(
    kind: WorthQueryPortableDefinitionKind,
    slot: &str,
    semantics: &str,
) -> Result<WorthQueryPortableDefinition, Denial> {
    match kind {
        WorthQueryPortableDefinitionKind::Invariant => {
            Ok(WorthQueryPortableDefinition::invariant(slot, semantics))
        }
        WorthQueryPortableDefinitionKind::GraphReadOperation => Ok(
            WorthQueryPortableDefinition::graph_read_operation(slot, semantics),
        ),
        WorthQueryPortableDefinitionKind::DeclarationFamily => Ok(
            WorthQueryPortableDefinition::declaration_family(slot, semantics),
        ),
        WorthQueryPortableDefinitionKind::DomainOperation => {
            Err(Denial::new(Kind::UnsupportedDefinitionKind))
        }
    }
}
