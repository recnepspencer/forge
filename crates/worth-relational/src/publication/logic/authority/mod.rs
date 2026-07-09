mod bundle_publication;
mod diagnostic_emission;
mod post_commit_consumption;
mod post_commit_diagnostics;
#[cfg(test)]
mod test_faults;

use crate::logic::runtime::RelationalRuntime;

pub struct PublicationAuthority<'runtime> {
    pub(super) runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub(crate) fn publication_authority(&mut self) -> PublicationAuthority<'_> {
        PublicationAuthority::new(self)
    }
}

impl<'runtime> PublicationAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }
}

#[cfg(test)]
pub(crate) use test_faults::{with_test_post_commit_fault, TestPostCommitFault};
