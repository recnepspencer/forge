use crate::capability::SurfaceId;
use crate::runtime::{
    WorthUiPrimitiveProofDenial, WorthUiProjectionDependencyDeclaration,
    WorthUiProjectionDependencySet, WorthUiProjectionFamily, WorthUiProjectionIdentity,
    WorthUiRuntimeFactId, WorthUiValidatedProjectionDependencyContract,
};

pub(super) fn primitive_dependency_contract(
    surface_id: &SurfaceId,
) -> Result<WorthUiValidatedProjectionDependencyContract, WorthUiPrimitiveProofDenial> {
    let dependencies = WorthUiProjectionDependencySet::empty()
        .depends_on(WorthUiRuntimeFactId::authored_surface_props(
            surface_id.as_str(),
        ))
        .depends_on(WorthUiRuntimeFactId::primitive_content(surface_id.as_str()))
        .depends_on(WorthUiRuntimeFactId::primitive_container(
            surface_id.as_str(),
        ))
        .depends_on(WorthUiRuntimeFactId::primitive_measurement(
            surface_id.as_str(),
        ))
        .depends_on(WorthUiRuntimeFactId::primitive_appearance(
            surface_id.as_str(),
        ))
        .depends_on(WorthUiRuntimeFactId::primitive_appearance_state(
            surface_id.as_str(),
        ))
        .depends_on(WorthUiRuntimeFactId::primitive_interaction(
            surface_id.as_str(),
        ))
        .depends_on(WorthUiRuntimeFactId::primitive_motion(surface_id.as_str()))
        .depends_on(WorthUiRuntimeFactId::primitive_flow_layout(
            surface_id.as_str(),
        ))
        .depends_on(WorthUiRuntimeFactId::primitive_event_geometry(
            surface_id.as_str(),
        ));
    let declaration = WorthUiProjectionDependencyDeclaration::from_set(dependencies);
    WorthUiValidatedProjectionDependencyContract::admit(
        WorthUiProjectionIdentity::runtime(format!("primitive-proof:{}", surface_id.as_str())),
        WorthUiProjectionFamily::PrimitiveProof,
        declaration,
    )
    .map_err(|_| WorthUiPrimitiveProofDenial::EmptyDependencyContract {
        surface_id: surface_id.as_str().to_owned(),
    })
}
