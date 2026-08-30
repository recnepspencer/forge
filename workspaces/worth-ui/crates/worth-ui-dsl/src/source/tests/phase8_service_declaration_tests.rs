use std::path::PathBuf;

use crate::{
    WorthUiAuthoredSourceInput, WorthUiDslCompileDiagnosticCode, WorthUiDslCompiler,
    WorthUiServiceDeclarationMeaning, WorthUiServiceFamily,
};

#[test]
fn representative_service_dsl_lowers_to_typed_source_linked_meaning() {
    let package = compile(
        r#"
        portal completion_menu {
          anchor editor_input
          layer transient
          dismiss escape outside_press accepted_selection anchor_gone
          focus first_enabled restore
          motion system_popover
        }

        selection results_selection {
          mode multiple
          identity result_key
          preserve stable_key
        }

        command show_palette {
          shortcut Primary+Shift+P
          scope application
        }
        "#,
    )
    .expect("representative service source compiles");
    let declarations = package.service_declarations().collect::<Vec<_>>();

    assert_eq!(declarations.len(), 3);
    assert_eq!(
        declarations
            .iter()
            .map(|(declaration, _)| declaration.family())
            .collect::<Vec<_>>(),
        [
            WorthUiServiceFamily::Portal,
            WorthUiServiceFamily::Selection,
            WorthUiServiceFamily::CommandRouting,
        ]
    );
    let WorthUiServiceDeclarationMeaning::Command(command) = declarations[2].0 else {
        panic!("third declaration is the command")
    };
    assert_eq!(command.shortcut().len(), 1);
    assert_eq!(
        command.shortcut()[0].modifiers(),
        [
            crate::WorthUiCommandModifier::Primary,
            crate::WorthUiCommandModifier::Shift,
        ]
    );
    assert_eq!(
        command.shortcut()[0].key(),
        crate::WorthUiCommandKey::Letter('P')
    );
    assert_eq!(declarations[2].1.module_path(), "app/main.wui");
}

#[test]
fn invalid_service_combination_has_a_source_span_and_lawful_repair() {
    let report = compile(
        r#"
        selection results_selection {
          mode multiple
          identity result_key
          preserve row_index
        }
        "#,
    )
    .expect_err("row index cannot become stable selection identity");
    let [diagnostic] = report.diagnostics() else {
        panic!("one invalid service declaration produces one diagnostic")
    };

    assert_eq!(
        diagnostic.identity().code(),
        WorthUiDslCompileDiagnosticCode::InvalidServiceDeclaration
    );
    assert_eq!(
        diagnostic.identity().span().unwrap().module_id(),
        "app/main.wui"
    );
    assert!(diagnostic.message().contains("lawful repair"));
    assert!(diagnostic.message().contains("stable_key"));
}

#[test]
fn portal_rejects_unowned_focus_and_motion_vocabulary() {
    for (focus, motion, observed, repair) in [
        (
            "first_enabled autofocus",
            "system_popover",
            "autofocus",
            "first_enabled",
        ),
        ("first_enabled", "springy", "springy", "system_popover"),
    ] {
        let source = format!(
            "portal completion_menu {{ anchor editor_input; layer transient; dismiss escape; \
             focus {focus}; motion {motion}; }}"
        );
        let report = compile(&source).expect_err("unowned portal vocabulary is rejected");
        let [diagnostic] = report.diagnostics() else {
            panic!("one invalid portal clause produces one diagnostic")
        };
        assert_eq!(
            diagnostic.identity().code(),
            WorthUiDslCompileDiagnosticCode::InvalidServiceDeclaration
        );
        assert!(diagnostic.message().contains(observed));
        assert!(diagnostic.message().contains(repair));
    }
}

#[test]
fn invalid_command_shortcuts_are_source_linked_and_teach_lawful_repairs() {
    for (shortcut, observed, repair) in [
        (
            "Primary+Control+S",
            "Primary with Control or Meta",
            "use Primary alone",
        ),
        (
            "Primary+Shift+Shift+S",
            "duplicate modifier",
            "declare each modifier once",
        ),
        (
            "Primary+K then Primary+C then Primary+S",
            "more than two strokes",
            "use one or two strokes",
        ),
    ] {
        assert_invalid_command_shortcut(shortcut, observed, repair);
    }

    let report = compile(
        r#"
        command missing_key {
          shortcut
          scope application
        }
        "#,
    )
    .expect_err("a command shortcut requires a key");
    let [diagnostic] = report.diagnostics() else {
        panic!("one missing shortcut key produces one diagnostic")
    };
    assert_eq!(
        diagnostic.identity().code(),
        WorthUiDslCompileDiagnosticCode::InvalidServiceDeclaration
    );
    assert_eq!(
        diagnostic.identity().span().unwrap().module_id(),
        "app/main.wui"
    );
    assert!(diagnostic.message().contains("declare a key"));
}

