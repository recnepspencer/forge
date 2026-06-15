use std::collections::BTreeMap;

use crate::source::{
    WorthUiParsedAuthoringDeclaration, WorthUiParsedPageDeclaration,
    WorthUiParsedSourceDeclaration, WorthUiParsedSourcePackage,
};

use super::{
    WorthUiAuthoringEntryDiagnostic, WorthUiAuthoringEntryDiagnosticCode,
    WorthUiAuthoringEntryReport,
};

#[derive(Clone, Copy)]
pub(crate) struct WorthUiNamedAuthoringDecl<'a> {
    pub(crate) declaration: &'a WorthUiParsedAuthoringDeclaration,
}

#[derive(Clone, Copy)]
pub(crate) struct WorthUiPageDecl<'a> {
    pub(crate) declaration: &'a WorthUiParsedPageDeclaration,
}

pub(crate) struct WorthUiAuthoringSymbolTable<'a> {
    app_declarations: Vec<&'a WorthUiParsedAuthoringDeclaration>,
    workspaces: BTreeMap<&'a str, WorthUiNamedAuthoringDecl<'a>>,
    pages: BTreeMap<&'a str, WorthUiPageDecl<'a>>,
    runtimes: BTreeMap<&'a str, WorthUiNamedAuthoringDecl<'a>>,
    layouts: BTreeMap<&'a str, WorthUiNamedAuthoringDecl<'a>>,
    contents: BTreeMap<&'a str, WorthUiNamedAuthoringDecl<'a>>,
    appearances: BTreeMap<&'a str, WorthUiNamedAuthoringDecl<'a>>,
}

impl<'a> WorthUiAuthoringSymbolTable<'a> {
    pub(crate) fn build(
        parsed_package: &'a WorthUiParsedSourcePackage,
    ) -> Result<Self, WorthUiAuthoringEntryReport> {
        let mut table = Self {
            app_declarations: Vec::new(),
            workspaces: BTreeMap::new(),
            pages: BTreeMap::new(),
            runtimes: BTreeMap::new(),
            layouts: BTreeMap::new(),
            contents: BTreeMap::new(),
            appearances: BTreeMap::new(),
        };
        let mut diagnostics = Vec::new();

        for module_id in parsed_package.module_ids() {
            let module = parsed_package
                .module(module_id)
                .expect("parsed package should contain canonical module");
            for declaration in module.declarations() {
                match declaration {
                    WorthUiParsedSourceDeclaration::App(authoring) => {
                        table.app_declarations.push(authoring);
                    }
                    WorthUiParsedSourceDeclaration::Workspace(authoring) => {
                        insert_named(
                            &mut table.workspaces,
                            authoring.name_text(),
                            authoring,
                            &mut diagnostics,
                        );
                    }
                    WorthUiParsedSourceDeclaration::Page(page) => {
                        insert_page(&mut table.pages, page, &mut diagnostics);
                    }
                    WorthUiParsedSourceDeclaration::Runtime(authoring) => {
                        insert_named(
                            &mut table.runtimes,
                            authoring.name_text(),
                            authoring,
                            &mut diagnostics,
                        );
                    }
                    WorthUiParsedSourceDeclaration::Layout(authoring) => {
                        insert_named(
                            &mut table.layouts,
                            authoring.name_text(),
                            authoring,
                            &mut diagnostics,
                        );
                    }
                    WorthUiParsedSourceDeclaration::Content(authoring) => {
                        insert_named(
                            &mut table.contents,
                            authoring.name_text(),
                            authoring,
                            &mut diagnostics,
                        );
                    }
                    WorthUiParsedSourceDeclaration::Appearance(authoring) => {
                        insert_named(
                            &mut table.appearances,
                            authoring.name_text(),
                            authoring,
                            &mut diagnostics,
                        );
                    }
                    _ => {}
                }
            }
        }

        if table.app_declarations.len() > 1 {
            for declaration in table.app_declarations.iter().skip(1) {
                diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
                    WorthUiAuthoringEntryDiagnosticCode::MultipleAppDeclarations,
                    "authoring hierarchy currently allows exactly one app declaration",
                    declaration.span().clone(),
                ));
            }
        }

        if diagnostics.is_empty() {
            Ok(table)
        } else {
            Err(WorthUiAuthoringEntryReport::new(diagnostics))
        }
    }

    pub(crate) fn has_authoring_roots(&self) -> bool {
        !self.app_declarations.is_empty() || !self.workspaces.is_empty() || !self.pages.is_empty()
    }

    pub(crate) fn app_declaration(&self) -> Option<&'a WorthUiParsedAuthoringDeclaration> {
        self.app_declarations.first().copied()
    }

    pub(crate) fn workspaces(&self) -> &BTreeMap<&'a str, WorthUiNamedAuthoringDecl<'a>> {
        &self.workspaces
    }

    pub(crate) fn pages(&self) -> &BTreeMap<&'a str, WorthUiPageDecl<'a>> {
        &self.pages
    }

    pub(crate) fn runtimes(&self) -> &BTreeMap<&'a str, WorthUiNamedAuthoringDecl<'a>> {
        &self.runtimes
    }

    pub(crate) fn layouts(&self) -> &BTreeMap<&'a str, WorthUiNamedAuthoringDecl<'a>> {
        &self.layouts
    }

    pub(crate) fn contents(&self) -> &BTreeMap<&'a str, WorthUiNamedAuthoringDecl<'a>> {
        &self.contents
    }

    pub(crate) fn appearances(&self) -> &BTreeMap<&'a str, WorthUiNamedAuthoringDecl<'a>> {
        &self.appearances
    }
}

fn insert_named<'a>(
    target: &mut BTreeMap<&'a str, WorthUiNamedAuthoringDecl<'a>>,
    name_text: &'a str,
    declaration: &'a WorthUiParsedAuthoringDeclaration,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) {
    if target
        .insert(name_text, WorthUiNamedAuthoringDecl { declaration })
        .is_some()
    {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::DuplicateDeclarationName,
            format!("duplicate authoring declaration '{name_text}'"),
            declaration.span().clone(),
        ));
    }
}

fn insert_page<'a>(
    target: &mut BTreeMap<&'a str, WorthUiPageDecl<'a>>,
    page: &'a WorthUiParsedPageDeclaration,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) {
    if target
        .insert(page.name_text(), WorthUiPageDecl { declaration: page })
        .is_some()
    {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::DuplicateDeclarationName,
            format!("duplicate authoring declaration '{}'", page.name_text()),
            page.span().clone(),
        ));
    }
}
