use worth_store::{
    SubscriptionSupportAccuracyCertificationRunner, SubscriptionSupportAccuracyPersistencePosture,
};

fn main() {
    let _ = SubscriptionSupportAccuracyCertificationRunner {
        persistence_posture: SubscriptionSupportAccuracyPersistencePosture::InMemoryCertificationOnly,
    };
}
