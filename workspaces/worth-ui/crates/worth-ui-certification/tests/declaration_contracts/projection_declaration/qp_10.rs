use worth_ui_dsl::{
    WorthUiProjectionCollectionPolicy, WorthUiProjectionCollectionSelection,
    WorthUiProjectionLifecycle, WorthUiProjectionNativeFamily,
    WorthUiRustAuthoredArtifactInputModule,
};

use super::{
    model::{Lifecycle, NativeFamily, RequirementModel},
    support::{capture, compile_file, compile_rust, CompiledRequirements},
};

const DECLARATION: &str = "platform.pulse.status";
const VIEW: &str = "platform.pulse.status";
const FIELD: &str = "status";

#[test]
fn whitespace_declaration_order_and_import_order_are_non_semantic() {
    let expected = vec![
        RequirementModel::collection(
            "a.rows",
            "a.rows",
            "identity",
            &["label", "value"],
            NativeFamily::Text,
            Lifecycle::Snapshot,
            (true, false),
        ),
        RequirementModel::scalar(
            "z.status",
            "z.status",
            "status",
            NativeFamily::Text,
            Lifecycle::Live,
        ),
    ];
    let file_left = capture(&compile_file(FILE_ORDER_LEFT, SUPPORT_MODULES));
    let file_right = capture(&compile_file(FILE_ORDER_RIGHT, SUPPORT_MODULES));
    let rust_left = capture(&compile_rust(rust_modules(false)));
    let rust_right = capture(&compile_rust(rust_modules(true)));

    for compiled in [&file_left, &file_right, &rust_left, &rust_right] {
        assert_eq!(compiled.models, expected);
    }
    assert_eq!(file_left.identities, file_right.identities);
    assert_eq!(file_left.identities, rust_left.identities);
    assert_eq!(file_left.identities, rust_right.identities);
}

#[test]
fn each_semantic_axis_changes_both_dsl_and_rust_canonical_identity() {
    let baseline = scalar_pair(VIEW, FIELD, NativeFamily::Text, Lifecycle::Live);
    assert_pair_matches_model(&baseline);
    let variants = [
        scalar_pair(
            "platform.pulse.other",
            FIELD,
            NativeFamily::Text,
            Lifecycle::Live,
        ),
        scalar_pair(VIEW, "other", NativeFamily::Text, Lifecycle::Live),
        collection_pair(),
        scalar_pair(VIEW, FIELD, NativeFamily::Boolean, Lifecycle::Live),
        scalar_pair(VIEW, FIELD, NativeFamily::Text, Lifecycle::Snapshot),
    ];

    for variant in variants {
        assert_pair_matches_model(&variant);
        assert_ne!(variant.model, baseline.model);
        assert_ne!(variant.file.identities, baseline.file.identities);
        assert_ne!(variant.rust.identities, baseline.rust.identities);
    }
}

struct RequirementPair {
    model: RequirementModel,
    file: CompiledRequirements,
    rust: CompiledRequirements,
}

fn scalar_pair(
    view: &str,
    field: &str,
    family: NativeFamily,
    lifecycle: Lifecycle,
) -> RequirementPair {
    let source = format!(
        "query_scalar {DECLARATION} {{ view {view} field {field} require {} lifecycle {} }}",
        family_keyword(family),
        lifecycle_keyword(lifecycle),
    );
    let rust = WorthUiRustAuthoredArtifactInputModule::new("main.wui")
        .try_with_query_scalar_native(
            DECLARATION,
            view,
            field,
            production_family(family),
            production_lifecycle(lifecycle),
        )
        .expect("QP10 scalar Rust declaration");
    RequirementPair {
        model: RequirementModel::scalar(DECLARATION, view, field, family, lifecycle),
        file: capture(&compile_file(&source, &[])),
        rust: capture(&compile_rust([rust])),
    }
}

fn collection_pair() -> RequirementPair {
    let source = format!(
        "query_collection {DECLARATION} {{ view {VIEW} row identity field {FIELD} \
         require text completeness complete continuation forbidden lifecycle live }}"
    );
    let rust = WorthUiRustAuthoredArtifactInputModule::new("main.wui")
        .try_with_query_collection_native(
            DECLARATION,
            VIEW,
            "identity",
            WorthUiProjectionNativeFamily::Text,
            WorthUiProjectionCollectionSelection::new(
                [FIELD],
                WorthUiProjectionLifecycle::Live,
                WorthUiProjectionCollectionPolicy::new(true, false),
            ),
        )
        .expect("QP10 collection Rust declaration");
    RequirementPair {
        model: RequirementModel::collection(
            DECLARATION,
            VIEW,
            "identity",
            &[FIELD],
            NativeFamily::Text,
            Lifecycle::Live,
            (true, false),
        ),
        file: capture(&compile_file(&source, &[])),
        rust: capture(&compile_rust([rust])),
    }
}

