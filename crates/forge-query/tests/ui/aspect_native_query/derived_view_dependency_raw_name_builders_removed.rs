use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{ForgeQueryAspectTouch, ForgeQueryDerivedView};

fn main() {
    let touch = ForgeQueryAspectTouch::field_path(
        AspectKey::new("title").unwrap(),
        CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
    );
    let _ = ForgeQueryDerivedView::new("computed.titles", [touch.clone()])
        .depends_on_live_name("tasks.table");
    let _ = ForgeQueryDerivedView::new("computed.summary", [touch])
        .depends_on_derived_name("computed.titles");
}
