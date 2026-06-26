use forge_query::facade::consumer_kit::{
    load_support_pin_contract_terminal_json_document,
    ForgeQueryExternalSupportPinContractTerminalJsonDocument, ForgeQuerySupportPinContract,
    ForgeQuerySupportPinContractSchemaVersion, ForgeQuerySupportPinningError,
};

const PRIMITIVE_CONSTRUCTION_QUERY_SUPPORT_PIN_CONTRACT_JSON: &str =
    include_str!("query_support_pins.json");

pub(crate) fn primitive_construction_query_support_pins(
) -> Result<ForgeQuerySupportPinContract, ForgeQuerySupportPinningError> {
    let document =
        ForgeQueryExternalSupportPinContractTerminalJsonDocument::from_static_external_terminal_json_document(
            PRIMITIVE_CONSTRUCTION_QUERY_SUPPORT_PIN_CONTRACT_JSON,
        );
    load_support_pin_contract_terminal_json_document(
        &document,
        ForgeQuerySupportPinContractSchemaVersion::current(),
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionQuerySupportPinAdoptionEvidence {
    consumer_name: String,
    loaded_contract_digest: String,
    evaluated_requirement_count: usize,
    observed_row_count: usize,
    schema_version: ForgeQuerySupportPinContractSchemaVersion,
}

impl PrimitiveConstructionQuerySupportPinAdoptionEvidence {
    pub(crate) fn consumer_name(&self) -> &str {
        &self.consumer_name
    }

    pub(crate) fn loaded_contract_digest(&self) -> &str {
        &self.loaded_contract_digest
    }

    pub(crate) fn evaluated_requirement_count(&self) -> usize {
        self.evaluated_requirement_count
    }

    pub(crate) fn observed_row_count(&self) -> usize {
        self.observed_row_count
    }

    pub(crate) fn schema_version(&self) -> ForgeQuerySupportPinContractSchemaVersion {
        self.schema_version
    }
}

pub(crate) fn primitive_construction_query_support_pin_adoption_evidence(
) -> Result<PrimitiveConstructionQuerySupportPinAdoptionEvidence, ForgeQuerySupportPinningError> {
    let contract = primitive_construction_query_support_pins()?;
    Ok(PrimitiveConstructionQuerySupportPinAdoptionEvidence {
        consumer_name: contract.consumer_name().to_owned(),
        loaded_contract_digest: contract.contract_digest().to_owned(),
        evaluated_requirement_count: contract.requirements().len(),
        observed_row_count: contract.observed_rows().len(),
        schema_version: ForgeQuerySupportPinContractSchemaVersion::current(),
    })
}
