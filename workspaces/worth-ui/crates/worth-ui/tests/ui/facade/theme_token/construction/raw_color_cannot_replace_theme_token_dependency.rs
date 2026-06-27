use worth_ui::facade::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership,
};

fn main() {
    let _descriptor = ComponentDescriptor::new(
        ComponentId::new("component.label").unwrap(),
        ComponentPropSchema::named("component.label.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
    .with_theme_token_dependency("#ffffff");
}
