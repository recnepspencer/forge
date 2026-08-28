mod bundle_publication;
mod deferred_settlement;
mod diagnostic_emission;
mod post_commit_consumer;
mod post_commit_consumption;
mod post_commit_diagnostics;

use crate::runtime::RelationalRuntime;

pub struct PublicationAuthority<'runtime> {
    pub(super) runtime: &'runtime mut RelationalRuntime,
}

pub(crate) struct PublicationPreparationAuthority<'runtime> {
    pub(super) runtime: &'runtime crate::runtime::RelationalPreparationRuntime,
}

impl RelationalRuntime {
    pub(crate) fn publication_authority(&mut self) -> PublicationAuthority<'_> {
        PublicationAuthority::new(self)
    }
}

impl crate::runtime::RelationalPreparationRuntime {
    pub(crate) fn publication_preparation_authority(&self) -> PublicationPreparationAuthority<'_> {
        PublicationPreparationAuthority::new(self)
    }
}

impl<'runtime> PublicationPreparationAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime crate::runtime::RelationalPreparationRuntime) -> Self {
        Self { runtime }
    }
}

impl<'runtime> PublicationAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }
}

pub(crate) use post_commit_consumer::production_post_commit_consumer;
pub use post_commit_consumer::{
    PostCommitConsumer, PostCommitConsumptionContext, PostCommitConsumptionFailure,
};
