//! Freshness classification against an owner-supplied source.
//!
//! The observation moment is sampled inside [`evaluate_freshness`]. Callers
//! cannot pass a reading at which their basis still looked current. The result
//! retains both the source type and the policy type, so evidence produced by a
//! caller-defined clock or classifier cannot satisfy an owner API expecting
//! its own source and policy.
//!
//! **The establishment moment is not sealed here.** Only the *observation* is.
//! `Basis` is whatever the owner hands in, so if it carries the generation or
//! deadline the assumption was established at, that value came from the
//! caller of [`evaluate_freshness`] — this module cannot tell a real
//! establishment stamp from a flattering one. Freshness is therefore a
//! statement about a basis, not a warrant that the basis is honest. An owner
//! who needs both must seal `Basis` on its own side: give it a private field
//! and mint it only where the assumption was actually established, exactly as
//! [`FreshnessSample`] does for the observation. A `Basis` any caller can
//! construct means any caller can choose to look current.
//!
//! The source remains runtime-owned. This module supplies only the legality
//! law and typed result; it owns no clock, counter, retry, or live table.

use std::fmt::{self, Debug as FmtDebug};
use std::marker::PhantomData;

use super::{
    AuthorityRevalidationRequired, CurrentValidity, FreshnessScopedBasis, RebindRequired,
    StaleReadable,
};

/// A host-supplied ordering of moments: a clock, generation, or commit ordinal.
pub trait FreshnessSource {
    type Sample;
    type Error;

    /// Read the source now.
    ///
    /// Source failure stays explicit. A clock that cannot answer may not mint
    /// a guessed freshness class.
    fn sample(&self) -> Result<Self::Sample, Self::Error>;
}

/// A reading obtained from one exact source type.
///
/// There is no public constructor. Raw readings are descriptive values;
/// `FreshnessSample<S>` is evidence of which owner source produced one.
pub struct FreshnessSample<S>
where
    S: FreshnessSource,
{
    value: S::Sample,
    source: PhantomData<fn() -> S>,
}

impl<S> FreshnessSample<S>
where
    S: FreshnessSource,
{
    fn seal(value: S::Sample) -> Self {
        Self {
            value,
            source: PhantomData,
        }
    }

    pub fn value(&self) -> &S::Sample {
        &self.value
    }

    pub fn into_value(self) -> S::Sample {
        self.value
    }
}

impl<S> FmtDebug for FreshnessSample<S>
where
    S: FreshnessSource,
    S::Sample: FmtDebug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("FreshnessSample")
            .field(&self.value)
            .finish()
    }
}

impl<S> PartialEq for FreshnessSample<S>
where
    S: FreshnessSource,
    S::Sample: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<S> Eq for FreshnessSample<S>
where
    S: FreshnessSource,
    S::Sample: Eq,
{
}

/// Obtain a source-bound sample without exposing a construction door.
pub fn take_sample<S>(source: &S) -> Result<FreshnessSample<S>, S::Error>
where
    S: FreshnessSource,
{
    source.sample().map(FreshnessSample::seal)
}

/// Which freshness class an owner policy selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshnessVerdict {
    Current,
    StaleReadable,
    RebindRequired,
    AuthorityRevalidationRequired,
}

/// Owner policy mapping a basis and current source observation to freshness.
///
/// `Basis` may contain an establishment generation, expiry deadline, lease, or
/// any other immutable fact the policy needs. The policy type is retained in
/// the output, preventing a caller-selected classifier from laundering itself
/// into owner evidence.
pub trait FreshnessPolicy<S, Basis>
where
    S: FreshnessSource,
{
    fn classify(&self, basis: &Basis, observed_at: &S::Sample) -> FreshnessVerdict;
}

/// Basis plus the exact source observation and policy that classified it.
pub struct FreshnessEvaluation<S, Policy, Basis>
where
    S: FreshnessSource,
    Policy: FreshnessPolicy<S, Basis>,
{
    basis: Basis,
    observed_at: FreshnessSample<S>,
    policy: PhantomData<fn() -> Policy>,
}

impl<S, Policy, Basis> FreshnessEvaluation<S, Policy, Basis>
where
    S: FreshnessSource,
    Policy: FreshnessPolicy<S, Basis>,
{
    pub fn basis(&self) -> &Basis {
        &self.basis
    }

    pub fn observed_at(&self) -> &FreshnessSample<S> {
        &self.observed_at
    }

    pub fn into_basis(self) -> Basis {
        self.basis
    }

    pub fn into_parts(self) -> (Basis, FreshnessSample<S>) {
        (self.basis, self.observed_at)
    }
}

impl<S, Policy, Basis> FmtDebug for FreshnessEvaluation<S, Policy, Basis>
where
    S: FreshnessSource,
    S::Sample: FmtDebug,
    Policy: FreshnessPolicy<S, Basis>,
    Basis: FmtDebug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FreshnessEvaluation")
            .field("basis", &self.basis)
            .field("observed_at", &self.observed_at)
            .finish()
    }
}

/// Typed result of one source-bound evaluation.
pub enum EvaluatedFreshness<S, Policy, Basis>
where
    S: FreshnessSource,
    Policy: FreshnessPolicy<S, Basis>,
{
    Current(FreshnessScopedBasis<CurrentValidity, FreshnessEvaluation<S, Policy, Basis>>),
    StaleReadable(FreshnessScopedBasis<StaleReadable, FreshnessEvaluation<S, Policy, Basis>>),
    RebindRequired(FreshnessScopedBasis<RebindRequired, FreshnessEvaluation<S, Policy, Basis>>),
    AuthorityRevalidationRequired(
        FreshnessScopedBasis<AuthorityRevalidationRequired, FreshnessEvaluation<S, Policy, Basis>>,
    ),
}

