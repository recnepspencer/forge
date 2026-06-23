use super::*;
use forge_foundational::facade::AspectValue;

pub(in crate::runtime::tests) fn insert_command<I, P, V>(
    collection: impl Into<String>,
    aspects: I,
) -> ForgeQueryWriteCommand
where
    I: IntoIterator<Item = (P, V)>,
    P: Into<String>,
    V: Into<AspectValue>,
{
    aspects
        .into_iter()
        .fold(
            ForgeQueryAspectMutationBuilder::new(),
            |builder, (path, value)| {
                let touch_fixture = path.into();
                builder.set_aspect(
                    test_aspect_touch(&touch_fixture),
                    ForgeQueryAuthoredAspectValue::from_foundational_value(value.into()),
                )
            },
        )
        .build_insert(collection)
        .expect("test insert command should build")
}
