use super::WorthQueryOperationTouchScope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationTouchContract {
    NotRequired,
    Declared {
        graph_roles: Vec<String>,
        scopes: Vec<WorthQueryOperationTouchScope>,
    },
}

impl WorthQueryOperationTouchContract {
    pub fn graph_roles(&self) -> &[String] {
        match self {
            Self::NotRequired => &[],
            Self::Declared { graph_roles, .. } => graph_roles,
        }
    }

    pub fn scopes(&self) -> &[WorthQueryOperationTouchScope] {
        match self {
            Self::NotRequired => &[],
            Self::Declared { scopes, .. } => scopes,
        }
    }
}
