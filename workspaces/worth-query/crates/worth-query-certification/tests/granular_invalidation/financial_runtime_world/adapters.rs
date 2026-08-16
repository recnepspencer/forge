use std::future::Future;
use std::pin::pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant, SystemTime};

use worth_query_host::facade::{admission, domain, primary_graph};

use super::contract::{CurveRiskNode, PortfolioRiskNode, PortfolioSiblingRiskNode, QuoteRiskNode};
use super::schema::{
    ExecuteFinancial, FinancialHostSchema, FinancialInput, FinancialIntentResult,
    MarketObservation, RiskValueField,
};

pub struct FinancialPredicate {
    eligible: Arc<AtomicBool>,
}

#[derive(Clone)]
pub struct FinancialGateController(Arc<AtomicBool>);

#[derive(Clone)]
pub struct FinancialQuoteOutputState(Arc<std::sync::atomic::AtomicU64>);

impl FinancialQuoteOutputState {
    pub fn new(value: u64) -> Self {
        Self(Arc::new(std::sync::atomic::AtomicU64::new(value)))
    }

    pub fn set(&self, value: u64) {
        self.0.store(value, Ordering::Release);
    }
}

pub struct QuoteOutputVersionProvider(pub FinancialQuoteOutputState);

impl domain::WorthQueryHostConditionalOutputVersionProvider<QuoteRiskNode>
    for QuoteOutputVersionProvider
{
    fn semantic_identity(&self) -> &'static str {
        "worth.query.financial.quote-output-version"
    }

    fn output_version(
        &self,
        _fallback_attempt: u64,
    ) -> Result<u64, domain::WorthQueryHostPredicateFailure> {
        Ok(self.0 .0.load(Ordering::Acquire))
    }
}

pub struct QuoteToleranceComparator;

impl domain::WorthQueryHostConditionalOutputComparatorProvider<QuoteRiskNode>
    for QuoteToleranceComparator
{
    fn semantic_identity(&self) -> &'static str {
        "worth.query.financial.quote-tolerance-5"
    }

    fn has_meaningful_change(
        &self,
        cached: u64,
        current: u64,
    ) -> Result<bool, domain::WorthQueryHostPredicateFailure> {
        Ok(cached.abs_diff(current) > 5)
    }
}

impl FinancialPredicate {
    pub fn blocked() -> (Self, FinancialGateController) {
        let eligible = Arc::new(AtomicBool::new(false));
        (
            Self {
                eligible: Arc::clone(&eligible),
            },
            FinancialGateController(eligible),
        )
    }
}

impl FinancialGateController {
    pub fn release(&self) {
        self.0.store(true, Ordering::Release);
    }
}

macro_rules! eligible_predicate {
    ($node:ty, $identity:literal) => {
        impl domain::WorthQueryHostConditionalPredicateProvider<$node> for FinancialPredicate {
            const SEMANTIC_IDENTITY: &'static str = $identity;

            fn evaluate(
                &self,
                observation: domain::WorthQueryConditionalObservationView<'_>,
            ) -> Result<
                domain::WorthQueryHostPredicateDecision,
                domain::WorthQueryHostPredicateFailure,
            > {
                Ok(
                    if self.eligible.load(Ordering::Acquire)
                        && observation.dependency(0).is_some_and(|dependency| {
                            matches!(
                                dependency.current(),
                                domain::WorthQueryConditionalObservedValue::Present(_)
                            )
                        })
                    {
                        domain::WorthQueryHostPredicateDecision::Satisfied
                    } else {
                        domain::WorthQueryHostPredicateDecision::Unsatisfied
                    },
                )
            }
        }
    };
}

eligible_predicate!(CurveRiskNode, "worth.query.financial.curve-predicate");
eligible_predicate!(QuoteRiskNode, "worth.query.financial.quote-predicate");
eligible_predicate!(
    PortfolioRiskNode,
    "worth.query.financial.portfolio-predicate"
);
eligible_predicate!(
    PortfolioSiblingRiskNode,
    "worth.query.financial.portfolio-sibling-predicate"
);

pub struct FinancialIntentProjector;

