use forge_store::{
    SubscriptionSupportAccuracyAccessCloseout, SubscriptionSupportAccuracyCertificationRun,
    SubscriptionSupportAccuracyCertificationSuite, SubscriptionSupportAccuracyPerformanceCloseout,
    SubscriptionSupportAccuracyPersistencePosture,
};

fn main() {
    let suite: Option<SubscriptionSupportAccuracyCertificationSuite> = None;
    let performance: Option<SubscriptionSupportAccuracyPerformanceCloseout> = None;
    let access: Option<SubscriptionSupportAccuracyAccessCloseout> = None;
    let _ = SubscriptionSupportAccuracyCertificationRun {
        suite: suite.unwrap(),
        performance_closeout: performance.unwrap(),
        access_closeout: access.unwrap(),
        persistence_posture: SubscriptionSupportAccuracyPersistencePosture::InMemoryCertificationOnly,
        run_digest: String::from("synthetic:run"),
    };
}
