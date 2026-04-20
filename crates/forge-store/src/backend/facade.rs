mod authority;
mod bulk;
mod counters;
mod delta;
mod durable_runtime;
mod layout;
mod live_query;
mod maintenance;
mod opening;
mod publication;
mod snapshots;
mod support;

use std::path::PathBuf;

use super::{
    embedded::EmbeddedStoreBackend,
    sqlite::SqliteStoreBackend,
};

#[derive(Debug, Clone)]
pub enum StoreBackendMode {
    InMemory,
    LocalFile(PathBuf),
    SqliteFile(PathBuf),
}

#[derive(Debug)]
pub enum StoreBackend {
    Embedded(EmbeddedStoreBackend),
    Sqlite(SqliteStoreBackend),
}

macro_rules! dispatch_ref {
    ($self:expr, |$backend:ident| $body:expr) => {
        match $self {
            Self::Embedded($backend) => $body,
            Self::Sqlite($backend) => $body,
        }
    };
}

macro_rules! dispatch_mut {
    ($self:expr, |$backend:ident| $body:expr) => {
        match $self {
            Self::Embedded($backend) => $body,
            Self::Sqlite($backend) => $body,
        }
    };
}

pub(crate) use dispatch_mut;
pub(crate) use dispatch_ref;
