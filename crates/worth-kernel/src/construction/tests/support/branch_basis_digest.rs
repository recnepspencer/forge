use forge_query::facade::{
    compose_construction_branch_basis_preparation_digest, ForgeQueryBranchOptions,
    ForgeQueryPreviewOptions, ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily,
    ForgeQuerySessionLabel, ForgeQueryWorkspace,
};

use crate::construction::intent::PrimitiveConstructionIntent;

pub(crate) fn prepare_branch_basis_digest(
    workspace: &mut ForgeQueryWorkspace,
    intent: &PrimitiveConstructionIntent,
) -> Result<String, ForgeQueryRuntimeError> {
    let family = intent.family();
    let contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)?
        .contract_digest()
        .to_string();
    let preview_admission = {
        let preview = workspace.preview_with_options(
            ForgeQuerySessionLabel::scoped_strs("worth-kernel", [family.as_str(), "preview"])
                .expect("preview label"),
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )?;
        preview.basis_admission().admission_identity().clone()
    };
    let branch_admission = {
        let branch = workspace.branch_with_options(
            ForgeQuerySessionLabel::scoped_strs("worth-kernel", [family.as_str(), "branch"])
                .expect("branch label"),
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )?;
        branch.basis_admission().admission_identity().clone()
    };

    Ok(compose_construction_branch_basis_preparation_digest(
        family.as_str(),
        contract_digest.as_str(),
        &preview_admission,
        &branch_admission,
    ))
}