fn assert_pair_matches_model(pair: &RequirementPair) {
    assert_eq!(pair.file.models, std::slice::from_ref(&pair.model));
    assert_eq!(pair.rust.models, std::slice::from_ref(&pair.model));
    assert_eq!(pair.file.identities, pair.rust.identities);
}

fn family_keyword(family: NativeFamily) -> &'static str {
    match family {
        NativeFamily::Text => "text",
        NativeFamily::Boolean => "boolean",
    }
}

fn lifecycle_keyword(lifecycle: Lifecycle) -> &'static str {
    match lifecycle {
        Lifecycle::Snapshot => "snapshot",
        Lifecycle::Live => "live",
    }
}

fn production_family(family: NativeFamily) -> WorthUiProjectionNativeFamily {
    match family {
        NativeFamily::Text => WorthUiProjectionNativeFamily::Text,
        NativeFamily::Boolean => WorthUiProjectionNativeFamily::Boolean,
    }
}

fn production_lifecycle(lifecycle: Lifecycle) -> WorthUiProjectionLifecycle {
    match lifecycle {
        Lifecycle::Snapshot => WorthUiProjectionLifecycle::Snapshot,
        Lifecycle::Live => WorthUiProjectionLifecycle::Live,
    }
}

fn rust_modules(reversed: bool) -> Vec<WorthUiRustAuthoredArtifactInputModule> {
    let mut main = WorthUiRustAuthoredArtifactInputModule::new("main.wui");
    if reversed {
        main = main
            .try_with_query_collection_text(
                "a.rows",
                "a.rows",
                "identity",
                WorthUiProjectionCollectionSelection::new(
                    ["label", "value"],
                    WorthUiProjectionLifecycle::Snapshot,
                    WorthUiProjectionCollectionPolicy::new(true, false),
                ),
            )
            .unwrap()
            .with_import("shared-b.wui")
            .try_with_query_scalar_text(
                "z.status",
                "z.status",
                "status",
                WorthUiProjectionLifecycle::Live,
            )
            .unwrap()
            .with_import("shared-a.wui");
    } else {
        main = main
            .with_import("shared-a.wui")
            .try_with_query_scalar_text(
                "z.status",
                "z.status",
                "status",
                WorthUiProjectionLifecycle::Live,
            )
            .unwrap()
            .with_import("shared-b.wui")
            .try_with_query_collection_text(
                "a.rows",
                "a.rows",
                "identity",
                WorthUiProjectionCollectionSelection::new(
                    ["value", "label"],
                    WorthUiProjectionLifecycle::Snapshot,
                    WorthUiProjectionCollectionPolicy::new(true, false),
                ),
            )
            .unwrap();
    }
    vec![
        main,
        WorthUiRustAuthoredArtifactInputModule::new("shared-a.wui").with_token("shared_a", "A"),
        WorthUiRustAuthoredArtifactInputModule::new("shared-b.wui").with_token("shared_b", "B"),
    ]
}

const SUPPORT_MODULES: &[(&str, &str)] = &[
    ("shared-a.wui", "token shared_a = \"A\";"),
    ("shared-b.wui", "token shared_b = \"B\";"),
];

const FILE_ORDER_LEFT: &str = r#"
    import "shared-a.wui";
    query_scalar z.status { view z.status field status require text lifecycle live }
    import "shared-b.wui";
    query_collection a.rows {
        view a.rows row identity field value field label require text
        completeness complete continuation forbidden lifecycle snapshot
    }
"#;

const FILE_ORDER_RIGHT: &str = r#"
    query_collection a.rows {
        continuation forbidden; lifecycle snapshot; field label;
        completeness complete; require text; row identity;
        field value; view a.rows;
    }
    import "shared-b.wui";
    import "shared-a.wui";
    query_scalar z.status {
        lifecycle live; require text; field status; view z.status;
    }
"#;
