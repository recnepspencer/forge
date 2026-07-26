use std::path::PathBuf;

use crate::source::{
    tokenize_module_source, WorthUiSourceModuleId, WorthUiSourceToken, WorthUiSourceTokenKind,
};

pub(super) fn scan_top_level_import_paths(
    module_id: &WorthUiSourceModuleId,
    source_text: &str,
) -> Vec<PathBuf> {
    let Ok(tokens) = tokenize_module_source(module_id, source_text) else {
        return Vec::new();
    };
    import_paths_from_tokens(&tokens)
}

fn import_paths_from_tokens(tokens: &[WorthUiSourceToken]) -> Vec<PathBuf> {
    let mut import_paths = Vec::new();
    let mut block_depth = 0usize;

    for (index, token) in tokens.iter().enumerate() {
        match token.kind() {
            WorthUiSourceTokenKind::LeftBrace => block_depth += 1,
            WorthUiSourceTokenKind::RightBrace => {
                block_depth = block_depth.saturating_sub(1);
            }
            WorthUiSourceTokenKind::KeywordImport if block_depth == 0 => {
                if let (
                    Some(WorthUiSourceTokenKind::StringLiteral(path)),
                    Some(WorthUiSourceTokenKind::Semicolon),
                ) = (
                    tokens.get(index + 1).map(WorthUiSourceToken::kind),
                    tokens.get(index + 2).map(WorthUiSourceToken::kind),
                ) {
                    import_paths.push(PathBuf::from(path));
                }
            }
            _ => {}
        }
    }

    import_paths
}
