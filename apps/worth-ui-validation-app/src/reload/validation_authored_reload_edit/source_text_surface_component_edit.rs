use super::ValidationAuthoredReloadEditDenial;

pub(super) fn repoint_surface_component(
    source_text: &str,
    surface_id: &str,
    component_id: &str,
) -> Result<String, ValidationAuthoredReloadEditDenial> {
    let declaration = format!("surface {surface_id} {{");
    let Some(surface_offset) = source_text.find(&declaration) else {
        return Err(
            ValidationAuthoredReloadEditDenial::SurfaceDeclarationNotFound {
                surface_id: surface_id.to_owned(),
            },
        );
    };
    let component_prefix = "    component ";
    let component_search_offset = surface_offset + declaration.len();
    let Some(component_offset) = source_text[component_search_offset..].find(component_prefix)
    else {
        return Err(
            ValidationAuthoredReloadEditDenial::SurfaceComponentLineNotFound {
                surface_id: surface_id.to_owned(),
            },
        );
    };
    let component_start = component_search_offset + component_offset + component_prefix.len();
    let Some(component_end) = source_text[component_start..].find('\n') else {
        return Err(
            ValidationAuthoredReloadEditDenial::SurfaceComponentLineNotFound {
                surface_id: surface_id.to_owned(),
            },
        );
    };
    let component_end = component_start + component_end;
    let mut next_source = source_text.to_owned();
    next_source.replace_range(component_start..component_end, component_id);
    Ok(next_source)
}
