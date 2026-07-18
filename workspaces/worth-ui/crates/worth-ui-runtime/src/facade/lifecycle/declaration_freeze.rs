use crate::declaration::{
    UiDeclarationArtifact, UiDeclarationGraphHandoff, UiDeclarationGraphHandoffDenial,
    UiDeclarationLowering,
};
use crate::runtime::WorthUiSourceBackedDeclarationWitness;
use crate::runtime::WorthUiSourceBackedDslPackage;
use worth_ui_dsl::WorthUiDslPackage;

pub(crate) fn lower_declaration_artifacts(
    dsl_package: &WorthUiDslPackage,
) -> Vec<UiDeclarationArtifact> {
    lower_declared_artifacts(dsl_package)
}

pub(crate) fn lower_source_backed_declaration_artifacts(
    declaration_source: &WorthUiSourceBackedDslPackage,
) -> Vec<UiDeclarationArtifact> {
    let mut declaration_artifacts = lower_declared_artifacts(declaration_source.dsl_package());
    admit_source_backed_mosaic_sizing_contracts(
        &mut declaration_artifacts,
        declaration_source.declaration_witness(),
    );
    declaration_artifacts
}

fn lower_declared_artifacts(dsl_package: &WorthUiDslPackage) -> Vec<UiDeclarationArtifact> {
    dsl_package
        .runtime_lowering_receipts()
        .iter()
        .cloned()
        .map(UiDeclarationLowering::lower)
        .collect()
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
) {
    for artifact in declaration_artifacts {
        let provenance = artifact.provenance().source_provenance();
        if let Some(claims) = source_backed_declaration_witness
            .claims_for(provenance.module_path(), provenance.declaration_index())
        {
            artifact.admit_source_backed_mosaic_sizing_contract_id(
                claims.mosaic_sizing_contract_id().clone(),
            );
            artifact.admit_source_backed_mosaic_membership_name(claims.mosaic_membership_name());
            artifact.admit_source_backed_measurement_constraint_modifier(
                claims.measurement_constraint_modifier(),
            );
        }
    }
}
