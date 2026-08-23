use worth_query_decl::facade::application_schema::{
    ApplicationAspectMarkerIdentity, AspectContractRevision, AspectIdentity,
};
use worth_query_decl::facade::{worth_query_aspect, worth_query_entity};

pub struct StableSchema;

worth_query_entity!(pub StableEntity in StableSchema);
worth_query_aspect!(
    pub StableAspect in StableSchema, StableEntity;
    identity = AspectIdentity(42),
    revision = AspectContractRevision(3),
);

fn main() {
    assert_eq!(StableAspect::ASPECT_IDENTITY, AspectIdentity(42));
    assert_eq!(StableAspect::CONTRACT_REVISION, AspectContractRevision(3));
}
