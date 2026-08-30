mod chunk;
mod manifest;

pub(in crate::integrity_ingress) use chunk::IntegrityAdmittedExtentChunkFrame;
pub(in crate::integrity_ingress) use manifest::IntegrityAdmittedExtentManifest;

#[cfg(test)]
pub(super) fn owner_valid_compile_contracts() {
    manifest::owner_valid_compile_contract();
    chunk::owner_valid_compile_contract();
}
