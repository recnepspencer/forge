use super::{
    ForgeServerBindAddress, ForgeServerMiddlewareConfig, ForgeServerMiddlewareConfigBuilder,
    ForgeServerOperatorEvidenceConfig, ForgeServerOperatorEvidenceConfigBuilder,
    ForgeServerQueryHandoffConfig, ForgeServerQueryHandoffConfigBuilder,
    ForgeServerRequestContextConfig, ForgeServerRequestContextConfigBuilder,
    ForgeServerResponseConfig, ForgeServerResponseConfigBuilder,
};

#[derive(Clone, Debug)]
pub struct ForgeServerConfig {
    bind_address: ForgeServerBindAddress,
    middleware: ForgeServerMiddlewareConfig,
    operator_evidence: ForgeServerOperatorEvidenceConfig,
    query_handoff: ForgeServerQueryHandoffConfig,
    response: ForgeServerResponseConfig,
    request_context: ForgeServerRequestContextConfig,
}

impl ForgeServerConfig {
    pub fn builder() -> ForgeServerConfigBuilder {
        ForgeServerConfigBuilder::default()
    }

    pub fn bind_address(&self) -> ForgeServerBindAddress {
        self.bind_address
    }

    pub fn middleware(&self) -> &ForgeServerMiddlewareConfig {
        &self.middleware
    }

    pub fn operator_evidence(&self) -> &ForgeServerOperatorEvidenceConfig {
        &self.operator_evidence
    }

    pub fn query_handoff(&self) -> &ForgeServerQueryHandoffConfig {
        &self.query_handoff
    }

    pub fn response(&self) -> &ForgeServerResponseConfig {
        &self.response
    }

    pub fn request_context(&self) -> &ForgeServerRequestContextConfig {
        &self.request_context
    }
}

#[derive(Clone, Debug)]
pub struct ForgeServerConfigBuilder {
    bind_address: Option<ForgeServerBindAddress>,
    middleware: ForgeServerMiddlewareConfigBuilder,
    operator_evidence: ForgeServerOperatorEvidenceConfigBuilder,
    query_handoff: ForgeServerQueryHandoffConfigBuilder,
    response: ForgeServerResponseConfigBuilder,
    request_context: ForgeServerRequestContextConfigBuilder,
}

impl Default for ForgeServerConfigBuilder {
    fn default() -> Self {
        Self {
            bind_address: None,
            middleware: ForgeServerMiddlewareConfig::builder(),
            operator_evidence: ForgeServerOperatorEvidenceConfig::builder(),
            query_handoff: ForgeServerQueryHandoffConfig::builder(),
            response: ForgeServerResponseConfig::builder(),
            request_context: ForgeServerRequestContextConfig::builder(),
        }
    }
}

impl ForgeServerConfigBuilder {
    pub fn with_bind_address(mut self, bind_address: ForgeServerBindAddress) -> Self {
        self.bind_address = Some(bind_address);
        self
    }

    pub fn with_middleware_config(mut self, middleware: ForgeServerMiddlewareConfig) -> Self {
        self.middleware = ForgeServerMiddlewareConfigBuilder::default()
            .with_compat_http_maximum_diagnostics_profile(
                middleware.compat_http_maximum_diagnostics_profile(),
            )
            .with_preview_branch_authorization_enabled(
                middleware.preview_branch_authorization_enabled(),
            )
            .with_query_mutation_enabled(middleware.query_mutation_enabled());
        self
    }

    pub fn with_middleware_config_builder(
        mut self,
        middleware: ForgeServerMiddlewareConfigBuilder,
    ) -> Self {
        self.middleware = middleware;
        self
    }

    pub fn with_request_context_config(
        mut self,
        request_context: ForgeServerRequestContextConfig,
    ) -> Self {
        self.request_context = ForgeServerRequestContextConfigBuilder::default()
            .with_default_diagnostics_profile(request_context.default_diagnostics_profile())
            .with_maximum_diagnostics_profile(request_context.maximum_diagnostics_profile())
            .with_branch_targeting_enabled(request_context.branch_targeting_enabled())
            .with_preview_targeting_enabled(request_context.preview_targeting_enabled());
        self
    }

    pub fn with_query_handoff_config(
        mut self,
        query_handoff: ForgeServerQueryHandoffConfig,
    ) -> Self {
        self.query_handoff = ForgeServerQueryHandoffConfigBuilder::default()
            .with_workspace_provider_arc(query_handoff.workspace_provider().clone());
        self
    }

    pub fn with_query_handoff_config_builder(
        mut self,
        query_handoff: ForgeServerQueryHandoffConfigBuilder,
    ) -> Self {
        self.query_handoff = query_handoff;
        self
    }

    pub fn with_operator_evidence_config(
        mut self,
        operator_evidence: ForgeServerOperatorEvidenceConfig,
    ) -> Self {
        self.operator_evidence = ForgeServerOperatorEvidenceConfigBuilder::default()
            .with_default_transform(operator_evidence.default_transform())
            .with_minimum_diagnostics_profile(operator_evidence.minimum_diagnostics_profile());
        self
    }

    pub fn with_operator_evidence_config_builder(
        mut self,
        operator_evidence: ForgeServerOperatorEvidenceConfigBuilder,
    ) -> Self {
        self.operator_evidence = operator_evidence;
        self
    }

    pub fn with_response_config(mut self, response: ForgeServerResponseConfig) -> Self {
        self.response = ForgeServerResponseConfigBuilder::default()
            .with_default_success_transform(response.default_success_transform())
            .with_default_denial_transform(response.default_denial_transform())
            .with_success_minimum_diagnostics_profile(
                response.success_minimum_diagnostics_profile(),
            )
            .with_denial_minimum_diagnostics_profile(response.denial_minimum_diagnostics_profile());
        self
    }

    pub fn with_response_config_builder(
        mut self,
        response: ForgeServerResponseConfigBuilder,
    ) -> Self {
        self.response = response;
        self
    }

    pub fn with_request_context_config_builder(
        mut self,
        request_context: ForgeServerRequestContextConfigBuilder,
    ) -> Self {
        self.request_context = request_context;
        self
    }

    pub fn build(self) -> Result<ForgeServerConfig, ForgeServerConfigError> {
        let bind_address = self
            .bind_address
            .ok_or(ForgeServerConfigError::MissingBindAddress)?;
        let middleware = self
            .middleware
            .build()
            .map_err(ForgeServerConfigError::InvalidMiddlewareConfig)?;
        let query_handoff = self
            .query_handoff
            .build()
            .map_err(ForgeServerConfigError::InvalidQueryHandoffConfig)?;
        let operator_evidence = self
            .operator_evidence
            .build()
            .map_err(ForgeServerConfigError::InvalidOperatorEvidenceConfig)?;
        let response = self
            .response
            .build()
            .map_err(ForgeServerConfigError::InvalidResponseConfig)?;
        let request_context = self
            .request_context
            .build()
            .map_err(ForgeServerConfigError::InvalidRequestContextConfig)?;
        Ok(ForgeServerConfig {
            bind_address,
            middleware,
            operator_evidence,
            query_handoff,
            response,
            request_context,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerConfigError {
    MissingBindAddress,
    InvalidMiddlewareConfig(super::ForgeServerMiddlewareConfigError),
    InvalidOperatorEvidenceConfig(super::ForgeServerOperatorEvidenceConfigError),
    InvalidQueryHandoffConfig(super::ForgeServerQueryHandoffConfigError),
    InvalidResponseConfig(super::ForgeServerResponseConfigError),
    InvalidRequestContextConfig(super::ForgeServerRequestContextConfigError),
}
