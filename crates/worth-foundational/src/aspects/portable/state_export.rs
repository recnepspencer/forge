use super::{
    contract_for_export, patch_export::validation_input, PortableAspectContractBasis,
    PortableAspectContractLookup, PortableAspectExportDenial, PortableRecordAspectState,
    PortableRecordAspectStateEntry,
};
use crate::aspects::AuthoritativeRecordAspectState;

pub fn export_portable_record_aspect_state(
    state: &AuthoritativeRecordAspectState,
    contracts: &impl PortableAspectContractLookup,
) -> Result<PortableRecordAspectState, PortableAspectExportDenial> {
    let mut entries = Vec::with_capacity(state.aspects().len());
    for (_, value) in state.aspects().entries() {
        let contract = contract_for_export(
            value.key(),
            value.contract_identity(),
            value.contract_revision(),
            contracts,
        )?;
        entries.push(PortableRecordAspectStateEntry::new(
            PortableAspectContractBasis::from_contract(&contract),
            validation_input(value),
        ));
    }
    Ok(PortableRecordAspectState::new(entries))
}
