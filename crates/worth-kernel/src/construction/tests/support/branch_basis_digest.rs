use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryBranchOptions, ForgeQueryEffectPolicy,
    ForgeQueryPreviewOptions, ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily,
    ForgeQuerySessionLabel, ForgeQueryWorkspace,
};

use crate::construction::digest::digest_owned_parts;
use crate::construction::intent::PrimitiveConstructionIntent;

fn basis_admission_digest(
    label: &str,
    effect_policy: ForgeQueryEffectPolicy,
    authority_lane: ForgeQueryAuthorityLane,
    evidence: &[String],
) -> String {
    digest_owned_parts(&[
        label.to_string(),
        effect_policy.to_string(),
        authority_lane.to_string(),
        evidence.join("|"),
    ])
}

pub(crate) fn prepare_branch_basis_digest(
    workspace: &mut ForgeQueryWorkspace,
    intent: &PrimitiveConstructionIntent,
) -> Result<String, ForgeQueryRuntimeError> {
    let family = intent.family();
    let contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)?
        .contract_digest()
        .to_string();
    let (preview_label, preview_effect_policy, preview_authority_lane, preview_evidence) = {
        let preview = workspace.preview_with_options(
            ForgeQuerySessionLabel::scoped_strs("worth-kernel", [family.as_str(), "preview"])
                .expect("preview label"),
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )?;
        let preview_basis = preview.basis_admission();
        (
            preview_basis.label().to_string(),
            preview_basis.effect_policy(),
            preview_basis.authority_lane(),
            preview_basis.evidence().to_vec(),
        )
    };
    let branch = workspace.branch_with_options(
        ForgeQuerySessionLabel::scoped_strs("worth-kernel", [family.as_str(), "branch"])
            .expect("branch label"),
        ForgeQueryBranchOptions::sandboxed_write_intent(),
    )?;
    let branch_basis = branch.basis_admission();

    Ok(digest_owned_parts(&[
        family.as_str().to_string(),
        contract_digest,
        basis_admission_digest(
            &preview_label,
            preview_effect_policy,
            preview_authority_lane,
            &preview_evidence,
        ),
        basis_admission_digest(
            branch_basis.label(),
            branch_basis.effect_policy(),
            branch_basis.authority_lane(),
            branch_basis.evidence(),
        ),
    ]))
}