impl<Node>
    domain::WorthQueryTemporalIntentProjector<
        Node,
        crate::adapters::CourtroomClock,
        FinancialIntentResult,
        FinancialInput,
    > for FinancialIntentProjector
{
    const SEMANTIC_IDENTITY: &'static str = "worth.query.financial.intent-projector";

    fn project(
        &self,
        row: &FinancialIntentResult,
    ) -> Result<
        domain::WorthQueryTemporalIntentCandidate<crate::adapters::CourtroomClock, FinancialInput>,
        domain::WorthQueryTemporalIntentProjectionFailure,
    > {
        let identity = domain::WorthQueryTemporalIntentIdentity::declare(row.identity.clone())
            .map_err(projection_failure)?;
        let input_identity =
            domain::WorthQueryTemporalOperationInputIdentity::declare(row.input.clone())
                .map_err(projection_failure)?;
        let idempotency = domain::WorthQueryTemporalIntentIdempotencyRelation::declare(format!(
            "{}:{}:{}",
            row.identity, row.revision, row.input
        ))
        .map_err(projection_failure)?;
        let input = FinancialInput(row.input.clone());
        let due = domain::WorthQueryClockCoordinate::from_nanoseconds(row.due);
        Ok(match row.lifecycle.as_str() {
            "active" => domain::WorthQueryTemporalIntentCandidate::active(
                identity,
                row.identity.clone(),
                row.revision,
                due,
                input,
                input_identity,
                idempotency,
            ),
            "cancelled" => domain::WorthQueryTemporalIntentCandidate::cancelled(
                identity,
                row.identity.clone(),
                row.revision,
                due,
                input,
                input_identity,
                idempotency,
            ),
            _ => domain::WorthQueryTemporalIntentCandidate::completed(
                identity,
                row.identity.clone(),
                row.revision,
                due,
                input,
                input_identity,
                idempotency,
            ),
        })
    }
}

fn projection_failure(detail: &'static str) -> domain::WorthQueryTemporalIntentProjectionFailure {
    domain::WorthQueryTemporalIntentProjectionFailure::new(
        domain::WorthQueryTemporalIntentProjectionFailureKind::InvalidIdentity,
        detail,
    )
}

pub struct FinancialInvoker;

impl
    primary_graph::WorthQueryTemporalOperationInvoker<
        FinancialHostSchema,
        ExecuteFinancial,
        FinancialInput,
        MarketObservation,
    > for FinancialInvoker
{
    const SEMANTIC_IDENTITY: &'static str = "worth.query.financial.invoker";
    type Projection =
        primary_graph::WorthQueryInvariantMutationTarget<FinancialHostSchema, MarketObservation>;

    fn preconditions(
        &self,
        _input: &FinancialInput,
    ) -> worth_query_host::facade::declaration::application_schema::TypedMutationPreconditions<
        FinancialHostSchema,
        ExecuteFinancial,
        MarketObservation,
    > {
        Default::default()
    }

    fn project(
        &self,
        _input: &FinancialInput,
        reader: &mut primary_graph::WorthQueryApplicationOperationInvariantProjectionReader<
            '_,
            '_,
            FinancialHostSchema,
            ExecuteFinancial,
        >,
        scope: &primary_graph::WorthQueryInvariantEntityIdentity<
            FinancialHostSchema,
            MarketObservation,
        >,
    ) -> Result<Self::Projection, primary_graph::WorthQueryTemporalInvocationFailure> {
        reader
            .decision_field(scope, RiskValueField::reference())
            .map_err(|denial| {
                primary_graph::WorthQueryTemporalInvocationFailure::new(
                    primary_graph::WorthQueryTemporalInvocationFailureKind::ProjectionRejected,
                    denial.to_string(),
                )
            })?;
        reader.mutation_target(scope).map_err(|detail| {
            primary_graph::WorthQueryTemporalInvocationFailure::new(
                primary_graph::WorthQueryTemporalInvocationFailureKind::ProjectionRejected,
                detail,
            )
        })
    }

    fn apply(
        &self,
        input: FinancialInput,
        target: Self::Projection,
        effects: &mut primary_graph::WorthQueryApplicationEffectProgramBuilder<
            FinancialHostSchema,
            ExecuteFinancial,
            FinancialInput,
            MarketObservation,
        >,
    ) -> Result<(), primary_graph::WorthQueryTemporalInvocationFailure> {
        let target = effects
            .projected_entity(&target)
            .map_err(invocation_failure)?;
        let value = input.0.parse::<u64>().unwrap_or_default();
        effects
            .write_field(&target, RiskValueField::reference(), value)
            .map_err(invocation_failure)?;
        Ok(())
    }
}

