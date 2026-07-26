use crate::domain_computation::{
    WorthQueryArtifactDowngradePosture, WorthQueryArtifactRetirementRule,
    WorthQueryPortableArtifactContract,
};

use super::{
    portable_text, WorthQueryArtifactContractValidationDenial,
    WorthQueryArtifactContractValidationDenialKind as Kind,
};

pub(super) fn validate(
    contract: &WorthQueryPortableArtifactContract,
) -> Result<(), WorthQueryArtifactContractValidationDenial> {
    let compatibility = &contract.compatibility;
    let schema = contract.schema_version.get();
    let protocol = contract.protocol_version.get();
    if compatibility.minimum_schema().get() == 0
        || compatibility.maximum_schema().get() == 0
        || compatibility.minimum_schema().get() > schema
        || compatibility.maximum_schema().get() < schema
        || compatibility.minimum_schema().get() > compatibility.maximum_schema().get()
    {
        return Err(denial(contract, Kind::UnsupportedSchemaVersion));
    }
    if compatibility.minimum_protocol().get() == 0
        || compatibility.maximum_protocol().get() == 0
        || compatibility.minimum_protocol().get() > protocol
        || compatibility.maximum_protocol().get() < protocol
        || compatibility.minimum_protocol().get() > compatibility.maximum_protocol().get()
    {
        return Err(denial(contract, Kind::UnsupportedProtocolVersion));
    }
    if matches!(
        compatibility.retirement(),
        WorthQueryArtifactRetirementRule::Retired
    ) || matches!(
        compatibility.retirement(),
        WorthQueryArtifactRetirementRule::RetiredThroughSchema(version)
            if schema <= version.get()
    ) {
        return Err(denial(contract, Kind::RetiredSchemaVersion));
    }
    if compatibility.migration_owners().len() != 1
        || !portable_text(&compatibility.migration_owners()[0])
    {
        return Err(denial(contract, Kind::AmbiguousMigration));
    }
    if matches!(
        compatibility.downgrade(),
        WorthQueryArtifactDowngradePosture::SupportedBy { family }
            if !portable_text(family)
    ) {
        return Err(denial(contract, Kind::InvalidSemanticEvidence));
    }
    Ok(())
}

fn denial(
    contract: &WorthQueryPortableArtifactContract,
    kind: Kind,
) -> WorthQueryArtifactContractValidationDenial {
    WorthQueryArtifactContractValidationDenial::new(kind, contract.family.as_str())
}
