use super::{
    exact_contract_for_export, PortableAspectContractBasis, PortableAspectContractLookup,
    PortableAspectExportDenial, PortableAspectFieldSet, PortableAspectPatchOperation,
    PortableRecordAspectPatch,
};
use crate::aspects::{
    AuthoritativeRecordAspectPatch, ContractValidatedAspectValue, ContractValidatedAspectValueView,
    ContractValidationInput,
};

pub fn export_portable_record_aspect_patch(
    patch: &AuthoritativeRecordAspectPatch,
    contracts: &impl PortableAspectContractLookup,
) -> Result<PortableRecordAspectPatch, PortableAspectExportDenial> {
    let mut operations = Vec::new();
    for (_, value) in patch.whole_aspect_sets() {
        let contract = super::contract_for_export(
            value.key(),
            value.contract_identity(),
            value.contract_revision(),
            contracts,
        )?;
        operations.push(PortableAspectPatchOperation::SetWhole {
            basis: PortableAspectContractBasis::from_contract(&contract),
            value: validation_input(value),
        });
    }
    for (_, clear_contract) in patch.whole_aspect_clear_contracts() {
        let contract = exact_contract_for_export(clear_contract, contracts)?;
        operations.push(PortableAspectPatchOperation::ClearWhole {
            basis: PortableAspectContractBasis::from_contract(&contract),
        });
    }
    for (_, field_patch) in patch.field_patches() {
        let contract = exact_contract_for_export(field_patch.contract(), contracts)?;
        operations.push(PortableAspectPatchOperation::PatchFields {
            basis: PortableAspectContractBasis::from_contract(&contract),
            selected_fields: field_patch
                .mask()
                .paths()
                .iter()
                .map(|path| path.fields()[0].clone())
                .collect(),
            field_sets: field_patch
                .field_sets()
                .map(|(field, value)| PortableAspectFieldSet::new(field.clone(), value.clone()))
                .collect(),
            field_clears: field_patch.field_clears().cloned().collect(),
        });
    }
    Ok(PortableRecordAspectPatch::new(operations))
}

pub(super) fn validation_input(value: &ContractValidatedAspectValue) -> ContractValidationInput {
    match value.view() {
        ContractValidatedAspectValueView::Scalar(value) => {
            ContractValidationInput::Scalar(value.clone())
        }
        ContractValidatedAspectValueView::Struct(value) => {
            ContractValidationInput::Struct(value.clone())
        }
    }
}
