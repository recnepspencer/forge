use std::fmt;

use serde::{Deserialize, Serialize};

use super::ErrorContext;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationalError {
    Transaction(crate::transactions::data::TransactionCommitError),
    Durability(crate::durability::data::DurabilityError),
    History(crate::history::data::BranchCreateError),
    Schema(crate::schema::data::SchemaRegistryError),
    Publication(crate::publication::data::PublicationError),
    Replay(crate::replay::data::ReplayError),
}

impl RelationalError {
    pub fn context(&self) -> &ErrorContext {
        match self {
            Self::Transaction(error) => error.context(),
            Self::Durability(error) => &error.context,
            Self::History(error) => &error.context,
            Self::Schema(error) => &error.context,
            Self::Publication(error) => &error.context,
            Self::Replay(error) => &error.context,
        }
    }
}

impl From<crate::transactions::data::TransactionCommitError> for RelationalError {
    fn from(value: crate::transactions::data::TransactionCommitError) -> Self {
        Self::Transaction(value)
    }
}

impl From<crate::durability::data::DurabilityError> for RelationalError {
    fn from(value: crate::durability::data::DurabilityError) -> Self {
        Self::Durability(value)
    }
}

impl From<crate::history::data::BranchCreateError> for RelationalError {
    fn from(value: crate::history::data::BranchCreateError) -> Self {
        Self::History(value)
    }
}

impl From<crate::schema::data::SchemaRegistryError> for RelationalError {
    fn from(value: crate::schema::data::SchemaRegistryError) -> Self {
        Self::Schema(value)
    }
}

impl From<crate::publication::data::PublicationError> for RelationalError {
    fn from(value: crate::publication::data::PublicationError) -> Self {
        Self::Publication(value)
    }
}

impl From<crate::replay::data::ReplayError> for RelationalError {
    fn from(value: crate::replay::data::ReplayError) -> Self {
        Self::Replay(value)
    }
}

impl fmt::Display for RelationalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transaction(error) => write!(f, "transaction: {}", error.detail()),
            Self::Durability(error) => write!(f, "durability: {}", error.detail),
            Self::History(error) => write!(f, "history: {}", error.detail),
            Self::Schema(error) => write!(f, "schema: {}", error.detail),
            Self::Publication(error) => write!(f, "publication: {}", error.detail),
            Self::Replay(error) => write!(f, "replay: {}", error.detail),
        }
    }
}

impl std::error::Error for RelationalError {}
