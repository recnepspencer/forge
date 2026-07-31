#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryResultTraversalDirection {
    Forward,
    Reverse,
}

mod traversal_seal {
    pub trait Sealed {}
}

pub trait ApplicationQueryResultTraversal: traversal_seal::Sealed {
    const DIRECTION: ApplicationQueryResultTraversalDirection;
}

pub trait ApplicationQueryResultTraversalEndpoints<Parent, Child, From, To>:
    ApplicationQueryResultTraversal
{
}

pub struct ForwardResultTraversal;
pub struct ReverseResultTraversal;

impl traversal_seal::Sealed for ForwardResultTraversal {}
impl traversal_seal::Sealed for ReverseResultTraversal {}

impl ApplicationQueryResultTraversal for ForwardResultTraversal {
    const DIRECTION: ApplicationQueryResultTraversalDirection =
        ApplicationQueryResultTraversalDirection::Forward;
}

impl ApplicationQueryResultTraversal for ReverseResultTraversal {
    const DIRECTION: ApplicationQueryResultTraversalDirection =
        ApplicationQueryResultTraversalDirection::Reverse;
}

impl<From, To> ApplicationQueryResultTraversalEndpoints<From, To, From, To>
    for ForwardResultTraversal
{
}

impl<From, To> ApplicationQueryResultTraversalEndpoints<To, From, From, To>
    for ReverseResultTraversal
{
}
