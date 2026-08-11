//! Provider-protocol result encoding after required publication is complete.

use super::super::super::super::super::WorthQueryPrimaryGraphProvider;
use super::WorthQueryPublishedApplicationCommit;
use crate::domain_computation::WorthQueryProviderSessionFailure;
#[cfg(test)]
use crate::domain_computation::WorthQueryProviderSessionProtocolStage;

pub(super) fn encode(
    _provider: &WorthQueryPrimaryGraphProvider,
    published: WorthQueryPublishedApplicationCommit,
) -> Result<String, WorthQueryProviderSessionFailure> {
    #[cfg(test)]
    if _provider.take_lost_commit_response() {
        return Err(failure(
            "application commit response was lost after authoritative publication",
        ));
    }
    Ok(format!(
        "primary-application-commit:{}:{}:{}:{}:{}",
        published.application().runtime_instance_id(),
        published.application().commit_reference().commit_id.0,
        published.application().changed_record_count(),
        published.application().emitted_effect_count(),
        published
            .application()
            .application_outcome_identity()
            .expect("published application has an outcome identity")
            .get(),
    ))
}

#[cfg(test)]
fn failure(detail: &'static str) -> WorthQueryProviderSessionFailure {
    super::super::super::super::provider_failure(
        WorthQueryProviderSessionProtocolStage::Commit,
        detail,
    )
}
