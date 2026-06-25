use crate::runtime::{
    WorthUiAccessibilityAssociationKind,
    WorthUiAuthoredCompositionAccessibilityAssociationDeclaration,
    WorthUiAuthoredCompositionDeclaration, WorthUiAuthoredLiveViewParseDenial,
    WorthUiPrimitiveSourceSpan,
};

use super::parse_denial;

pub(super) fn parse_accessibility_block<'a, I>(
    source_lines: &mut std::iter::Peekable<I>,
    composition: &mut WorthUiAuthoredCompositionDeclaration,
) -> Result<(), WorthUiAuthoredLiveViewParseDenial>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    loop {
        let Some((index, raw_line)) = source_lines.next() else {
            return Err(WorthUiAuthoredLiveViewParseDenial::new(
                0,
                "unterminated accessibility block",
            ));
        };
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "}" {
            return Ok(());
        }
        parse_accessibility_association(index, line, composition)?;
    }
}

fn parse_accessibility_association(
    line_index: usize,
    line: &str,
    composition: &mut WorthUiAuthoredCompositionDeclaration,
) -> Result<(), WorthUiAuthoredLiveViewParseDenial> {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 4 || parts[2] != "->" {
        return Err(parse_denial(
            line_index,
            "accessibility relationship syntax is label|description|error <source> -> <target>",
        ));
    }
    let kind = parse_accessibility_association_kind(line_index, parts[0])?;
    composition.push_accessibility_association(
        WorthUiAuthoredCompositionAccessibilityAssociationDeclaration::spanned(
            kind,
            parts[1],
            parts[3],
            WorthUiPrimitiveSourceSpan::new(line_index + 1, line_index + 1),
        ),
    );
    Ok(())
}

fn parse_accessibility_association_kind(
    line_index: usize,
    token: &str,
) -> Result<WorthUiAccessibilityAssociationKind, WorthUiAuthoredLiveViewParseDenial> {
    match token {
        "label" => Ok(WorthUiAccessibilityAssociationKind::Label),
        "description" => Ok(WorthUiAccessibilityAssociationKind::Description),
        "error" => Ok(WorthUiAccessibilityAssociationKind::Error),
        _ => Err(parse_denial(
            line_index,
            "accessibility relationship kind must be label, description, or error",
        )),
    }
}
