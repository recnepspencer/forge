use super::ValidationAuthoredReloadEditDenial;

pub(super) fn set_surface_prop(
    source_text: &str,
    surface_id: &str,
    prop_key: &str,
    authored_value: &str,
) -> Result<String, ValidationAuthoredReloadEditDenial> {
    let block = surface_block_range(source_text, surface_id)?;
    let mut next_source = source_text.to_owned();
    if let Some(line) = surface_prop_line_range(source_text, block.clone(), prop_key) {
        next_source.replace_range(line, &format!("    {prop_key} {authored_value}"));
        return Ok(next_source);
    }
    next_source.insert_str(block.end, &format!("\n    {prop_key} {authored_value}"));
    Ok(next_source)
}

pub(super) fn remove_surface_prop(
    source_text: &str,
    surface_id: &str,
    prop_key: &str,
) -> Result<String, ValidationAuthoredReloadEditDenial> {
    let block = surface_block_range(source_text, surface_id)?;
    let Some(mut line) = surface_prop_line_range(source_text, block, prop_key) else {
        return Err(surface_prop_line_not_found(surface_id, prop_key));
    };
    if source_text[line.end..].starts_with('\n') {
        line.end += 1;
    }
    let mut next_source = source_text.to_owned();
    next_source.replace_range(line, "");
    Ok(next_source)
}

fn surface_block_range(
    source_text: &str,
    surface_id: &str,
) -> Result<std::ops::Range<usize>, ValidationAuthoredReloadEditDenial> {
    let declaration = format!("surface {surface_id} {{");
    let Some(block_start) = source_text.find(&declaration) else {
        return Err(
            ValidationAuthoredReloadEditDenial::SurfaceDeclarationNotFound {
                surface_id: surface_id.to_owned(),
            },
        );
    };
    let content_start = block_start + declaration.len();
    let Some(block_end) = source_text[content_start..].find("\n}") else {
        return Err(ValidationAuthoredReloadEditDenial::SurfaceBlockMalformed {
            surface_id: surface_id.to_owned(),
        });
    };
    Ok(content_start..content_start + block_end)
}

fn surface_prop_line_range(
    source_text: &str,
    block: std::ops::Range<usize>,
    prop_key: &str,
) -> Option<std::ops::Range<usize>> {
    let prop_prefix = format!("    {prop_key} ");
    let block_text = &source_text[block.clone()];
    let line_offset = block_text
        .lines()
        .scan(block.start, |offset, line| {
            let start = *offset;
            *offset += line.len() + 1;
            Some((start, line))
        })
        .find_map(|(start, line)| line.starts_with(&prop_prefix).then_some(start))?;
    let line_end = source_text[line_offset..]
        .find('\n')
        .map(|offset| line_offset + offset)
        .unwrap_or(source_text.len());
    Some(line_offset..line_end)
}

fn surface_prop_line_not_found(
    surface_id: &str,
    prop_key: &str,
) -> ValidationAuthoredReloadEditDenial {
    ValidationAuthoredReloadEditDenial::SurfacePropLineNotFound {
        surface_id: surface_id.to_owned(),
        prop_key: prop_key.to_owned(),
    }
}