fn invocation_failure(
    denial: impl std::fmt::Display,
) -> primary_graph::WorthQueryTemporalInvocationFailure {
    primary_graph::WorthQueryTemporalInvocationFailure::new(
        primary_graph::WorthQueryTemporalInvocationFailureKind::InvocationRejected,
        denial.to_string(),
    )
}

pub struct FinancialIdentityAdapter;

impl admission::authenticated_principal::WorthQueryAuthenticationAdapter
    for FinancialIdentityAdapter
{
    type Credential = ();

    fn configuration_identity(&self) -> &str {
        "worth-query-financial-courtroom-adapter-v1"
    }

    fn validate<'a>(
        &'a self,
        _credential: (),
        _scope: &'a admission::authenticated_principal::WorthQueryRequestScope,
    ) -> admission::authenticated_principal::WorthQueryAuthenticationFuture<'a> {
        Box::pin(async move {
            let now = SystemTime::now();
            admission::authenticated_principal::WorthQueryValidatedExternalPrincipal::new(
                worth_query_host::facade::declaration::authentication::WorthQueryExternalPrincipalIdentity::new(
                    "https://issuer.example",
                    "financial-courtroom",
                ).unwrap(),
                admission::authenticated_principal::WorthQueryAuthenticationAudience::new("host").unwrap(),
                admission::authenticated_principal::WorthQueryAuthenticationMethod::new("test").unwrap(),
                now,
                now + Duration::from_secs(3600),
                Vec::new(),
            ).map_err(|_| admission::authenticated_principal::WorthQueryAuthenticationAdapterFailure::new(
                admission::authenticated_principal::WorthQueryAuthenticationAdapterFailureKind::ProtocolViolation,
            ))
        })
    }
}

pub fn request_scope() -> admission::authenticated_principal::WorthQueryRequestScope {
    let cancellation = admission::authenticated_principal::WorthQueryCancellationSource::new();
    admission::authenticated_principal::WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        cancellation.token(),
    )
}

pub fn block_on<F: Future>(future: F) -> F::Output {
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

pub type FinancialAuthentication =
    admission::authenticated_principal::WorthQueryAdmittedAuthenticationAdapter<
        FinancialHostSchema,
        FinancialIdentityAdapter,
    >;

pub fn admitted_identity_adapter(
    schema: &domain::WorthQueryInstalledApplicationSchema<FinancialHostSchema>,
) -> FinancialAuthentication {
    admission::authenticated_principal::admit_authentication_adapter(
        schema,
        admission::authenticated_principal::WorthQueryAuthenticationAdapterAdmission::new(
            admission::authenticated_principal::WorthQueryAuthenticationAudience::new("host")
                .unwrap(),
            admission::authenticated_principal::WorthQueryAuthenticationMethod::new("test")
                .unwrap(),
        ),
        FinancialIdentityAdapter,
    )
    .unwrap()
}

pub struct FinancialPrincipalSource(pub Arc<FinancialAuthentication>);

impl primary_graph::WorthQueryTemporalPrincipalSource<FinancialHostSchema>
    for FinancialPrincipalSource
{
    const SEMANTIC_IDENTITY: &'static str = "worth.query.financial.principal-source";

    fn admit(
        &self,
    ) -> Result<
        primary_graph::WorthQueryTemporalPrincipalAdmission<FinancialHostSchema>,
        primary_graph::WorthQueryTemporalPrincipalFailure,
    > {
        let scope = request_scope();
        let external = block_on(self.0.authenticate((), &scope)).map_err(|denial| {
            primary_graph::WorthQueryTemporalPrincipalFailure::new(
                primary_graph::WorthQueryTemporalPrincipalFailureKind::AdmissionRejected,
                format!("{denial:?}"),
            )
        })?;
        Ok(primary_graph::WorthQueryTemporalPrincipalAdmission::new(
            external, scope,
        ))
    }
}
