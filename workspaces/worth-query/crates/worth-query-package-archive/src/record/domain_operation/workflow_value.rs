use worth_query_installation::facade::WorthQueryWorkflowValueContract;

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

use super::artifact_reference::{decode_reference, write_reference};

pub(super) fn write_value(
    output: &mut dyn BinaryEncodingSink,
    value: &WorthQueryWorkflowValueContract,
) -> Result<(), Denial> {
    output.u16(match value {
        WorthQueryWorkflowValueContract::NotRequired => 1,
        WorthQueryWorkflowValueContract::Bool => 2,
        WorthQueryWorkflowValueContract::I64 => 3,
        WorthQueryWorkflowValueContract::U64 => 4,
        WorthQueryWorkflowValueContract::Text => 5,
        WorthQueryWorkflowValueContract::EntityIdentity => 6,
        WorthQueryWorkflowValueContract::Projection => 7,
        WorthQueryWorkflowValueContract::InstalledArtifact(_) => 8,
    })?;
    if let WorthQueryWorkflowValueContract::InstalledArtifact(reference) = value {
        write_reference(output, reference)?;
    }
    Ok(())
}

pub(super) fn decode_value(
    input: &mut BinaryInput<'_>,
) -> Result<WorthQueryWorkflowValueContract, Denial> {
    match input.u16()? {
        1 => Ok(WorthQueryWorkflowValueContract::NotRequired),
        2 => Ok(WorthQueryWorkflowValueContract::Bool),
        3 => Ok(WorthQueryWorkflowValueContract::I64),
        4 => Ok(WorthQueryWorkflowValueContract::U64),
        5 => Ok(WorthQueryWorkflowValueContract::Text),
        6 => Ok(WorthQueryWorkflowValueContract::EntityIdentity),
        7 => Ok(WorthQueryWorkflowValueContract::Projection),
        8 => Ok(WorthQueryWorkflowValueContract::InstalledArtifact(
            decode_reference(input)?,
        )),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}
