use crate::obligations::verdict::UiObligationDispatchStopPosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiObligationDispatchExecution {
    ImmediateCheck,
    TypedStop(UiObligationDispatchStopPosture),
}
