use super::{declare, WorthQueryMutationDeclaration};
use crate::runtime::{WorthQueryAspectTouch, WorthQueryAuthoredAspectValue};

fn mutation(value: &str, metadata: Option<&str>) -> WorthQueryMutationDeclaration {
    declare(|builder| {
        let builder = if let Some(metadata) = metadata {
            builder.metadata("intent", metadata)
        } else {
            builder
        };
        builder
            .set_aspect(
                WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")?,
                WorthQueryAuthoredAspectValue::string(value),
            )
            .build_insert("Task")
    })
    .expect("mutation should declare")
}

#[test]
fn declaration_identity_covers_complete_mutation_semantics() {
    let original = mutation("task-a", Some("sync"));
    let equivalent = mutation("task-a", Some("sync"));
    let changed_value = mutation("task-b", Some("sync"));
    let changed_metadata = mutation("task-a", Some("reconcile"));

    assert_eq!(original.identity(), equivalent.identity());
    assert_ne!(original.identity(), changed_value.identity());
    assert_ne!(original.identity(), changed_metadata.identity());
}
