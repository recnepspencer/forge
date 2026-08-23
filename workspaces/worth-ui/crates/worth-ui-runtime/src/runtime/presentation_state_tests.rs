use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use super::{UiApplicationPresentationState, UiApplicationSemanticTextRow};

#[test]
fn complete_projection_retains_current_paint_after_incremental_projection_commits() {
    let token =
        crate::capability::ThemeTokenId::new("theme.test.text").expect("test theme-token identity");
    let color = crate::capability::ThemeTokenValue::color(
        crate::capability::ThemeColorValue::hex("#ffffff").expect("test theme-token value"),
    );
    let node = crate::graph::UiGraphNodeIdentity::new(91_001);
    let row = UiApplicationSemanticTextRow {
        graph_node: Some(node),
        value: Some(Arc::from("current text")),
        contract: crate::capability::ComponentSemanticTextContract::body_default(token.clone(), 1),
        semantic_revision: 1,
        presentation_revision: 2,
        projected_presentation_revision: None,
    };
    let mut state = UiApplicationPresentationState {
        rows: HashMap::from([(Box::<str>::from("component:test"), row)]),
        token_values: BTreeMap::from([(token.clone(), color.clone())]),
        resolved_targets: BTreeMap::from([(token.clone(), token.clone())]),
        mutable_token_revisions: BTreeMap::from([(token.clone(), 1)]),
    };

    let incremental = state.project().expect("current row projects incrementally");
    state.commit(&incremental);
    assert!(state
        .project()
        .expect("committed projection")
        .content()
        .is_empty());

    let complete = state
        .project_complete()
        .expect("complete current projection");
    let complete_content = complete.content();
    let crate::mounting::UiMountedSemanticTextContent::Scalar(content) =
        complete_content.get(node).expect("current row is retained")
    else {
        panic!("current row remains scalar semantic text");
    };
    assert_eq!(
        content
            .formatting()
            .and_then(|formatting| formatting.token_value(&token)),
        Some(&color)
    );
}
