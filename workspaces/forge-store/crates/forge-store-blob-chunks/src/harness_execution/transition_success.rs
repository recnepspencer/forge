pub(super) trait TransitionSuccess<S> {
    fn success(self, message: &str) -> S;
}

impl<S, D, De, St, R, F> TransitionSuccess<S> for forge_proof::TransitionOutcome<S, D, De, St, R, F>
where
    S: core::fmt::Debug,
    D: core::fmt::Debug,
    De: core::fmt::Debug,
    St: core::fmt::Debug,
    R: core::fmt::Debug,
    F: core::fmt::Debug,
{
    fn success(self, message: &str) -> S {
        match self {
            forge_proof::TransitionOutcome::Success(value) => value,
            outcome => panic!("{message}: {outcome:?}"),
        }
    }
}
