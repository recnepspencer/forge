use crate::authoring::QueryFamily;
use crate::identity::CollectionPlanDigest;
use crate::validation::ValidatedQueryBundle;

use super::{
    CollectionOrderingBasis, CollectionOrderingDirection, CollectionOrderingEntry,
    CollectionPlanningMode, CollectionResultFamily, CollectionWindowPolicy, CursorAdvanceContract,
    MaterializationBreadthClass, OrderingKeyPath, PostReadShapingPlan, TraversalBoundContract,
    TraversalDepthLimit, TraversalEdgeClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionPlanningContext {
    query_family: QueryFamily,
    result_family: CollectionResultFamily,
}

impl CollectionPlanningContext {
    pub fn query_family(&self) -> &QueryFamily {
        &self.query_family
    }

    pub fn result_family(&self) -> &CollectionResultFamily {
        &self.result_family
    }

    fn new(result_family: CollectionResultFamily) -> Self {
        Self {
            query_family: QueryFamily::Collection,
            result_family,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionPlanBundle {
    digest: CollectionPlanDigest,
    planning_context: CollectionPlanningContext,
    ordering_basis: CollectionOrderingBasis,
    window_policy: CollectionWindowPolicy,
    cursor_contract: CursorAdvanceContract,
    traversal_bound: TraversalBoundContract,
    post_read_shaping: PostReadShapingPlan,
}

impl CollectionPlanBundle {
    pub fn digest(&self) -> &CollectionPlanDigest {
        &self.digest
    }

    pub fn planning_context(&self) -> &CollectionPlanningContext {
        &self.planning_context
    }

    pub fn ordering_basis(&self) -> &CollectionOrderingBasis {
        &self.ordering_basis
    }

    pub fn window_policy(&self) -> &CollectionWindowPolicy {
        &self.window_policy
    }

    pub fn cursor_contract(&self) -> &CursorAdvanceContract {
        &self.cursor_contract
    }

    pub fn traversal_bound(&self) -> &TraversalBoundContract {
        &self.traversal_bound
    }

    pub fn post_read_shaping(&self) -> &PostReadShapingPlan {
        &self.post_read_shaping
    }

    pub fn from_validated_bundle_for_mode(
        bundle: &ValidatedQueryBundle,
        mode: CollectionPlanningMode,
        planned_projection_count: usize,
    ) -> Option<Self> {
        if bundle.query().family() != &QueryFamily::Collection {
            return None;
        }

        let ordering_basis = collection_ordering_basis(bundle);
        let traversal_bound = collection_traversal_bound(bundle);
        let input_breadth = planned_projection_count
            + bundle.query().predicates().entries().len()
            + bundle.query().traversal().len()
            + bundle.query().ordering().entries().len();
        let post_read_shaping = PostReadShapingPlan::for_mode(input_breadth, &mode);
        let planning_context =
            CollectionPlanningContext::new(post_read_shaping.result_family().clone());
        let window_policy = CollectionWindowPolicy::FullSnapshotRead;
        let cursor_contract = CursorAdvanceContract::BasisBoundOpaque;
        let digest = collection_plan_digest(
            &planning_context,
            &ordering_basis,
            &window_policy,
            &cursor_contract,
            &traversal_bound,
            &post_read_shaping,
        );

        Some(Self {
            digest,
            planning_context,
            ordering_basis,
            window_policy,
            cursor_contract,
            traversal_bound,
            post_read_shaping,
        })
    }
}

fn collection_ordering_basis(bundle: &ValidatedQueryBundle) -> CollectionOrderingBasis {
    let entries = if bundle.query().ordering().entries().is_empty() {
        vec![CollectionOrderingEntry::new(
            OrderingKeyPath::new("identity", "id"),
            CollectionOrderingDirection::Ascending,
        )]
    } else {
        bundle
            .query()
            .ordering()
            .entries()
            .iter()
            .map(|entry| {
                CollectionOrderingEntry::new(
                    OrderingKeyPath::from_native_keys(
                        entry.native_aspect_key().clone(),
                        entry.native_field_key().clone(),
                    ),
                    CollectionOrderingDirection::from_validated_direction(entry.direction()),
                )
            })
            .collect()
    };
    CollectionOrderingBasis::new(entries)
}

fn collection_traversal_bound(bundle: &ValidatedQueryBundle) -> TraversalBoundContract {
    let max_depth = bundle
        .query()
        .traversal()
        .iter()
        .map(|entry| entry.max_depth())
        .max()
        .unwrap_or(0);
    let edge_classes = bundle
        .query()
        .traversal()
        .iter()
        .map(|entry| TraversalEdgeClass::new(entry.relation_name().as_str()))
        .collect();
    TraversalBoundContract::new(
        TraversalDepthLimit::new(max_depth),
        edge_classes,
        if bundle.query().traversal().is_empty() {
            MaterializationBreadthClass::ScalarOnly
        } else {
            MaterializationBreadthClass::RootPlusTraversal
        },
    )
}

fn collection_plan_digest(
    planning_context: &CollectionPlanningContext,
    ordering_basis: &CollectionOrderingBasis,
    window_policy: &CollectionWindowPolicy,
    cursor_contract: &CursorAdvanceContract,
    traversal_bound: &TraversalBoundContract,
    post_read_shaping: &PostReadShapingPlan,
) -> CollectionPlanDigest {
    let mut digest_parts = vec![
        format!("query_family:{:?}", planning_context.query_family()),
        format!(
            "result_family:{}",
            planning_context.result_family().digest_label()
        ),
        window_policy.digest_part(),
        cursor_contract.digest_part(),
    ];
    digest_parts.extend(ordering_basis.digest_parts());
    digest_parts.extend(traversal_bound.digest_parts());
    digest_parts.extend(post_read_shaping.digest_parts());
    CollectionPlanDigest::from_parts(&digest_parts)
}
