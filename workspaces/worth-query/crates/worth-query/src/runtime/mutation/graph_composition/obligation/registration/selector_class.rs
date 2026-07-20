#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime::mutation::graph_composition::obligation) enum WorthQueryGraphTouchSelectorClass
{
    Any,
    Collection,
    RelationKindId,
    AspectTouch,
    DeclaredAspectOperation,
    DeclaredMutationCollection,
    MutationFamily,
    LifecycleFamily,
    ReadVerb,
}
