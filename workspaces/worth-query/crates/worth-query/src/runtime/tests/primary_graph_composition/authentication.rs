use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant, SystemTime};

use worth_query_admission::facade::authenticated_principal::{
    admit_authentication_adapter, WorthQueryAuthenticatedExternalPrincipal,
    WorthQueryAuthenticationAdapter, WorthQueryAuthenticationAdapterAdmission,
    WorthQueryAuthenticationAdapterFailure, WorthQueryAuthenticationAdapterFailureKind,
    WorthQueryAuthenticationAudience, WorthQueryAuthenticationFuture,
    WorthQueryAuthenticationMethod, WorthQueryCancellationSource, WorthQueryRequestScope,
    WorthQueryValidatedExternalPrincipal,
};
use worth_query_declaration::facade::authentication::WorthQueryExternalPrincipalIdentity;
use worth_query_installation::facade::WorthQueryInstalledApplicationSchema;

use super::schema::PrimaryGraphCompositionSchema;

pub(super) fn external_identity(subject: &str) -> WorthQueryExternalPrincipalIdentity {
    WorthQueryExternalPrincipalIdentity::new("https://issuer.test.invalid", subject)
        .expect("test external identity should admit")
}

pub(super) fn live_scope() -> WorthQueryRequestScope {
    let cancellation = WorthQueryCancellationSource::new();
    WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        cancellation.token(),
    )
}

pub(super) fn authenticate(
    schema: &WorthQueryInstalledApplicationSchema<PrimaryGraphCompositionSchema>,
    subject: &str,
    lifetime: Duration,
    scope: &WorthQueryRequestScope,
) -> WorthQueryAuthenticatedExternalPrincipal<PrimaryGraphCompositionSchema> {
    let adapter = admit_authentication_adapter(
        schema,
        WorthQueryAuthenticationAdapterAdmission::new(
            WorthQueryAuthenticationAudience::new("primary-graph-test")
                .expect("test audience should admit"),
            WorthQueryAuthenticationMethod::new("causal-test-adapter")
                .expect("test method should admit"),
        ),
        CausalIdentityAdapter,
    )
    .expect("causal test adapter should admit");
    block_on(adapter.authenticate(
        TestCredential {
            subject: subject.to_string(),
            lifetime,
        },
        scope,
    ))
    .expect("test credential should validate")
}

struct TestCredential {
    subject: String,
    lifetime: Duration,
}

struct CausalIdentityAdapter;

impl WorthQueryAuthenticationAdapter for CausalIdentityAdapter {
    type Credential = TestCredential;

    fn configuration_identity(&self) -> &str {
        "primary-graph-composition-adapter-v1"
    }

    fn validate<'a>(
        &'a self,
        credential: Self::Credential,
        _scope: &'a WorthQueryRequestScope,
    ) -> WorthQueryAuthenticationFuture<'a> {
        Box::pin(async move {
            let now = SystemTime::now();
            WorthQueryValidatedExternalPrincipal::new(
                external_identity(&credential.subject),
                WorthQueryAuthenticationAudience::new("primary-graph-test")
                    .expect("test audience should admit"),
                WorthQueryAuthenticationMethod::new("causal-test-adapter")
                    .expect("test method should admit"),
                now,
                now + credential.lifetime,
                Vec::new(),
            )
            .map_err(|_| {
                WorthQueryAuthenticationAdapterFailure::new(
                    WorthQueryAuthenticationAdapterFailureKind::ProtocolViolation,
                )
            })
        })
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}
