use crate::orientation::CrateOrientation;
use crate::render::render_context;
use std::fs;
use std::path::Path;

pub(crate) fn write_contexts(root: &Path, orientations: &[CrateOrientation]) -> Result<(), String> {
    for orientation in orientations {
        let path = root
            .join(&orientation.relative_path)
            .join("AGENT_CONTEXT.md");
        let rendered = render_context(orientation)?;
        fs::write(&path, rendered).map_err(|e| format!("write {}: {e}", path.display()))?;
    }
    Ok(())
}

pub(crate) fn check_freshness(
    root: &Path,
    orientations: &[CrateOrientation],
) -> Result<(), Vec<String>> {
    let mut stale = Vec::new();
    for orientation in orientations {
        let path = root
            .join(&orientation.relative_path)
            .join("AGENT_CONTEXT.md");
        let expected = match render_context(orientation) {
            Ok(rendered) => rendered,
            Err(error) => return Err(vec![error]),
        };
        let actual = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) => {
                stale.push(format!("{}: read failed: {error}", path.display()));
                continue;
            }
        };
        if normalize_newlines(&actual) != normalize_newlines(&expected) {
            stale.push(format!(
                "{} is stale or hand-edited; rerun agent-context generate",
                path.display()
            ));
        }
    }
    if stale.is_empty() {
        Ok(())
    } else {
        Err(stale)
    }
}

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}

#[cfg(test)]
mod tests {
    use super::normalize_newlines;

    #[test]
    fn normalizes_windows_newlines_for_generated_context_comparison() {
        assert_eq!(normalize_newlines("first\r\nsecond\r\n"), "first\nsecond\n");
    }
}