/// Classify a basis using an observation sampled at this boundary.
pub fn evaluate_freshness<S, Policy, Basis>(
    basis: Basis,
    source: &S,
    policy: &Policy,
) -> Result<EvaluatedFreshness<S, Policy, Basis>, S::Error>
where
    S: FreshnessSource,
    Policy: FreshnessPolicy<S, Basis>,
{
    let observed_at = take_sample(source)?;
    let verdict = policy.classify(&basis, observed_at.value());
    let evaluation = FreshnessEvaluation {
        basis,
        observed_at,
        policy: PhantomData,
    };

    Ok(match verdict {
        FreshnessVerdict::Current => {
            EvaluatedFreshness::Current(FreshnessScopedBasis::new(evaluation))
        }
        FreshnessVerdict::StaleReadable => {
            EvaluatedFreshness::StaleReadable(FreshnessScopedBasis::new(evaluation))
        }
        FreshnessVerdict::RebindRequired => {
            EvaluatedFreshness::RebindRequired(FreshnessScopedBasis::new(evaluation))
        }
        FreshnessVerdict::AuthorityRevalidationRequired => {
            EvaluatedFreshness::AuthorityRevalidationRequired(FreshnessScopedBasis::new(evaluation))
        }
    })
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::convert::Infallible;

    use super::{
        evaluate_freshness, take_sample, EvaluatedFreshness, FreshnessPolicy, FreshnessSource,
        FreshnessVerdict,
    };

    struct Generations {
        now: Cell<u64>,
    }

    impl Generations {
        fn starting_at(now: u64) -> Self {
            Self {
                now: Cell::new(now),
            }
        }

        fn advance(&self, by: u64) {
            self.now.set(self.now.get() + by);
        }
    }

    impl FreshnessSource for Generations {
        type Sample = u64;
        type Error = Infallible;

        fn sample(&self) -> Result<u64, Infallible> {
            Ok(self.now.get())
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct EstablishedAt(u64);

    struct AgePolicy;

    impl FreshnessPolicy<Generations, EstablishedAt> for AgePolicy {
        fn classify(&self, basis: &EstablishedAt, observed_at: &u64) -> FreshnessVerdict {
            match observed_at.saturating_sub(basis.0) {
                0 => FreshnessVerdict::Current,
                1..=3 => FreshnessVerdict::StaleReadable,
                4..=9 => FreshnessVerdict::RebindRequired,
                _ => FreshnessVerdict::AuthorityRevalidationRequired,
            }
        }
    }

    fn evaluate_after(elapsed: u64) -> EvaluatedFreshness<Generations, AgePolicy, EstablishedAt> {
        let source = Generations::starting_at(100);
        source.advance(elapsed);
        evaluate_freshness(EstablishedAt(100), &source, &AgePolicy)
            .expect("the generation source is infallible")
    }

    #[test]
    fn a_sample_records_what_the_source_read() {
        let source = Generations::starting_at(42);

        assert_eq!(take_sample(&source).unwrap().value(), &42);
        source.advance(1);
        assert_eq!(take_sample(&source).unwrap().value(), &43);
    }

    #[test]
    fn every_verdict_produces_its_own_typed_basis() {
        assert!(matches!(evaluate_after(0), EvaluatedFreshness::Current(_)));
        assert!(matches!(
            evaluate_after(2),
            EvaluatedFreshness::StaleReadable(_)
        ));
        assert!(matches!(
            evaluate_after(5),
            EvaluatedFreshness::RebindRequired(_)
        ));
        assert!(matches!(
            evaluate_after(50),
            EvaluatedFreshness::AuthorityRevalidationRequired(_)
        ));
    }

    #[test]
    fn evaluation_retains_basis_and_exact_observation() {
        let EvaluatedFreshness::Current(scoped) = evaluate_after(0) else {
            panic!("equal generations are current");
        };

        assert_eq!(scoped.basis().basis(), &EstablishedAt(100));
        assert_eq!(scoped.basis().observed_at().value(), &100);
        assert_eq!(scoped.into_basis().into_basis(), EstablishedAt(100));
    }

    #[test]
    fn the_observation_moment_comes_from_the_source() {
        let source = Generations::starting_at(0);
        source.advance(20);

        let evaluated = evaluate_freshness(EstablishedAt(0), &source, &AgePolicy).unwrap();
        assert!(matches!(
            evaluated,
            EvaluatedFreshness::AuthorityRevalidationRequired(_)
        ));
    }

    struct FailingSource;

    impl FreshnessSource for FailingSource {
        type Sample = u64;
        type Error = &'static str;

        fn sample(&self) -> Result<u64, Self::Error> {
            Err("clock unavailable")
        }
    }

    struct FailingPolicy;

    impl FreshnessPolicy<FailingSource, ()> for FailingPolicy {
        fn classify(&self, _basis: &(), _observed_at: &u64) -> FreshnessVerdict {
            FreshnessVerdict::Current
        }
    }

    #[test]
    fn source_failure_cannot_be_relabelled_as_freshness() {
        assert!(matches!(
            evaluate_freshness((), &FailingSource, &FailingPolicy),
            Err("clock unavailable")
        ));
    }
}
