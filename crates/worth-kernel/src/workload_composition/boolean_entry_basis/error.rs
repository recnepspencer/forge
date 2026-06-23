#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEntryBasisError {
    MissingQueryDeclaration,
    QueryAdmissionFailed(String),
}

impl PlanarBooleanEntryBasisError {
    pub fn human_reason(&self) -> String {
        match self {
            Self::MissingQueryDeclaration => {
                "planar boolean entry basis requires a human-readable Query intent".to_string()
            }
            Self::QueryAdmissionFailed(reason) => {
                format!("planar boolean entry basis could not be admitted by Forge Query: {reason}")
            }
        }
    }
}
