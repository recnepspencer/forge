use crate::identity::hash_parts;
use crate::query_context::AdmittedQueryBasisContext;
use crate::runtime::{
    ForgeQueryLiveView, ForgeQueryReadFamily, ForgeQueryRuntimeLiveSubscriptionInstallation,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryReadExecutionIntentSeed {
    read_family: ForgeQueryReadFamily,
    basis_context: Option<AdmittedQueryBasisContext>,
    request_label: String,
    request_input_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryLiveReadIntentSeed {
    installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    live_view_name: String,
    live_view_digest: String,
    request_label: String,
    request_input_digest: String,
}

impl ForgeQueryReadExecutionIntentSeed {
    pub fn current_runtime(read_family: ForgeQueryReadFamily) -> Self {
        let request_label = format!("read.family.{}", read_family.family_name());
        let request_input_digest = hash_parts(&[
            "forge_query_read_execution_intent_seed_v1".to_string(),
            format!("family:{}", read_family.family_digest()),
            "basis:runtime-current".to_string(),
        ]);
        Self {
            read_family,
            basis_context: None,
            request_label,
            request_input_digest,
        }
    }

    pub fn in_basis_context(
        read_family: ForgeQueryReadFamily,
        basis_context: AdmittedQueryBasisContext,
    ) -> Self {
        let request_label = format!(
            "read.family.{}.basis.{}",
            read_family.family_name(),
            basis_context.family().as_str()
        );
        let request_input_digest = hash_parts(&[
            "forge_query_read_execution_intent_seed_v1".to_string(),
            format!("family:{}", read_family.family_digest()),
            format!("basis:{}", basis_context.basis_digest()),
            format!("query:{}", basis_context.query_digest()),
            format!("context-family:{}", basis_context.family().as_str()),
        ]);
        Self {
            read_family,
            basis_context: Some(basis_context),
            request_label,
            request_input_digest,
        }
    }

    pub fn read_family(&self) -> &ForgeQueryReadFamily {
        &self.read_family
    }

    pub fn basis_context(&self) -> Option<&AdmittedQueryBasisContext> {
        self.basis_context.as_ref()
    }

    pub fn request_label(&self) -> &str {
        &self.request_label
    }

    pub fn request_input_digest(&self) -> &str {
        &self.request_input_digest
    }
}

impl ForgeQueryLiveReadIntentSeed {
    pub fn from_live_view<T>(live_view: &ForgeQueryLiveView<T>) -> Self {
        Self::from_installation(live_view.subscription_installation())
    }

    pub fn from_installation(installation: &ForgeQueryRuntimeLiveSubscriptionInstallation) -> Self {
        let live_view_name = installation.view_name().to_string();
        let live_view_digest = installation.installation_projection().label().to_string();
        let request_label = format!("read.live-view.{live_view_name}");
        let request_input_digest = hash_parts(&[
            "forge_query_live_read_intent_seed_v1".to_string(),
            format!("view:{live_view_name}"),
            format!("installation:{live_view_digest}"),
        ]);
        Self {
            installation: installation.clone(),
            live_view_name,
            live_view_digest,
            request_label,
            request_input_digest,
        }
    }

    pub fn installation(&self) -> &ForgeQueryRuntimeLiveSubscriptionInstallation {
        &self.installation
    }

    pub fn live_view_name(&self) -> &str {
        &self.live_view_name
    }

    pub fn live_view_digest(&self) -> &str {
        &self.live_view_digest
    }

    pub fn request_label(&self) -> &str {
        &self.request_label
    }

    pub fn request_input_digest(&self) -> &str {
        &self.request_input_digest
    }
}
