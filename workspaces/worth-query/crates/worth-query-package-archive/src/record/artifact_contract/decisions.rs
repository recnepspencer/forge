use worth_query_installation::facade::{
    WorthQueryArtifactKeyFamily as ArtifactKeyFamily,
    WorthQueryDecisionCausalParentShape as CausalParent,
    WorthQueryDecisionGovernance as DecisionGovernance,
    WorthQueryDecisionIdentity as DecisionIdentity, WorthQueryDecisionKind as DecisionKind,
    WorthQueryDecisionPayloadVersion as PayloadVersion,
    WorthQueryDecisionReasonFamily as ReasonFamily,
    WorthQueryDecisionRecordContract as DecisionContract,
    WorthQueryDecisionSchema as DecisionSchema,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::record::decode_budget::RecordDecodeAttempt;
use crate::record::sequence::{decode_sequence, require_canonical_sequence_by, write_sequence};

use super::governance::{
    decode_classification, decode_retention, write_classification, write_retention,
};

pub(super) fn write_decisions(
    output: &mut dyn BinaryEncodingSink,
    contract: &DecisionContract,
) -> Result<(), Denial> {
    match contract {
        DecisionContract::NotRequired => output.u16(1),
        DecisionContract::Declared { schemas } => {
            output.u16(2)?;
            write_sequence(output, schemas, write_schema)
        }
    }
}

pub(super) fn decode_decisions(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<DecisionContract, Denial> {
    match input.u16()? {
        1 => Ok(DecisionContract::not_required()),
        2 => {
            let schemas = decode_sequence(input, budget, 24, |input, _| decode_schema(input))?;
            require_canonical_sequence_by(&schemas, |schema| schema.kind())?;
            Ok(DecisionContract::declared(schemas))
        }
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn write_schema(
    output: &mut dyn BinaryEncodingSink,
    schema: &DecisionSchema,
) -> Result<(), Denial> {
    output.text(schema.kind().as_str())?;
    output.text(schema.reason_family().as_str())?;
    output.text(schema.affected_artifact_key_family().as_str())?;
    output.u16(causal_parent_tag(schema.causal_parent()))?;
    output.u32(schema.payload_version().get())?;
    write_classification(output, schema.classification())?;
    write_retention(output, schema.retention())
}

fn decode_schema(input: &mut BinaryInput<'_>) -> Result<DecisionSchema, Denial> {
    let kind = DecisionKind::new(input.text()?.to_owned())
        .map_err(|_| Denial::new(Kind::InvalidRecordShape))?;
    let reason_family = ReasonFamily::new(input.text()?.to_owned())
        .map_err(|_| Denial::new(Kind::InvalidRecordShape))?;
    let affected_artifact_key_family = ArtifactKeyFamily::new(input.text()?.to_owned())
        .map_err(|_| Denial::new(Kind::InvalidRecordShape))?;
    let identity = DecisionIdentity::new(kind, reason_family, affected_artifact_key_family);
    let causal_parent = causal_parent_from_tag(input.u16()?)?;
    let payload_version = PayloadVersion::new(input.u32()?);
    let governance =
        DecisionGovernance::new(decode_classification(input)?, decode_retention(input)?);
    Ok(DecisionSchema::new(
        identity,
        causal_parent,
        payload_version,
        governance,
    ))
}

const fn causal_parent_tag(value: CausalParent) -> u16 {
    match value {
        CausalParent::None => 1,
        CausalParent::OptionalSingle => 2,
        CausalParent::RequiredSingle => 3,
        CausalParent::OrderedMany => 4,
    }
}

fn causal_parent_from_tag(tag: u16) -> Result<CausalParent, Denial> {
    match tag {
        1 => Ok(CausalParent::None),
        2 => Ok(CausalParent::OptionalSingle),
        3 => Ok(CausalParent::RequiredSingle),
        4 => Ok(CausalParent::OrderedMany),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}
