use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::policy::WorthQueryDerivedView;
use worth_query::facade::runtime::WorthQueryAspectTouch;

fn main() {
    let touch = WorthQueryAspectTouch::aspect_field_path(
        AspectKey::new("title").unwrap(),
        CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
    );
    let _ = WorthQueryDerivedView::new("computed.titles", [touch.clone()])
        .depends_on_live_name("tasks.table");
    let _ = WorthQueryDerivedView::new("computed.summary", [touch])
        .depends_on_derived_name("computed.titles");
}
