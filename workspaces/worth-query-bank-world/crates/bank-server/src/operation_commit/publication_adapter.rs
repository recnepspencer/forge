//! One-way crossing from Query execution evidence into Bank publication.

use worth_query_host::facade::primary_graph::WorthQueryApplicationCommitReceipt;
use worth_query_host::facade::publication::domain_computation::{
    publish_application_commit, WorthQueryApplicationCommitPublicationReceipt,
};

use super::BankCommitReceipt;

pub(super) struct BankCommitPublicationProjection {
    publication: WorthQueryApplicationCommitPublicationReceipt,
    execution: WorthQueryApplicationCommitReceipt,
}

impl BankCommitPublicationProjection {
    fn from_execution(execution: WorthQueryApplicationCommitReceipt) -> Self {
        let publication = publish_application_commit(execution.clone()).into_receipt();
        Self {
            publication,
            execution,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthQueryApplicationCommitPublicationReceipt,
        WorthQueryApplicationCommitReceipt,
    ) {
        (self.publication, self.execution)
    }
}

pub(crate) fn commit_receipt(execution: WorthQueryApplicationCommitReceipt) -> BankCommitReceipt {
    BankCommitReceipt::from_publication_projection(BankCommitPublicationProjection::from_execution(
        execution,
    ))
}
