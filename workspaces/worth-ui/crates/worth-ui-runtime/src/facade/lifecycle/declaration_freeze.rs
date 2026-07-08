use crate::declaration::{
    UiDeclarationArtifact, UiDeclarationGraphHandoff, UiDeclarationGraphHandoffDenial,
    UiDeclarationLowering,
};
use worth_ui_dsl::WorthUiDslPackage;
use crate::runtime::WorthUiSourceBackedDeclarationWitness;

pub(crate) fn lower_declaration_artifacts(
    dsl_package: &WorthUiDslPackage,
    source_backed_declaration_witness: Option<&WorthUiSourceBackedDeclarationWitness>,
) -> Vec<UiDeclarationArtifact> {
    let mut declaration_artifacts = dsl_package
        .runtime_lowering_receipts()
        .iter()
        .cloned()
        .map(UiDeclarationLowering::lower)
        .collect::<Vec<_>>();
    if let Some(source_backed_declaration_witness) = source_backed_declaration_witness {
        admit_source_backed_mosaic_sizing_contracts(
            &mut declaration_artifacts,
            source_backed_declaration_witness,
        )
        .expect(
            "freeze path must deny source-backed declaration authority drift before graph handoff",
        );
    }
    declaration_artifacts
}

pub(crate) fn lower_graph_handoffs(
    declaration_artifacts: &[UiDeclarationArtifact],
) -> Result<Vec<UiDeclarationGraphHandoff>, UiDeclarationGraphHandoffDenial> {
    declaration_artifacts
        .iter()
        .map(UiDeclarationArtifact::graph_handoff)
        .collect()
}

fn admit_source_backed_mosaic_sizing_contracts(
    declaration_artifacts: &mut [UiDeclarationArtifact],
    source_backed_declaration_witness: &WorthUiSourceBackedDeclarationWitness,
) -> Result<(), UiDeclarationGraphHandoffDenial> {
    for artifact in declaration_artifacts {
        let provenance = artifact.provenance().source_provenance();
        if let Some(claims) = source_backed_declaration_witness
            .claims_for(provenance.module_path(), provenance.declaration_index())
        {
            artifact.admit_source_backed_mosaic_sizing_contract_id(
                claims.mosaic_sizing_contract_id().clone(),
            )?;
            artifact
                .admit_source_backed_mosaic_membership_name(claims.mosaic_membership_name());
            artifact.admit_source_backed_measurement_constraint_modifier(
                claims.measurement_constraint_modifier(),
            );
        }
    }

    Ok(())
}