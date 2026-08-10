use super::super::failure::SupportTrustFailure;
use super::access_closeout::SubscriptionSupportAccuracyAccessCloseout;
use super::digest::stable_digest;
use super::performance_closeout::SubscriptionSupportAccuracyPerformanceCloseout;
use super::persistence_posture::SubscriptionSupportAccuracyPersistencePosture;
use super::suite::SubscriptionSupportAccuracyCertificationSuite;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyCertificationRun {
    suite: SubscriptionSupportAccuracyCertificationSuite,
    performance_closeout: SubscriptionSupportAccuracyPerformanceCloseout,
    access_closeout: SubscriptionSupportAccuracyAccessCloseout,
    persistence_posture: SubscriptionSupportAccuracyPersistencePosture,
    run_digest: String,
}

impl SubscriptionSupportAccuracyCertificationRun {
    pub(super) fn from_closeouts(
        suite: SubscriptionSupportAccuracyCertificationSuite,
        performance_closeout: SubscriptionSupportAccuracyPerformanceCloseout,
        access_closeout: SubscriptionSupportAccuracyAccessCloseout,
        persistence_posture: SubscriptionSupportAccuracyPersistencePosture,
    ) -> Result<Self, SupportTrustFailure> {
        let run_digest = stable_digest(&SubscriptionSupportAccuracyCertificationRunDigestBasis {
            suite_digest: suite.suite_digest(),
            performance_closeout: &performance_closeout,
            access_closeout: &access_closeout,
            persistence_posture,
        })?;
        Ok(Self {
            suite,
            performance_closeout,
            access_closeout,
            persistence_posture,
            run_digest,
        })
    }

    pub fn suite(&self) -> &SubscriptionSupportAccuracyCertificationSuite {
        &self.suite
    }

    pub fn performance_closeout(&self) -> &SubscriptionSupportAccuracyPerformanceCloseout {
        &self.performance_closeout
    }

    pub fn access_closeout(&self) -> &SubscriptionSupportAccuracyAccessCloseout {
        &self.access_closeout
    }

    pub fn persistence_posture(&self) -> SubscriptionSupportAccuracyPersistencePosture {
        self.persistence_posture
    }

    pub fn run_digest(&self) -> &str {
        &self.run_digest
    }
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyCertificationRunDigestBasis<'a> {
    suite_digest: &'a str,
    performance_closeout: &'a SubscriptionSupportAccuracyPerformanceCloseout,
    access_closeout: &'a SubscriptionSupportAccuracyAccessCloseout,
    persistence_posture: SubscriptionSupportAccuracyPersistencePosture,
}
