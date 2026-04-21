use forge_query::facade::{
    narrow_policy_query, AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath,
    DetailQueryBuilder, DetailResultShapeBuilder, PolicyAspectMask, PolicyMaskSnapshot,
    PolicyInfluenceSet, RelationshipProofDescriptorSet, RootEntityKey,
};

fn main() {
    let query = DetailQueryBuilder::new(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let shape = DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    let canonical = GuidedAuthoringPath::canonicalize_detail(query, shape).unwrap();

    let _ = narrow_policy_query(
        &canonical,
        canonical.query().clone(),
        PolicyMaskSnapshot::synthetic_authority("policy", PolicyAspectMask::allow_all()),
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    );
}
