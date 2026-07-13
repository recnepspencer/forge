use worth_query::facade::foundation::LivePromotionDescriptor;
use worth_query::facade::runtime::{LiveQueryAdmissionArtifact, QuerySubscriptionAdmissionDimensions, QuerySubscriptionBasisPosture};

fn main() {
    let descriptor: LivePromotionDescriptor = todo!();
    let dimensions: QuerySubscriptionAdmissionDimensions = todo!();
    let _ = LiveQueryAdmissionArtifact::from_live_promotion(
        &descriptor,
        QuerySubscriptionBasisPosture::CurrentHead,
        dimensions,
    );
}
