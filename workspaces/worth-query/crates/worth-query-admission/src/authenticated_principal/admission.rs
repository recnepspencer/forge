use std::marker::PhantomData;
use std::time::{Instant, SystemTime};

use sha2::{Digest, Sha256};
use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaBindingIdentity,
};
use worth_query_installation::facade::WorthQueryInstalledApplicationSchema;

use super::{
    WorthQueryAuthenticatedExternalPrincipal, WorthQueryAuthenticationAdapter,
    WorthQueryAuthenticationAudience, WorthQueryAuthenticationDenial,
    WorthQueryAuthenticationDenialKind, WorthQueryAuthenticationMethod, WorthQueryRequestScope,
};

const MAX_CONFIGURATION_IDENTITY_BYTES: usize = 512;

pub struct WorthQueryAuthenticationAdapterAdmission {
    audience: WorthQueryAuthenticationAudience,
    method: WorthQueryAuthenticationMethod,
}

impl WorthQueryAuthenticationAdapterAdmission {
    pub fn new(
        audience: WorthQueryAuthenticationAudience,
        method: WorthQueryAuthenticationMethod,
    ) -> Self {
        Self { audience, method }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAuthenticationAdapterAdmissionDenial {
    InvalidConfigurationIdentity,
}

pub fn admit_authentication_adapter<Schema, Adapter>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    admission: WorthQueryAuthenticationAdapterAdmission,
    adapter: Adapter,
) -> Result<
    WorthQueryAdmittedAuthenticationAdapter<Schema, Adapter>,
    WorthQueryAuthenticationAdapterAdmissionDenial,
>
where
    Schema: ApplicationSchema,
    Adapter: WorthQueryAuthenticationAdapter,
{
    let configuration_identity = adapter.configuration_identity();
    if configuration_identity.is_empty()
        || configuration_identity.trim() != configuration_identity
        || configuration_identity.len() > MAX_CONFIGURATION_IDENTITY_BYTES
        || configuration_identity.chars().any(char::is_control)
    {
        return Err(WorthQueryAuthenticationAdapterAdmissionDenial::InvalidConfigurationIdentity);
    }
    let binding_identity = schema.binding_identity();
    let adapter_identity = adapter_identity::<Adapter>(
        &binding_identity,
        configuration_identity,
        &admission.audience,
        &admission.method,
    );
    Ok(WorthQueryAdmittedAuthenticationAdapter {
        adapter,
        audience: admission.audience,
        method: admission.method,
        adapter_identity,
        binding_identity,
        _schema: PhantomData,
    })
}

pub struct WorthQueryAdmittedAuthenticationAdapter<Schema, Adapter> {
    adapter: Adapter,
    audience: WorthQueryAuthenticationAudience,
    method: WorthQueryAuthenticationMethod,
    adapter_identity: String,
    binding_identity: ApplicationSchemaBindingIdentity,
    _schema: PhantomData<fn() -> Schema>,
}

impl<Schema, Adapter> WorthQueryAdmittedAuthenticationAdapter<Schema, Adapter>
where
    Adapter: WorthQueryAuthenticationAdapter,
{
    /// Returns the admitted protocol adapter without exposing proof-minting authority.
    pub fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    pub async fn authenticate(
        &self,
        credential: Adapter::Credential,
        scope: &WorthQueryRequestScope,
    ) -> Result<WorthQueryAuthenticatedExternalPrincipal<Schema>, WorthQueryAuthenticationDenial>
    {
        if let Some(interruption) = scope.interruption() {
            return Err(WorthQueryAuthenticationDenial::interrupted(interruption));
        }
        let candidate = self
            .adapter
            .validate(credential, scope)
            .await
            .map_err(|failure| {
                WorthQueryAuthenticationDenial::new(
                    WorthQueryAuthenticationDenialKind::AdapterFailed(failure.kind()),
                )
            })?;
        if let Some(interruption) = scope.interruption() {
            return Err(WorthQueryAuthenticationDenial::interrupted(interruption));
        }
        self.admit_candidate(candidate)
    }

    fn admit_candidate(
        &self,
        candidate: super::WorthQueryValidatedExternalPrincipal,
    ) -> Result<WorthQueryAuthenticatedExternalPrincipal<Schema>, WorthQueryAuthenticationDenial>
    {
        if candidate.audience() != &self.audience {
            return Err(WorthQueryAuthenticationDenial::new(
                WorthQueryAuthenticationDenialKind::AudienceMismatch,
            ));
        }
        if candidate.method() != &self.method {
            return Err(WorthQueryAuthenticationDenial::new(
                WorthQueryAuthenticationDenialKind::MethodMismatch,
            ));
        }
        let wall_now = SystemTime::now();
        let monotonic_now = Instant::now();
        if candidate.validated_at() > wall_now {
            return Err(WorthQueryAuthenticationDenial::new(
                WorthQueryAuthenticationDenialKind::ValidationTimeInFuture,
            ));
        }
        let remaining = candidate
            .expires_at()
            .duration_since(wall_now)
            .map_err(|_| {
                WorthQueryAuthenticationDenial::new(WorthQueryAuthenticationDenialKind::Expired)
            })?;
        let valid_until = monotonic_now.checked_add(remaining).ok_or_else(|| {
            WorthQueryAuthenticationDenial::new(WorthQueryAuthenticationDenialKind::Expired)
        })?;
        let (identity, audience, method, validated_at, expires_at, attributes) =
            candidate.into_parts();
        Ok(WorthQueryAuthenticatedExternalPrincipal::mint(
            identity,
            audience,
            method,
            validated_at,
            expires_at,
            valid_until,
            self.adapter_identity.clone(),
            self.binding_identity.clone(),
            attributes,
        ))
    }

    pub fn adapter_identity(&self) -> &str {
        &self.adapter_identity
    }
}

impl<Schema, Adapter> std::fmt::Debug for WorthQueryAdmittedAuthenticationAdapter<Schema, Adapter> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryAdmittedAuthenticationAdapter")
            .field("audience", &self.audience)
            .field("method", &self.method)
            .field("adapter_identity", &self.adapter_identity)
            .field("binding_identity", &self.binding_identity)
            .finish_non_exhaustive()
    }
}

fn adapter_identity<Adapter>(
    binding: &ApplicationSchemaBindingIdentity,
    configuration: &str,
    audience: &WorthQueryAuthenticationAudience,
    method: &WorthQueryAuthenticationMethod,
) -> String {
    let mut hash = Sha256::new();
    hash.update(b"worth-query-authentication-adapter-v1");
    for value in [
        std::any::type_name::<Adapter>(),
        configuration,
        binding.package_identity(),
        binding.schema_identity().as_str(),
        audience.as_str(),
        method.as_str(),
    ] {
        hash.update(value.len().to_le_bytes());
        hash.update(value.as_bytes());
    }
    hash.update(binding.runtime_ordinal().to_le_bytes());
    hash.update(binding.generation().to_le_bytes());
    format!("{:x}", hash.finalize())
}
