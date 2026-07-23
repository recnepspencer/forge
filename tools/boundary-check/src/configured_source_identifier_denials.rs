use std::{collections::BTreeSet, fs, path::Path};

use syn::visit::Visit;

use crate::config::SourceIdentifierDenialConfig;
use crate::diagnostics::{Diagnostic, DiagnosticCode};

pub(crate) fn validate_source_identifier_denials(
    workspace: &Path,
    rules: &[SourceIdentifierDenialConfig],
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for rule in rules {
        visit_rust_sources(workspace, &workspace.join(&rule.root), rule, &mut diagnostics);
    }
    diagnostics
}

fn visit_rust_sources(
    workspace: &Path,
    path: &Path,
    rule: &SourceIdentifierDenialConfig,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(error) => {
            diagnostics.push(unreadable_source_diagnostic(workspace, path, &error));
            return;
        }
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                diagnostics.push(unreadable_source_diagnostic(workspace, path, &error));
                continue;
            }
        };
        let path = entry.path();
        if path.is_dir() {
            visit_rust_sources(workspace, &path, rule, diagnostics);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            let source = match fs::read_to_string(&path) {
                Ok(source) => source,
                Err(error) => {
                    diagnostics.push(unreadable_source_diagnostic(workspace, &path, &error));
                    continue;
                }
            };
            diagnostics.extend(diagnostics_for_source(
                &path.strip_prefix(workspace).unwrap_or(&path).display().to_string(),
                &source,
                rule,
            ));
        }
    }
}

fn unreadable_source_diagnostic(workspace: &Path, path: &Path, error: &std::io::Error) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::Bc2001BandDependencyViolation,
        &path
            .strip_prefix(workspace)
            .unwrap_or(path)
            .display()
            .to_string(),
        format!("configured source boundary could not be read: {error}"),
    )
}

fn diagnostics_for_source(
    path: &str,
    source: &str,
    rule: &SourceIdentifierDenialConfig,
) -> Vec<Diagnostic> {
    let Ok(file) = syn::parse_file(source) else {
        return vec![Diagnostic::new(
            DiagnosticCode::Bc2001BandDependencyViolation,
            path,
            "source could not be parsed for forbidden authority identifiers",
        )];
    };
    let mut visitor = ForbiddenIdentifierVisitor {
        forbidden: &rule.forbidden_identifiers,
        found: BTreeSet::new(),
    };
    visitor.visit_file(&file);
    visitor
        .found
        .into_iter()
        .map(|identifier| {
            Diagnostic::new(
                DiagnosticCode::Bc2001BandDependencyViolation,
                path,
                format!(
                    "source contains forbidden identifier `{identifier}`: {}",
                    rule.guidance
                ),
            )
        })
        .collect()
}

struct ForbiddenIdentifierVisitor<'a> {
    forbidden: &'a [String],
    found: BTreeSet<String>,
}

impl Visit<'_> for ForbiddenIdentifierVisitor<'_> {
    fn visit_ident(&mut self, identifier: &proc_macro2::Ident) {
        let identifier = identifier.to_string();
        if self.forbidden.iter().any(|denied| denied == &identifier) {
            self.found.insert(identifier);
        }
    }

    fn visit_macro(&mut self, node: &syn::Macro) {
        syn::visit::visit_macro(self, node);
        self.visit_token_stream(node.tokens.clone());
    }
}

impl ForbiddenIdentifierVisitor<'_> {
    fn visit_token_stream(&mut self, tokens: proc_macro2::TokenStream) {
        for token in tokens {
            match token {
                proc_macro2::TokenTree::Ident(identifier) => self.visit_ident(&identifier),
                proc_macro2::TokenTree::Group(group) => self.visit_token_stream(group.stream()),
                proc_macro2::TokenTree::Punct(_) | proc_macro2::TokenTree::Literal(_) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        diagnostics_for_source, validate_source_identifier_denials, SourceIdentifierDenialConfig,
    };

    fn rule() -> SourceIdentifierDenialConfig {
        SourceIdentifierDenialConfig {
            root: "physical_runtime".into(),
            forbidden_identifiers: vec![
                "serde_json".into(),
                "BranchWriterAuthority".into(),
                "MVCC".into(),
            ],
            guidance: "wrong authority layer".into(),
        }
    }

    #[test]
    fn rust_identifiers_are_rejected_without_matching_comments_or_strings() {
        assert_eq!(
            diagnostics_for_source(
                "physical_runtime/work/shortcut.rs",
                "use serde_json::Value; type BranchWriterAuthority = Value;",
                &rule(),
            )
            .len(),
            2
        );
        assert!(diagnostics_for_source(
            "physical_runtime/work/honest.rs",
            "// serde_json BranchWriterAuthority MVCC\nconst NOTE: &str = \"MVCC\";",
            &rule(),
        )
        .is_empty());
        assert_eq!(
            diagnostics_for_source(
                "physical_runtime/work/macro_shortcut.rs",
                "macro_rules! shortcut { () => { struct BranchWriterAuthority; } }",
                &rule(),
            )
            .len(),
            1
        );
    }

    #[test]
    fn terminal_json_compatibility_reexports_are_rejected() {
        let mut rule = rule();
        rule.forbidden_identifiers = vec![
            "project_store_boundary_fact_to_terminal_json".into(),
            "StoreTerminalJsonReadmission".into(),
        ];
        let diagnostics = diagnostics_for_source(
            "physical_runtime/work/json_shortcut.rs",
            "use worth_store_aspect_native::{project_store_boundary_fact_to_terminal_json as project, StoreTerminalJsonReadmission};",
            &rule,
        );
        assert_eq!(diagnostics.len(), 2);
    }

    #[test]
    fn missing_governed_source_root_fails_closed() {
        let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut rule = rule();
        rule.root = "definitely-missing-governed-source-root".into();

        let diagnostics = validate_source_identifier_denials(workspace, &[rule]);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0]
            .message()
            .contains("configured source boundary could not be read"));
    }
}
