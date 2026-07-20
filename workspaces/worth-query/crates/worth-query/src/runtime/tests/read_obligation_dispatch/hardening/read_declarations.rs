use super::super::*;
use crate::authoring::TraversalSelector;

pub(super) fn profile_read_declaration(
    read: WorthQueryReadBuilder,
) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
    read.local_detail(
        "user",
        manager_schema(),
        |query| {
            query
                .project(
                    AspectFieldSelector::new("identity", "id")
                        .expect("identity projection should build"),
                )
                .project(
                    AspectFieldSelector::new("profile", "display_name")
                        .expect("profile projection should build"),
                )
        },
        |shape| {
            shape
                .field(
                    AuthoredResultShapeField::new("identity", "id", "id")
                        .expect("identity result-shape field should build"),
                )
                .field(
                    AuthoredResultShapeField::new("profile", "display_name", "display_name")
                        .expect("profile result-shape field should build"),
                )
        },
    )
}

pub(super) fn traversal_read_family(workspace: &mut WorthQueryWorkspace) -> WorthQueryReadFamily {
    workspace
        .define_read_family("manager-traversal", |read| {
            read.local_detail(
                "user",
                manager_schema(),
                |query| {
                    query
                        .project(
                            AspectFieldSelector::new("identity", "id")
                                .expect("identity projection should build"),
                        )
                        .traverse(
                            TraversalSelector::bounded("manager", 1)
                                .expect("manager traversal should build"),
                        )
                },
                |shape| {
                    shape.field(
                        AuthoredResultShapeField::new("identity", "id", "id")
                            .expect("identity result-shape field should build"),
                    )
                },
            )
        })
        .expect("traversal read family should define")
}