#[test]
fn active_region_command_scope_is_source_linked_and_deferred_honestly() {
    let report = compile(
        r#"
        command region_action {
          shortcut Primary+R
          scope active_region
        }
        "#,
    )
    .expect_err("active-region scope requires runtime authority that does not exist yet");
    let [diagnostic] = report.diagnostics() else {
        panic!("one unsupported command scope produces one diagnostic")
    };

    assert_eq!(
        diagnostic.identity().code(),
        WorthUiDslCompileDiagnosticCode::InvalidServiceDeclaration
    );
    assert_eq!(
        diagnostic.identity().span().unwrap().module_id(),
        "app/main.wui"
    );
    assert!(diagnostic
        .message()
        .contains("active-region runtime authority"));
    assert!(diagnostic.message().contains("focused_control"));
}

#[test]
fn every_service_family_rejects_duplicate_and_unconsumed_clauses() {
    let hostile_sources = [
        "portal p { anchor a layer transient layer modal dismiss escape focus first_enabled motion system_popover }",
        "portal p { anchor a layer transient dismiss escape focus first_enabled motion system_popover mystery }",
        "focus f { scope workbench scope portal }",
        "focus f { scope workbench restore mystery }",
        "motion m { reduced system_respecting reduced system_respecting }",
        "motion m { reduced system_respecting mystery }",
        "command c { shortcut Primary+K scope application scope surface }",
        "command c { shortcut Primary+K scope application mystery }",
        "scroll s { nested anchor clamp anchor stable_key }",
        "scroll s { nested anchor clamp mystery }",
        "selection s { mode single mode multiple identity item_key preserve stable_key }",
        "selection s { mode single identity item_key preserve stable_key mystery }",
    ];
    for source in hostile_sources {
        let report = compile(source).expect_err("duplicate and unconsumed clauses are rejected");
        let [diagnostic] = report.diagnostics() else {
            panic!("one hostile service declaration produces one diagnostic: {source}")
        };
        assert_eq!(
            diagnostic.identity().code(),
            WorthUiDslCompileDiagnosticCode::InvalidServiceDeclaration
        );
        assert!(diagnostic.message().contains("lawful repair"));
    }
}

#[test]
fn scoped_command_requires_and_preserves_an_exact_authored_binding() {
    let report = compile("command c { shortcut Primary+K scope focused_control }")
        .expect_err("focused-control routing without an exact binding is rejected");
    assert!(report.diagnostics()[0]
        .message()
        .contains("command binding"));

    let package =
        compile("command c { shortcut Primary+K scope focused_control binding editor_control }")
            .expect("an exact focused-control binding is admitted");
    let (declaration, _) = package.service_declarations().next().unwrap();
    let WorthUiServiceDeclarationMeaning::Command(command) = declaration else {
        panic!("the declaration remains typed command meaning")
    };
    assert_eq!(command.scope_identity(), Some("editor_control"));
}

fn assert_invalid_command_shortcut(shortcut: &str, observed: &str, repair: &str) {
    let report = compile(&format!(
        "command invalid {{ shortcut {shortcut}; scope application; }}"
    ))
    .expect_err("invalid command shortcut is denied");
    let [diagnostic] = report.diagnostics() else {
        panic!("one invalid shortcut produces one diagnostic")
    };
    assert_eq!(
        diagnostic.identity().code(),
        WorthUiDslCompileDiagnosticCode::InvalidServiceDeclaration
    );
    assert_eq!(
        diagnostic.identity().span().unwrap().module_id(),
        "app/main.wui"
    );
    assert!(diagnostic.message().contains(observed));
    assert!(diagnostic.message().contains(repair));
}

fn compile(
    source: &str,
) -> Result<crate::WorthUiSealedSemanticPackage, crate::WorthUiDslCompileReport> {
    WorthUiDslCompiler::compile_source(
        WorthUiAuthoredSourceInput::rooted_at(PathBuf::from("workspace"))
            .with_module("app/main.wui", source),
    )
}
