#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KernelError {
    UnsupportedNormalEvaluation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolicyKind {
    CoincidentGeometry,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PolicyQuery {
    pub kind: PolicyKind,
    pub location: [f64; 3],
    pub margin: f64,
    pub overridable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PolicyResult<T> {
    Success(T),
    Ambiguous {
        query: PolicyQuery,
        potential_value: T,
    },
}

impl<T> PolicyResult<T> {
    pub fn into_result_strict(self) -> Result<T, PolicyQuery> {
        match self {
            Self::Success(value) => Ok(value),
            Self::Ambiguous { query, .. } => Err(query),
        }
    }
}
