#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEndpointBoundarySplitAction {
    FragmentCut,
    EndpointContactDecision,
}
