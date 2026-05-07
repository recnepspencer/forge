use super::*;
use serde::Serialize;

pub(in crate::runtime::tests) fn insert_command<I, P, V>(
    collection: impl Into<String>,
    aspects: I,
) -> ForgeQueryWriteCommand
where
    I: IntoIterator<Item = (P, V)>,
    P: Into<String>,
    V: Serialize,
{
    aspects
        .into_iter()
        .fold(
            ForgeQueryAspectMutationBuilder::new(),
            |builder, (path, value)| builder.aspect(path, value),
        )
        .build_insert(collection)
        .expect("test insert command should build")
}
