use crate::basis::{BasisPreflightError, ExecutionPreflightBundle, ResolvedSnapshotBasis};
use crate::collection::CollectionPlanBundle;
use crate::execution::{execute_preflight_bundle, ExecutionError};
use crate::identity::{hash_parts, CollectionPlanDigest, PlanDigest, ValidatedQueryDigest};
use crate::live_performance::{IncrementalPatchEligibility, LivePerformanceReport};
use crate::validation::ValidatedQueryBundle;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LiveQueryFamily {
    Detail,
    OrderedCollection,
    BoundedMaterialization,
}

impl LiveQueryFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Detail => "detail",
            Self::OrderedCollection => "ordered_collection",
            Self::BoundedMaterialization => "bounded_materialization",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct QueryFieldKey {
    aspect: String,
    field: String,
}

impl QueryFieldKey {
    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    fn new(aspect: impl Into<String>, field: impl Into<String>) -> Self {
        Self {
            aspect: aspect.into(),
            field: field.into(),
        }
    }

    fn matches(&self, delta: &BridgeFieldDelta) -> bool {
        self.aspect == delta.aspect && self.field == delta.field
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryRelevanceContract {
    family: LiveQueryFamily,
    projected_fields: Vec<QueryFieldKey>,
    ordering_fields: Vec<QueryFieldKey>,
    traversal_relations: Vec<String>,
}

impl QueryRelevanceContract {
    pub fn family(&self) -> &LiveQueryFamily {
        &self.family
    }

    pub fn projected_fields(&self) -> &[QueryFieldKey] {
        &self.projected_fields
    }

    pub fn ordering_fields(&self) -> &[QueryFieldKey] {
        &self.ordering_fields
    }

    pub fn traversal_relations(&self) -> &[String] {
        &self.traversal_relations
    }

    fn for_detail(bundle: &ValidatedQueryBundle) -> Self {
        Self {
            family: LiveQueryFamily::Detail,
            projected_fields: bundle
                .query()
                .projection()
                .iter()
                .map(|entry| QueryFieldKey::new(entry.aspect(), entry.field()))
                .collect(),
            ordering_fields: Vec::new(),
            traversal_relations: Vec::new(),
        }
    }

    fn for_ordered_collection(
        bundle: &ValidatedQueryBundle,
        _collection: &CollectionPlanBundle,
    ) -> Self {
        Self {
            family: LiveQueryFamily::OrderedCollection,
            projected_fields: bundle
                .query()
                .projection()
                .iter()
                .map(|entry| QueryFieldKey::new(entry.aspect(), entry.field()))
                .collect(),
            ordering_fields: bundle
                .query()
                .ordering()
                .entries()
                .iter()
                .map(|entry| QueryFieldKey::new(entry.aspect(), entry.field()))
                .collect(),
            traversal_relations: Vec::new(),
        }
    }

    fn for_bounded_materialization(
        bundle: &ValidatedQueryBundle,
        collection: &CollectionPlanBundle,
    ) -> Self {
        Self {
            family: LiveQueryFamily::BoundedMaterialization,
            projected_fields: bundle
                .query()
                .projection()
                .iter()
                .map(|entry| QueryFieldKey::new(entry.aspect(), entry.field()))
                .collect(),
            ordering_fields: bundle
                .query()
                .ordering()
                .entries()
                .iter()
                .map(|entry| QueryFieldKey::new(entry.aspect(), entry.field()))
                .collect(),
            traversal_relations: collection
                .traversal_bound()
                .edge_classes()
                .iter()
                .map(|entry| entry.as_str().to_string())
                .collect(),
        }
    }

    pub fn classify_change(&self, change: &BridgeChangeSummary) -> ChangeRelevance {
        let projected_overlap = change.field_deltas.iter().any(|delta| {
            self.projected_fields
                .iter()
                .any(|field| field.matches(delta))
        });
        let ordering_overlap = change.field_deltas.iter().any(|delta| {
            self.ordering_fields
                .iter()
                .any(|field| field.matches(delta))
        });
        let traversal_overlap = change.relation_deltas.iter().any(|delta| {
            self.traversal_relations
                .iter()
                .any(|relation| relation == &delta.relation)
        });

        match self.family {
            LiveQueryFamily::Detail => {
                if projected_overlap {
                    ChangeRelevance::Relevant(RelevantChangeClass::DetailProjectionChange)
                } else {
                    ChangeRelevance::Irrelevant(IrrelevantChangeClass::NoProjectedFieldOverlap)
                }
            }
            LiveQueryFamily::OrderedCollection => {
                if change.membership_changed() {
                    ChangeRelevance::Relevant(
                        RelevantChangeClass::OrderedCollectionMembershipChange,
                    )
                } else if ordering_overlap {
                    ChangeRelevance::Relevant(RelevantChangeClass::OrderedCollectionOrderingChange)
                } else if projected_overlap {
                    ChangeRelevance::Relevant(RelevantChangeClass::DetailProjectionChange)
                } else {
                    ChangeRelevance::Irrelevant(IrrelevantChangeClass::NoProjectedFieldOverlap)
                }
            }
            LiveQueryFamily::BoundedMaterialization => {
                if change.materialization_scope_changed() || traversal_overlap {
                    ChangeRelevance::Relevant(
                        RelevantChangeClass::BoundedMaterializationScopeChange,
                    )
                } else if change.membership_changed() {
                    ChangeRelevance::Relevant(
                        RelevantChangeClass::OrderedCollectionMembershipChange,
                    )
                } else if ordering_overlap {
                    ChangeRelevance::Relevant(RelevantChangeClass::OrderedCollectionOrderingChange)
                } else if projected_overlap {
                    ChangeRelevance::Relevant(RelevantChangeClass::DetailProjectionChange)
                } else {
                    ChangeRelevance::Irrelevant(IrrelevantChangeClass::NoProjectedFieldOverlap)
                }
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeFieldDelta {
    aspect: String,
    field: String,
    old_value: Option<String>,
    new_value: Option<String>,
}

impl BridgeFieldDelta {
    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn old_value(&self) -> Option<&str> {
        self.old_value.as_deref()
    }

    pub fn new_value(&self) -> Option<&str> {
        self.new_value.as_deref()
    }

    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        old_value: Option<impl Into<String>>,
        new_value: Option<impl Into<String>>,
    ) -> Self {
        Self {
            aspect: aspect.into(),
            field: field.into(),
            old_value: old_value.map(Into::into),
            new_value: new_value.map(Into::into),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeRelationDelta {
    relation: String,
}

impl BridgeRelationDelta {
    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn new(relation: impl Into<String>) -> Self {
        Self {
            relation: relation.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BridgeSliceCategory {
    EntityRegion,
    EntityPartition,
    CoarseFallback,
}

impl BridgeSliceCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EntityRegion => "entity_region",
            Self::EntityPartition => "entity_partition",
            Self::CoarseFallback => "coarse_fallback",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeLocalitySlice {
    category: BridgeSliceCategory,
    scope: String,
}

impl BridgeLocalitySlice {
    pub fn category(&self) -> &BridgeSliceCategory {
        &self.category
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn region(scope: impl Into<String>) -> Self {
        Self {
            category: BridgeSliceCategory::EntityRegion,
            scope: scope.into(),
        }
    }

    pub fn partition(scope: impl Into<String>) -> Self {
        Self {
            category: BridgeSliceCategory::EntityPartition,
            scope: scope.into(),
        }
    }

    pub fn coarse_fallback(scope: impl Into<String>) -> Self {
        Self {
            category: BridgeSliceCategory::CoarseFallback,
            scope: scope.into(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BridgeChangeSummary {
    field_deltas: Vec<BridgeFieldDelta>,
    relation_deltas: Vec<BridgeRelationDelta>,
    membership_transition: Option<MembershipTransition>,
    materialization_scope_transition: Option<MaterializationScopeTransition>,
    locality_slices: Vec<BridgeLocalitySlice>,
}

impl BridgeChangeSummary {
    pub fn field_deltas(&self) -> &[BridgeFieldDelta] {
        &self.field_deltas
    }

    pub fn relation_deltas(&self) -> &[BridgeRelationDelta] {
        &self.relation_deltas
    }

    pub fn membership_changed(&self) -> bool {
        self.membership_transition
            .as_ref()
            .is_some_and(MembershipTransition::changed)
    }

    pub fn materialization_scope_changed(&self) -> bool {
        self.materialization_scope_transition
            .as_ref()
            .is_some_and(MaterializationScopeTransition::changed)
    }

    pub fn membership_transition(&self) -> Option<&MembershipTransition> {
        self.membership_transition.as_ref()
    }

    pub fn materialization_scope_transition(&self) -> Option<&MaterializationScopeTransition> {
        self.materialization_scope_transition.as_ref()
    }

    pub fn locality_slices(&self) -> &[BridgeLocalitySlice] {
        &self.locality_slices
    }

    pub fn with_field_delta(mut self, delta: BridgeFieldDelta) -> Self {
        self.field_deltas.push(delta);
        self
    }

    pub fn with_relation_delta(mut self, delta: BridgeRelationDelta) -> Self {
        self.relation_deltas.push(delta);
        self
    }

    pub fn with_membership_transition(mut self, was_member: bool, is_member: bool) -> Self {
        self.membership_transition = Some(MembershipTransition::new(was_member, is_member));
        self
    }

    pub fn with_materialization_scope_transition(
        mut self,
        was_in_scope: bool,
        is_in_scope: bool,
    ) -> Self {
        self.materialization_scope_transition = Some(MaterializationScopeTransition::new(
            was_in_scope,
            is_in_scope,
        ));
        self
    }

    pub fn with_locality_slice(mut self, slice: BridgeLocalitySlice) -> Self {
        self.locality_slices.push(slice);
        self
    }

    pub fn with_region_slice(self, scope: impl Into<String>) -> Self {
        self.with_locality_slice(BridgeLocalitySlice::region(scope))
    }

    pub fn with_partition_slice(self, scope: impl Into<String>) -> Self {
        self.with_locality_slice(BridgeLocalitySlice::partition(scope))
    }

    pub fn with_coarse_fallback_slice(self, scope: impl Into<String>) -> Self {
        self.with_locality_slice(BridgeLocalitySlice::coarse_fallback(scope))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LocalityScopeKind {
    Region,
    Partition,
}

impl LocalityScopeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Region => "region",
            Self::Partition => "partition",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LocalityScopeDigest(String);

impl LocalityScopeDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalityPredicateContract {
    scope_kind: LocalityScopeKind,
    scope: String,
    digest: LocalityScopeDigest,
}

impl LocalityPredicateContract {
    pub fn scope_kind(&self) -> &LocalityScopeKind {
        &self.scope_kind
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn digest(&self) -> &LocalityScopeDigest {
        &self.digest
    }

    pub fn region(scope: impl Into<String>) -> Self {
        let scope = scope.into();
        let digest =
            LocalityScopeDigest::from_parts(&["kind:region".to_string(), format!("scope:{scope}")]);
        Self {
            scope_kind: LocalityScopeKind::Region,
            scope,
            digest,
        }
    }

    pub fn partition(scope: impl Into<String>) -> Self {
        let scope = scope.into();
        let digest = LocalityScopeDigest::from_parts(&[
            "kind:partition".to_string(),
            format!("scope:{scope}"),
        ]);
        Self {
            scope_kind: LocalityScopeKind::Partition,
            scope,
            digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipTransition {
    was_member: bool,
    is_member: bool,
}

impl MembershipTransition {
    pub fn was_member(&self) -> bool {
        self.was_member
    }

    pub fn is_member(&self) -> bool {
        self.is_member
    }

    pub fn changed(&self) -> bool {
        self.was_member != self.is_member
    }

    fn new(was_member: bool, is_member: bool) -> Self {
        Self {
            was_member,
            is_member,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaterializationScopeTransition {
    was_in_scope: bool,
    is_in_scope: bool,
}

impl MaterializationScopeTransition {
    pub fn was_in_scope(&self) -> bool {
        self.was_in_scope
    }

    pub fn is_in_scope(&self) -> bool {
        self.is_in_scope
    }

    pub fn changed(&self) -> bool {
        self.was_in_scope != self.is_in_scope
    }

    fn new(was_in_scope: bool, is_in_scope: bool) -> Self {
        Self {
            was_in_scope,
            is_in_scope,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelevantChangeClass {
    DetailProjectionChange,
    OrderedCollectionMembershipChange,
    OrderedCollectionOrderingChange,
    BoundedMaterializationScopeChange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IrrelevantChangeClass {
    NoProjectedFieldOverlap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChangeRelevance {
    Relevant(RelevantChangeClass),
    Irrelevant(IrrelevantChangeClass),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RefreshAdmissionClass {
    WidthOverflow,
}

impl RefreshAdmissionClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WidthOverflow => "width_overflow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefreshAdmissionMatrix {
    admitted_classes: Vec<RefreshAdmissionClass>,
}

impl RefreshAdmissionMatrix {
    pub fn admitted_classes(&self) -> &[RefreshAdmissionClass] {
        &self.admitted_classes
    }

    pub fn admits(&self, class: &RefreshAdmissionClass) -> bool {
        self.admitted_classes.contains(class)
    }

    fn detail_family() -> Self {
        Self {
            admitted_classes: Vec::new(),
        }
    }

    fn ordered_collection_family() -> Self {
        Self {
            admitted_classes: Vec::new(),
        }
    }

    fn bounded_materialization_family() -> Self {
        Self {
            admitted_classes: vec![RefreshAdmissionClass::WidthOverflow],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefreshFallback {
    admission_class: RefreshAdmissionClass,
    cost_class: crate::live_performance::RefreshCostClass,
    admission_status: crate::live_performance::RefreshAdmissionStatus,
}

impl RefreshFallback {
    pub fn admission_class(&self) -> &RefreshAdmissionClass {
        &self.admission_class
    }

    pub fn cost_class(&self) -> &crate::live_performance::RefreshCostClass {
        &self.cost_class
    }

    pub fn admission_status(&self) -> &crate::live_performance::RefreshAdmissionStatus {
        &self.admission_status
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PatchWidthResolution {
    Deliver,
    Coalesce,
    Refresh(RefreshFallback),
    Reject,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatchWidthAssessment {
    measured_width: usize,
    budget_limit: usize,
    resolution: PatchWidthResolution,
}

impl PatchWidthAssessment {
    pub fn measured_width(&self) -> usize {
        self.measured_width
    }

    pub fn budget_limit(&self) -> usize {
        self.budget_limit
    }

    pub fn resolution(&self) -> &PatchWidthResolution {
        &self.resolution
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CoalescingDecision {
    NotNeeded,
    Admitted { bundle_count: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveCoalescingError {
    BundleCountTooSmall,
    Forbidden,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveRefreshError {
    ForbiddenAdmissionClass(RefreshAdmissionClass),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveExpectedRejectionError {
    UnexpectedRefreshAdmission {
        admission_class: RefreshAdmissionClass,
        admission_status: crate::live_performance::RefreshAdmissionStatus,
    },
    UnexpectedCoalescingAdmission {
        decision: CoalescingDecision,
    },
    UnexpectedProgressAdvance {
        ordinal: u64,
        replay_digest: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LivePromotionDescriptor {
    family: LiveQueryFamily,
    query_digest: ValidatedQueryDigest,
    plan_digest: PlanDigest,
    collection_digest: Option<CollectionPlanDigest>,
    relevance_contract: QueryRelevanceContract,
    refresh_admission_matrix: RefreshAdmissionMatrix,
    incremental_eligibility: IncrementalPatchEligibility,
    performance_report: LivePerformanceReport,
}

impl LivePromotionDescriptor {
    pub fn family(&self) -> &LiveQueryFamily {
        &self.family
    }

    pub fn query_digest(&self) -> &ValidatedQueryDigest {
        &self.query_digest
    }

    pub fn plan_digest(&self) -> &PlanDigest {
        &self.plan_digest
    }

    pub fn collection_digest(&self) -> Option<&CollectionPlanDigest> {
        self.collection_digest.as_ref()
    }

    pub fn relevance_contract(&self) -> &QueryRelevanceContract {
        &self.relevance_contract
    }

    pub fn refresh_admission_matrix(&self) -> &RefreshAdmissionMatrix {
        &self.refresh_admission_matrix
    }

    pub fn incremental_eligibility(&self) -> &IncrementalPatchEligibility {
        &self.incremental_eligibility
    }

    pub fn performance_report(&self) -> &LivePerformanceReport {
        &self.performance_report
    }

    pub(crate) fn for_plan(
        bundle: &ValidatedQueryBundle,
        plan_digest: PlanDigest,
        collection: Option<&CollectionPlanBundle>,
    ) -> Self {
        match collection {
            None => Self {
                family: LiveQueryFamily::Detail,
                query_digest: bundle.query().digest().clone(),
                plan_digest,
                collection_digest: None,
                relevance_contract: QueryRelevanceContract::for_detail(bundle),
                refresh_admission_matrix: RefreshAdmissionMatrix::detail_family(),
                incremental_eligibility: IncrementalPatchEligibility::incremental(
                    "detail live family is admitted for milestone 5",
                ),
                performance_report: LivePerformanceReport::verified_detail_family(),
            },
            Some(collection) if collection.traversal_bound().edge_classes().is_empty() => Self {
                family: LiveQueryFamily::OrderedCollection,
                query_digest: bundle.query().digest().clone(),
                plan_digest,
                collection_digest: Some(collection.digest().clone()),
                relevance_contract: QueryRelevanceContract::for_ordered_collection(
                    bundle, collection,
                ),
                refresh_admission_matrix: RefreshAdmissionMatrix::ordered_collection_family(),
                incremental_eligibility: IncrementalPatchEligibility::incremental(
                    "ordered collection live family is admitted for milestone 5",
                ),
                performance_report: LivePerformanceReport::verified_ordered_collection_family(),
            },
            Some(collection) => Self {
                family: LiveQueryFamily::BoundedMaterialization,
                query_digest: bundle.query().digest().clone(),
                plan_digest,
                collection_digest: Some(collection.digest().clone()),
                relevance_contract: QueryRelevanceContract::for_bounded_materialization(
                    bundle, collection,
                ),
                refresh_admission_matrix: RefreshAdmissionMatrix::bounded_materialization_family(),
                incremental_eligibility: IncrementalPatchEligibility::incremental(
                    "bounded materialization live family is admitted for milestone 5",
                ),
                performance_report: LivePerformanceReport::debt_bounded_materialization_family(),
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LiveSubscriptionDigest(String);

impl LiveSubscriptionDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LiveChangeSequenceId(String);

impl LiveChangeSequenceId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_subscription_digest(digest: &LiveSubscriptionDigest) -> Self {
        Self(hash_parts(&[format!(
            "live_change_sequence:{}",
            digest.as_str()
        )]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LiveChangeOrdinal(u64);

impl LiveChangeOrdinal {
    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn from_value(value: u64) -> Self {
        Self(value)
    }

    fn zero() -> Self {
        Self(0)
    }

    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LiveReplayDigest(String);

impl LiveReplayDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveStartBasis {
    basis: ResolvedSnapshotBasis,
}

impl LiveStartBasis {
    pub fn basis(&self) -> &ResolvedSnapshotBasis {
        &self.basis
    }

    fn new(basis: ResolvedSnapshotBasis) -> Self {
        Self { basis }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveProgressBasis {
    current_basis: ResolvedSnapshotBasis,
    change_sequence_id: LiveChangeSequenceId,
    last_ordinal: LiveChangeOrdinal,
    replay_digest: LiveReplayDigest,
}

impl LiveProgressBasis {
    pub fn current_basis(&self) -> &ResolvedSnapshotBasis {
        &self.current_basis
    }

    pub fn change_sequence_id(&self) -> &LiveChangeSequenceId {
        &self.change_sequence_id
    }

    pub fn last_ordinal(&self) -> &LiveChangeOrdinal {
        &self.last_ordinal
    }

    pub fn replay_digest(&self) -> &LiveReplayDigest {
        &self.replay_digest
    }

    fn initial(subscription_digest: &LiveSubscriptionDigest, start_basis: &LiveStartBasis) -> Self {
        let change_sequence_id =
            LiveChangeSequenceId::from_subscription_digest(subscription_digest);
        let last_ordinal = LiveChangeOrdinal::zero();
        let replay_digest = LiveReplayDigest::from_parts(&[
            format!("subscription:{}", subscription_digest.as_str()),
            format!("basis:{}", start_basis.basis().proof().digest().as_str()),
            format!("change_sequence:{}", change_sequence_id.as_str()),
            format!("ordinal:{}", last_ordinal.value()),
        ]);
        Self {
            current_basis: start_basis.basis().clone(),
            change_sequence_id,
            last_ordinal,
            replay_digest,
        }
    }

    pub fn advance(
        &self,
        change_sequence_id: &LiveChangeSequenceId,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Result<Self, LiveProgressError> {
        if self.change_sequence_id != *change_sequence_id {
            return Err(LiveProgressError::ChangeSequenceMismatch);
        }

        let expected = self.last_ordinal.next();
        if next_ordinal.value() > expected.value() {
            return Err(LiveProgressError::ChangeSequenceGap {
                expected: expected.value(),
                received: next_ordinal.value(),
            });
        }
        if next_ordinal != expected {
            return Err(LiveProgressError::NonMonotonicOrdinal {
                expected: expected.value(),
                received: next_ordinal.value(),
            });
        }

        let replay_digest = LiveReplayDigest::from_parts(&[
            format!("basis:{}", next_basis.proof().digest().as_str()),
            format!("change_sequence:{}", change_sequence_id.as_str()),
            format!("ordinal:{}", next_ordinal.value()),
        ]);

        Ok(Self {
            current_basis: next_basis,
            change_sequence_id: change_sequence_id.clone(),
            last_ordinal: next_ordinal,
            replay_digest,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveQueryPlan {
    descriptor: LivePromotionDescriptor,
    start_basis: LiveStartBasis,
    progress_basis: LiveProgressBasis,
    subscription_digest: LiveSubscriptionDigest,
    baseline_result_digest: String,
}

impl LiveQueryPlan {
    pub fn descriptor(&self) -> &LivePromotionDescriptor {
        &self.descriptor
    }

    pub fn start_basis(&self) -> &LiveStartBasis {
        &self.start_basis
    }

    pub fn progress_basis(&self) -> &LiveProgressBasis {
        &self.progress_basis
    }

    pub fn subscription_digest(&self) -> &LiveSubscriptionDigest {
        &self.subscription_digest
    }

    pub fn baseline_result_digest(&self) -> &str {
        &self.baseline_result_digest
    }

    pub fn performance_status(&self) -> &str {
        self.descriptor.performance_report().performance_status()
    }

    pub fn advance_progress(
        &self,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Result<Self, LiveProgressError> {
        let progress_basis = self.progress_basis.advance(
            self.progress_basis.change_sequence_id(),
            next_ordinal,
            next_basis,
        )?;
        Ok(Self {
            descriptor: self.descriptor.clone(),
            start_basis: self.start_basis.clone(),
            progress_basis,
            subscription_digest: self.subscription_digest.clone(),
            baseline_result_digest: self.baseline_result_digest.clone(),
        })
    }

    pub fn evaluate_delivery_width(&self, measured_width: usize) -> PatchWidthAssessment {
        let budget_limit = self.descriptor.performance_report().width_budget().limit();
        if measured_width <= budget_limit {
            return PatchWidthAssessment {
                measured_width,
                budget_limit,
                resolution: PatchWidthResolution::Deliver,
            };
        }

        let resolution = match self.descriptor.performance_report().width_policy() {
            crate::live_performance::PatchWidthPolicy::DeliverWithinBudget => {
                PatchWidthResolution::Reject
            }
            crate::live_performance::PatchWidthPolicy::CoalesceWithinAdmittedClass => {
                PatchWidthResolution::Coalesce
            }
            crate::live_performance::PatchWidthPolicy::RefreshWithinAdmissionMatrix => {
                if self
                    .descriptor
                    .refresh_admission_matrix()
                    .admits(&RefreshAdmissionClass::WidthOverflow)
                {
                    PatchWidthResolution::Refresh(RefreshFallback {
                        admission_class: RefreshAdmissionClass::WidthOverflow,
                        cost_class: self
                            .descriptor
                            .performance_report()
                            .refresh_cost_class()
                            .clone(),
                        admission_status: self
                            .descriptor
                            .performance_report()
                            .refresh_admission_status()
                            .clone(),
                    })
                } else {
                    PatchWidthResolution::Reject
                }
            }
            crate::live_performance::PatchWidthPolicy::RejectOverflow => {
                PatchWidthResolution::Reject
            }
        };

        PatchWidthAssessment {
            measured_width,
            budget_limit,
            resolution,
        }
    }

    pub fn request_coalesced_delivery(
        &self,
        bundle_count: usize,
    ) -> Result<CoalescingDecision, LiveCoalescingError> {
        if bundle_count == 0 {
            return Err(LiveCoalescingError::BundleCountTooSmall);
        }
        if bundle_count == 1 {
            return Ok(CoalescingDecision::NotNeeded);
        }

        match self.descriptor.performance_report().coalescing_admission() {
            crate::live_performance::CoalescingAdmissionClass::BasisStableEquivalent => {
                Ok(CoalescingDecision::Admitted { bundle_count })
            }
            crate::live_performance::CoalescingAdmissionClass::Forbidden => {
                Err(LiveCoalescingError::Forbidden)
            }
        }
    }

    pub fn request_refresh_fallback(
        &self,
        admission_class: RefreshAdmissionClass,
    ) -> Result<RefreshFallback, LiveRefreshError> {
        if self
            .descriptor
            .refresh_admission_matrix()
            .admits(&admission_class)
            && self
                .descriptor
                .performance_report()
                .refresh_admission_status()
                != &crate::live_performance::RefreshAdmissionStatus::Forbidden
        {
            Ok(RefreshFallback {
                admission_class,
                cost_class: self
                    .descriptor
                    .performance_report()
                    .refresh_cost_class()
                    .clone(),
                admission_status: self
                    .descriptor
                    .performance_report()
                    .refresh_admission_status()
                    .clone(),
            })
        } else {
            Err(LiveRefreshError::ForbiddenAdmissionClass(admission_class))
        }
    }

    pub fn classify_change(&self, change: &BridgeChangeSummary) -> ChangeRelevance {
        self.descriptor.relevance_contract().classify_change(change)
    }

    pub fn detail_live_outcome(
        &self,
        change: &BridgeChangeSummary,
    ) -> Result<DetailLiveOutcome, LiveDetailPatchError> {
        if self.descriptor.family() != &LiveQueryFamily::Detail {
            return Err(LiveDetailPatchError::UnsupportedFamily);
        }

        match self.classify_change(change) {
            ChangeRelevance::Irrelevant(reason) => Ok(DetailLiveOutcome::Suppressed(
                SuppressionReason::IrrelevantChange(reason),
            )),
            ChangeRelevance::Relevant(RelevantChangeClass::DetailProjectionChange) => {
                let field_deltas: Vec<ProjectionFieldDelta> = change
                    .field_deltas()
                    .iter()
                    .filter(|delta| {
                        self.descriptor
                            .relevance_contract()
                            .projected_fields()
                            .iter()
                            .any(|field| field.matches(delta))
                    })
                    .map(|delta| ProjectionFieldDelta {
                        field: QueryFieldKey::new(delta.aspect(), delta.field()),
                        old_value: delta.old_value().map(ToOwned::to_owned),
                        new_value: delta.new_value().map(ToOwned::to_owned),
                    })
                    .collect();

                if field_deltas.is_empty() {
                    return Err(LiveDetailPatchError::RelevantChangeWithoutProjectedDelta);
                }

                let width = self.evaluate_delivery_width(field_deltas.len());
                match width.resolution() {
                    PatchWidthResolution::Deliver => {}
                    PatchWidthResolution::Reject => {
                        return Err(LiveDetailPatchError::WidthBudgetExceeded {
                            limit: width.budget_limit(),
                            actual: width.measured_width(),
                        });
                    }
                    PatchWidthResolution::Coalesce => {
                        return Err(LiveDetailPatchError::CoalescingRequired);
                    }
                    PatchWidthResolution::Refresh(fallback) => {
                        return Ok(DetailLiveOutcome::Refresh(fallback.clone()));
                    }
                }

                let mut digest_parts = Vec::new();
                digest_parts.extend(field_deltas.iter().map(|delta| {
                    format!(
                        "field_delta:{}:{}:{:?}:{:?}",
                        delta.field.aspect(),
                        delta.field.field(),
                        delta.old_value,
                        delta.new_value
                    )
                }));

                Ok(DetailLiveOutcome::Patch(DetailPatch {
                    digest: self.patch_digest(&digest_parts),
                    field_deltas,
                }))
            }
            ChangeRelevance::Relevant(other) => {
                Err(LiveDetailPatchError::UnsupportedRelevantClass(other))
            }
        }
    }

    pub fn ordered_collection_live_outcome(
        &self,
        change: &BridgeChangeSummary,
    ) -> Result<OrderedCollectionLiveOutcome, LiveCollectionPatchError> {
        if self.descriptor.family() != &LiveQueryFamily::OrderedCollection {
            return Err(LiveCollectionPatchError::UnsupportedFamily);
        }

        match self.classify_change(change) {
            ChangeRelevance::Irrelevant(reason) => Ok(OrderedCollectionLiveOutcome::Suppressed(
                SuppressionReason::IrrelevantChange(reason),
            )),
            ChangeRelevance::Relevant(RelevantChangeClass::OrderedCollectionMembershipChange) => {
                let transition = change
                    .membership_transition()
                    .ok_or(LiveCollectionPatchError::MissingMembershipTransition)?;
                let membership_change = CollectionMembershipChange::try_from_transition(transition)
                    .ok_or(LiveCollectionPatchError::NoMembershipDelta)?;
                let patch = OrderedCollectionPatch {
                    digest: self.patch_digest(&[
                        "kind:collection_membership".to_string(),
                        format!("membership:{}", membership_change.as_str()),
                    ]),
                    kind: OrderedCollectionPatchKind::Membership(membership_change),
                    projected_field_deltas: self.projected_field_deltas(change),
                };
                self.resolve_ordered_collection_outcome(patch)
            }
            ChangeRelevance::Relevant(RelevantChangeClass::OrderedCollectionOrderingChange) => {
                let ordering_field_deltas = self.ordering_field_deltas(change);
                if ordering_field_deltas.is_empty() {
                    return Err(LiveCollectionPatchError::MissingOrderingDelta);
                }

                let patch = OrderedCollectionPatch {
                    digest: self.patch_digest(&[
                        "kind:collection_reordered".to_string(),
                        format!("ordering_fields:{}", ordering_field_deltas.len()),
                    ]),
                    kind: OrderedCollectionPatchKind::Reordered(CollectionOrderingChange {
                        ordering_field_deltas,
                    }),
                    projected_field_deltas: self.projected_field_deltas(change),
                };
                self.resolve_ordered_collection_outcome(patch)
            }
            ChangeRelevance::Relevant(RelevantChangeClass::DetailProjectionChange) => {
                let projected_field_deltas = self.projected_field_deltas(change);
                if projected_field_deltas.is_empty() {
                    return Err(LiveCollectionPatchError::MissingProjectedDelta);
                }

                let patch = OrderedCollectionPatch {
                    digest: self.patch_digest(&[
                        "kind:collection_row_update".to_string(),
                        format!("projected_fields:{}", projected_field_deltas.len()),
                    ]),
                    kind: OrderedCollectionPatchKind::RowUpdated,
                    projected_field_deltas,
                };
                self.resolve_ordered_collection_outcome(patch)
            }
            ChangeRelevance::Relevant(other) => {
                Err(LiveCollectionPatchError::UnsupportedRelevantClass(other))
            }
        }
    }

    pub fn bounded_materialization_live_outcome(
        &self,
        change: &BridgeChangeSummary,
    ) -> Result<BoundedMaterializationLiveOutcome, LiveBoundedMaterializationPatchError> {
        if self.descriptor.family() != &LiveQueryFamily::BoundedMaterialization {
            return Err(LiveBoundedMaterializationPatchError::UnsupportedFamily);
        }

        match self.classify_change(change) {
            ChangeRelevance::Irrelevant(reason) => {
                Ok(BoundedMaterializationLiveOutcome::Suppressed(
                    SuppressionReason::IrrelevantChange(reason),
                ))
            }
            ChangeRelevance::Relevant(RelevantChangeClass::BoundedMaterializationScopeChange) => {
                let transition = change.materialization_scope_transition().ok_or(
                    LiveBoundedMaterializationPatchError::MissingMaterializationScopeTransition,
                )?;
                let scope_change = MaterializationScopeChange::try_from_transition(transition)
                    .ok_or(LiveBoundedMaterializationPatchError::NoMaterializationScopeDelta)?;
                let relation_deltas = self.relation_deltas(change);
                let patch = BoundedMaterializationPatch {
                    digest: self.patch_digest(&[
                        "kind:materialization_scope".to_string(),
                        format!("scope:{}", scope_change.as_str()),
                        format!("relations:{}", relation_deltas.len()),
                    ]),
                    kind: BoundedMaterializationPatchKind::Scope(scope_change),
                    projected_field_deltas: self.projected_field_deltas(change),
                    relation_deltas,
                };
                self.resolve_bounded_materialization_outcome(patch)
            }
            ChangeRelevance::Relevant(RelevantChangeClass::OrderedCollectionMembershipChange) => {
                let transition = change
                    .membership_transition()
                    .ok_or(LiveBoundedMaterializationPatchError::MissingMembershipTransition)?;
                let membership_change = CollectionMembershipChange::try_from_transition(transition)
                    .ok_or(LiveBoundedMaterializationPatchError::NoMembershipDelta)?;
                let patch = BoundedMaterializationPatch {
                    digest: self.patch_digest(&[
                        "kind:bounded_collection_membership".to_string(),
                        format!("membership:{}", membership_change.as_str()),
                    ]),
                    kind: BoundedMaterializationPatchKind::Membership(membership_change),
                    projected_field_deltas: self.projected_field_deltas(change),
                    relation_deltas: self.relation_deltas(change),
                };
                self.resolve_bounded_materialization_outcome(patch)
            }
            ChangeRelevance::Relevant(RelevantChangeClass::OrderedCollectionOrderingChange) => {
                let ordering_field_deltas = self.ordering_field_deltas(change);
                if ordering_field_deltas.is_empty() {
                    return Err(LiveBoundedMaterializationPatchError::MissingOrderingDelta);
                }
                let patch = BoundedMaterializationPatch {
                    digest: self.patch_digest(&[
                        "kind:bounded_collection_reordered".to_string(),
                        format!("ordering_fields:{}", ordering_field_deltas.len()),
                    ]),
                    kind: BoundedMaterializationPatchKind::Reordered(CollectionOrderingChange {
                        ordering_field_deltas,
                    }),
                    projected_field_deltas: self.projected_field_deltas(change),
                    relation_deltas: self.relation_deltas(change),
                };
                self.resolve_bounded_materialization_outcome(patch)
            }
            ChangeRelevance::Relevant(RelevantChangeClass::DetailProjectionChange) => {
                let projected_field_deltas = self.projected_field_deltas(change);
                if projected_field_deltas.is_empty() {
                    return Err(LiveBoundedMaterializationPatchError::MissingProjectedDelta);
                }
                let patch = BoundedMaterializationPatch {
                    digest: self.patch_digest(&[
                        "kind:bounded_row_update".to_string(),
                        format!("projected_fields:{}", projected_field_deltas.len()),
                    ]),
                    kind: BoundedMaterializationPatchKind::RowUpdated,
                    projected_field_deltas,
                    relation_deltas: self.relation_deltas(change),
                };
                self.resolve_bounded_materialization_outcome(patch)
            }
        }
    }

    fn resolve_ordered_collection_outcome(
        &self,
        patch: OrderedCollectionPatch,
    ) -> Result<OrderedCollectionLiveOutcome, LiveCollectionPatchError> {
        let width = self.evaluate_delivery_width(self.measure_ordered_collection_width(&patch));
        match width.resolution() {
            PatchWidthResolution::Deliver => Ok(OrderedCollectionLiveOutcome::Patch(patch)),
            PatchWidthResolution::Coalesce => Err(LiveCollectionPatchError::CoalescingRequired {
                limit: width.budget_limit(),
                actual: width.measured_width(),
            }),
            PatchWidthResolution::Refresh(fallback) => {
                Ok(OrderedCollectionLiveOutcome::Refresh(fallback.clone()))
            }
            PatchWidthResolution::Reject => Err(LiveCollectionPatchError::WidthBudgetExceeded {
                limit: width.budget_limit(),
                actual: width.measured_width(),
            }),
        }
    }

    fn resolve_bounded_materialization_outcome(
        &self,
        patch: BoundedMaterializationPatch,
    ) -> Result<BoundedMaterializationLiveOutcome, LiveBoundedMaterializationPatchError> {
        let width =
            self.evaluate_delivery_width(self.measure_bounded_materialization_width(&patch));
        match width.resolution() {
            PatchWidthResolution::Deliver => Ok(BoundedMaterializationLiveOutcome::Patch(patch)),
            PatchWidthResolution::Refresh(fallback) => {
                Ok(BoundedMaterializationLiveOutcome::Refresh(fallback.clone()))
            }
            PatchWidthResolution::Coalesce => {
                Err(LiveBoundedMaterializationPatchError::CoalescingRequired {
                    limit: width.budget_limit(),
                    actual: width.measured_width(),
                })
            }
            PatchWidthResolution::Reject => {
                Err(LiveBoundedMaterializationPatchError::WidthBudgetExceeded {
                    limit: width.budget_limit(),
                    actual: width.measured_width(),
                })
            }
        }
    }

    fn projected_field_deltas(&self, change: &BridgeChangeSummary) -> Vec<ProjectionFieldDelta> {
        change
            .field_deltas()
            .iter()
            .filter(|delta| {
                self.descriptor
                    .relevance_contract()
                    .projected_fields()
                    .iter()
                    .any(|field| field.matches(delta))
            })
            .map(|delta| ProjectionFieldDelta {
                field: QueryFieldKey::new(delta.aspect(), delta.field()),
                old_value: delta.old_value().map(ToOwned::to_owned),
                new_value: delta.new_value().map(ToOwned::to_owned),
            })
            .collect()
    }

    fn ordering_field_deltas(&self, change: &BridgeChangeSummary) -> Vec<OrderingFieldDelta> {
        change
            .field_deltas()
            .iter()
            .filter(|delta| {
                self.descriptor
                    .relevance_contract()
                    .ordering_fields()
                    .iter()
                    .any(|field| field.matches(delta))
            })
            .map(|delta| OrderingFieldDelta {
                field: QueryFieldKey::new(delta.aspect(), delta.field()),
                old_value: delta.old_value().map(ToOwned::to_owned),
                new_value: delta.new_value().map(ToOwned::to_owned),
            })
            .collect()
    }

    fn relation_deltas(&self, change: &BridgeChangeSummary) -> Vec<String> {
        change
            .relation_deltas()
            .iter()
            .map(|delta| delta.relation().to_string())
            .collect()
    }

    fn patch_digest(&self, extra_parts: &[String]) -> LivePatchDigest {
        let mut digest_parts = vec![
            format!("query:{}", self.descriptor.query_digest().as_str()),
            format!("family:{}", self.descriptor.family().as_str()),
        ];
        digest_parts.extend(extra_parts.iter().cloned());
        LivePatchDigest::from_parts(&digest_parts)
    }

    fn measure_ordered_collection_width(&self, patch: &OrderedCollectionPatch) -> usize {
        patch.projected_field_deltas().len() + 1
    }

    fn measure_bounded_materialization_width(&self, patch: &BoundedMaterializationPatch) -> usize {
        patch.projected_field_deltas().len() + patch.relation_deltas().len() + 1
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LocalityAdmissionClass {
    DetailRegion,
    DetailPartition,
    OrderedCollectionPartition,
    BoundedMaterializationRegion,
}

impl LocalityAdmissionClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailRegion => "detail_region",
            Self::DetailPartition => "detail_partition",
            Self::OrderedCollectionPartition => "ordered_collection_partition",
            Self::BoundedMaterializationRegion => "bounded_materialization_region",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LocalityCostPosture {
    SingleSliceNarrowing,
    PartitionScopedMembershipNarrowing,
    BoundedTraversalRegionNarrowing,
}

impl LocalityCostPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SingleSliceNarrowing => "single_slice_narrowing",
            Self::PartitionScopedMembershipNarrowing => "partition_scoped_membership_narrowing",
            Self::BoundedTraversalRegionNarrowing => "bounded_traversal_region_narrowing",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LocalityBreadthBudget {
    limit: usize,
}

impl LocalityBreadthBudget {
    pub fn limit(&self) -> usize {
        self.limit
    }

    fn single_surface() -> Self {
        Self { limit: 1 }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LocalityWideningBudget {
    limit: usize,
}

impl LocalityWideningBudget {
    pub fn limit(&self) -> usize {
        self.limit
    }

    fn deny_all() -> Self {
        Self { limit: 0 }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StreamLoweringCostPosture {
    SingleDetailCurrentStateMember,
    CdcPatchWithProjectedDeltas,
    BoundedMaterializationDeferred,
}

impl StreamLoweringCostPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SingleDetailCurrentStateMember => "single_detail_current_state_member",
            Self::CdcPatchWithProjectedDeltas => "cdc_patch_with_projected_deltas",
            Self::BoundedMaterializationDeferred => "bounded_materialization_deferred",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StreamMemberWidthBudget {
    limit: usize,
}

impl StreamMemberWidthBudget {
    pub fn limit(&self) -> usize {
        self.limit
    }

    fn single_member() -> Self {
        Self { limit: 1 }
    }

    fn cdc_projected_patch() -> Self {
        Self { limit: 2 }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionScopedLivePlan {
    live: LiveQueryPlan,
    locality: LocalityPredicateContract,
    admission_class: LocalityAdmissionClass,
    locality_subscription_digest: String,
    locality_cost_posture: LocalityCostPosture,
    locality_breadth_budget: LocalityBreadthBudget,
    locality_widening_budget: LocalityWideningBudget,
    stream_lowering_cost_posture: StreamLoweringCostPosture,
    stream_member_width_budget: StreamMemberWidthBudget,
}

impl RegionScopedLivePlan {
    pub fn live(&self) -> &LiveQueryPlan {
        &self.live
    }

    pub fn locality(&self) -> &LocalityPredicateContract {
        &self.locality
    }

    pub fn admission_class(&self) -> &LocalityAdmissionClass {
        &self.admission_class
    }

    pub fn locality_subscription_digest(&self) -> &str {
        &self.locality_subscription_digest
    }

    pub fn locality_cost_posture(&self) -> &LocalityCostPosture {
        &self.locality_cost_posture
    }

    pub fn locality_breadth_budget(&self) -> &LocalityBreadthBudget {
        &self.locality_breadth_budget
    }

    pub fn locality_widening_budget(&self) -> &LocalityWideningBudget {
        &self.locality_widening_budget
    }

    pub fn stream_lowering_cost_posture(&self) -> &StreamLoweringCostPosture {
        &self.stream_lowering_cost_posture
    }

    pub fn stream_member_width_budget(&self) -> &StreamMemberWidthBudget {
        &self.stream_member_width_budget
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalityMatchKind {
    InRegionRegionScope,
    InRegionPartitionScope,
    OffRegionSuppressed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionScopedExecutionReport {
    query_digest: String,
    locality_digest: String,
    locality_outcome: String,
    result_digest: String,
    delivery_digest: String,
    replay_digest: String,
}

impl RegionScopedExecutionReport {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }

    pub fn locality_outcome(&self) -> &str {
        &self.locality_outcome
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionScopedLiveExecutionEnvelope {
    report: RegionScopedExecutionReport,
    patch_envelope: LivePatchEnvelope,
    replay_bundle: LiveReplayBundle,
    counters: LivePolicyCounters,
}

impl RegionScopedLiveExecutionEnvelope {
    pub fn report(&self) -> &RegionScopedExecutionReport {
        &self.report
    }

    pub fn patch_envelope(&self) -> &LivePatchEnvelope {
        &self.patch_envelope
    }

    pub fn replay_bundle(&self) -> &LiveReplayBundle {
        &self.replay_bundle
    }

    pub fn counters(&self) -> &LivePolicyCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StreamConsumerShape {
    DetailCurrentState,
    CdcCollectionPatch,
}

impl StreamConsumerShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailCurrentState => "detail_current_state",
            Self::CdcCollectionPatch => "cdc_collection_patch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamLoweredDeliveryContract {
    query_digest: String,
    locality_digest: String,
    delivery_digest: String,
    stream_contract_digest: String,
    consumer_shape: StreamConsumerShape,
    member_count: usize,
    delivery_width: usize,
    cost_posture: StreamLoweringCostPosture,
}

impl StreamLoweredDeliveryContract {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn stream_contract_digest(&self) -> &str {
        &self.stream_contract_digest
    }

    pub fn consumer_shape(&self) -> &StreamConsumerShape {
        &self.consumer_shape
    }

    pub fn member_count(&self) -> usize {
        self.member_count
    }

    pub fn delivery_width(&self) -> usize {
        self.delivery_width
    }

    pub fn cost_posture(&self) -> &StreamLoweringCostPosture {
        &self.cost_posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegionScopedLiveError {
    UnsupportedLocalityFamily,
    UnsupportedLocalityPredicate,
    LocalityBreadthBudgetExceeded {
        limit: usize,
        actual: usize,
    },
    WideningDenied {
        expected: String,
        received: Vec<String>,
    },
    StreamMemberWidthBudgetExceeded {
        limit: usize,
        actual: usize,
    },
    BridgeSliceIncompatibility,
    UnsupportedStreamConsumerShape,
    LiveExecution(LiveExecutionError),
}

impl From<LiveExecutionError> for RegionScopedLiveError {
    fn from(value: LiveExecutionError) -> Self {
        Self::LiveExecution(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LivePromotionError {
    UnsupportedPreflightRoute,
    UnsupportedLiveCollectionFamily,
    PlanDescriptorMismatch,
    BasisPreflight(BasisPreflightError),
    Execution(ExecutionError),
}

impl From<BasisPreflightError> for LivePromotionError {
    fn from(value: BasisPreflightError) -> Self {
        Self::BasisPreflight(value)
    }
}

impl From<ExecutionError> for LivePromotionError {
    fn from(value: ExecutionError) -> Self {
        Self::Execution(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveProgressError {
    ChangeSequenceMismatch,
    ChangeSequenceGap { expected: u64, received: u64 },
    NonMonotonicOrdinal { expected: u64, received: u64 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SuppressionReason {
    IrrelevantChange(IrrelevantChangeClass),
    OffRegionChange {
        scope_kind: LocalityScopeKind,
        scope: String,
        locality_digest: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SuppressionDecision {
    Deliver,
    Suppress(SuppressionReason),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LivePolicyCounters {
    live_invalidation_event_count: usize,
    live_relevance_match_count: usize,
    live_irrelevant_suppression_count: usize,
    live_threshold_suppression_count: usize,
    live_patch_count: usize,
    live_patch_delivery_count: usize,
    live_suppressed_update_count: usize,
    live_patch_field_delta_count: usize,
    live_collection_membership_change_count: usize,
    live_collection_reorder_count: usize,
    live_materialization_patch_count: usize,
    live_refresh_fallback_count: usize,
    live_refresh_denial_count: usize,
    live_replay_change_count: usize,
    live_change_sequence_gap_count: usize,
    live_coalesced_change_bundle_count: usize,
    live_coalescing_denial_count: usize,
    live_delivery_width: usize,
    live_patch_width_overflow_count: usize,
    live_refresh_cost_class_count: usize,
    live_work_avoided_by_irrelevance_count: usize,
    live_work_avoided_by_stable_ordering_count: usize,
    live_work_avoided_by_scope_proof_count: usize,
    live_executor_rediscovery_count: usize,
    live_progress_advance_count: usize,
    live_non_monotonic_sequence_rejection_count: usize,
    live_invalid_promotion_rejection_count: usize,
    live_unsupported_patch_family_rejection_count: usize,
    locality_region_match_count: usize,
    locality_partition_match_count: usize,
    locality_off_region_suppression_count: usize,
    locality_breadth_budget_cross_count: usize,
    locality_widening_budget_cross_count: usize,
    locality_widening_denial_count: usize,
    locality_bridge_slice_incompatibility_count: usize,
    stream_contract_admission_count: usize,
    stream_contract_denial_count: usize,
    stream_lowered_delivery_count: usize,
    stream_lowered_delivery_member_count: usize,
    stream_lowered_delivery_width: usize,
    stream_member_width_budget_cross_count: usize,
    locality_work_avoided_by_region_narrowing_count: usize,
    locality_work_avoided_vs_broad_control_count: usize,
    locality_executor_rediscovery_count: usize,
    locality_unsupported_family_rejection_count: usize,
    locality_unsupported_predicate_rejection_count: usize,
}

impl LivePolicyCounters {
    pub fn live_invalidation_event_count(&self) -> usize {
        self.live_invalidation_event_count
    }

    pub fn live_relevance_match_count(&self) -> usize {
        self.live_relevance_match_count
    }

    pub fn live_irrelevant_suppression_count(&self) -> usize {
        self.live_irrelevant_suppression_count
    }

    pub fn live_threshold_suppression_count(&self) -> usize {
        self.live_threshold_suppression_count
    }

    pub fn live_patch_count(&self) -> usize {
        self.live_patch_count
    }

    pub fn live_patch_delivery_count(&self) -> usize {
        self.live_patch_delivery_count
    }

    pub fn live_suppressed_update_count(&self) -> usize {
        self.live_suppressed_update_count
    }

    pub fn live_patch_field_delta_count(&self) -> usize {
        self.live_patch_field_delta_count
    }

    pub fn live_collection_membership_change_count(&self) -> usize {
        self.live_collection_membership_change_count
    }

    pub fn live_collection_reorder_count(&self) -> usize {
        self.live_collection_reorder_count
    }

    pub fn live_materialization_patch_count(&self) -> usize {
        self.live_materialization_patch_count
    }

    pub fn live_refresh_fallback_count(&self) -> usize {
        self.live_refresh_fallback_count
    }

    pub fn live_refresh_denial_count(&self) -> usize {
        self.live_refresh_denial_count
    }

    pub fn live_replay_change_count(&self) -> usize {
        self.live_replay_change_count
    }

    pub fn live_change_sequence_gap_count(&self) -> usize {
        self.live_change_sequence_gap_count
    }

    pub fn live_coalesced_change_bundle_count(&self) -> usize {
        self.live_coalesced_change_bundle_count
    }

    pub fn live_coalescing_denial_count(&self) -> usize {
        self.live_coalescing_denial_count
    }

    pub fn live_delivery_width(&self) -> usize {
        self.live_delivery_width
    }

    pub fn live_patch_width_overflow_count(&self) -> usize {
        self.live_patch_width_overflow_count
    }

    pub fn live_refresh_cost_class_count(&self) -> usize {
        self.live_refresh_cost_class_count
    }

    pub fn live_work_avoided_by_irrelevance_count(&self) -> usize {
        self.live_work_avoided_by_irrelevance_count
    }

    pub fn live_work_avoided_by_stable_ordering_count(&self) -> usize {
        self.live_work_avoided_by_stable_ordering_count
    }

    pub fn live_work_avoided_by_scope_proof_count(&self) -> usize {
        self.live_work_avoided_by_scope_proof_count
    }

    pub fn live_executor_rediscovery_count(&self) -> usize {
        self.live_executor_rediscovery_count
    }

    pub fn live_progress_advance_count(&self) -> usize {
        self.live_progress_advance_count
    }

    pub fn live_non_monotonic_sequence_rejection_count(&self) -> usize {
        self.live_non_monotonic_sequence_rejection_count
    }

    pub fn live_invalid_promotion_rejection_count(&self) -> usize {
        self.live_invalid_promotion_rejection_count
    }

    pub fn live_unsupported_patch_family_rejection_count(&self) -> usize {
        self.live_unsupported_patch_family_rejection_count
    }

    pub fn locality_region_match_count(&self) -> usize {
        self.locality_region_match_count
    }

    pub fn locality_partition_match_count(&self) -> usize {
        self.locality_partition_match_count
    }

    pub fn locality_off_region_suppression_count(&self) -> usize {
        self.locality_off_region_suppression_count
    }

    pub fn locality_breadth_budget_cross_count(&self) -> usize {
        self.locality_breadth_budget_cross_count
    }

    pub fn locality_widening_budget_cross_count(&self) -> usize {
        self.locality_widening_budget_cross_count
    }

    pub fn locality_widening_denial_count(&self) -> usize {
        self.locality_widening_denial_count
    }

    pub fn locality_bridge_slice_incompatibility_count(&self) -> usize {
        self.locality_bridge_slice_incompatibility_count
    }

    pub fn stream_contract_admission_count(&self) -> usize {
        self.stream_contract_admission_count
    }

    pub fn stream_contract_denial_count(&self) -> usize {
        self.stream_contract_denial_count
    }

    pub fn stream_lowered_delivery_count(&self) -> usize {
        self.stream_lowered_delivery_count
    }

    pub fn stream_lowered_delivery_member_count(&self) -> usize {
        self.stream_lowered_delivery_member_count
    }

    pub fn stream_lowered_delivery_width(&self) -> usize {
        self.stream_lowered_delivery_width
    }

    pub fn stream_member_width_budget_cross_count(&self) -> usize {
        self.stream_member_width_budget_cross_count
    }

    pub fn locality_work_avoided_by_region_narrowing_count(&self) -> usize {
        self.locality_work_avoided_by_region_narrowing_count
    }

    pub fn locality_work_avoided_vs_broad_control_count(&self) -> usize {
        self.locality_work_avoided_vs_broad_control_count
    }

    pub fn locality_executor_rediscovery_count(&self) -> usize {
        self.locality_executor_rediscovery_count
    }

    pub fn locality_unsupported_family_rejection_count(&self) -> usize {
        self.locality_unsupported_family_rejection_count
    }

    pub fn locality_unsupported_predicate_rejection_count(&self) -> usize {
        self.locality_unsupported_predicate_rejection_count
    }

    pub fn has_activity(&self) -> bool {
        self.live_invalidation_event_count > 0
            || self.live_relevance_match_count > 0
            || self.live_irrelevant_suppression_count > 0
            || self.live_threshold_suppression_count > 0
            || self.live_patch_count > 0
            || self.live_patch_delivery_count > 0
            || self.live_suppressed_update_count > 0
            || self.live_patch_field_delta_count > 0
            || self.live_collection_membership_change_count > 0
            || self.live_collection_reorder_count > 0
            || self.live_materialization_patch_count > 0
            || self.live_refresh_fallback_count > 0
            || self.live_refresh_denial_count > 0
            || self.live_replay_change_count > 0
            || self.live_change_sequence_gap_count > 0
            || self.live_coalesced_change_bundle_count > 0
            || self.live_coalescing_denial_count > 0
            || self.live_delivery_width > 0
            || self.live_patch_width_overflow_count > 0
            || self.live_refresh_cost_class_count > 0
            || self.live_work_avoided_by_irrelevance_count > 0
            || self.live_work_avoided_by_stable_ordering_count > 0
            || self.live_work_avoided_by_scope_proof_count > 0
            || self.live_progress_advance_count > 0
            || self.live_non_monotonic_sequence_rejection_count > 0
            || self.live_invalid_promotion_rejection_count > 0
            || self.live_unsupported_patch_family_rejection_count > 0
            || self.locality_region_match_count > 0
            || self.locality_partition_match_count > 0
            || self.locality_off_region_suppression_count > 0
            || self.locality_breadth_budget_cross_count > 0
            || self.locality_widening_budget_cross_count > 0
            || self.locality_widening_denial_count > 0
            || self.locality_bridge_slice_incompatibility_count > 0
            || self.stream_contract_admission_count > 0
            || self.stream_contract_denial_count > 0
            || self.stream_lowered_delivery_count > 0
            || self.stream_lowered_delivery_member_count > 0
            || self.stream_lowered_delivery_width > 0
            || self.stream_member_width_budget_cross_count > 0
            || self.locality_work_avoided_by_region_narrowing_count > 0
            || self.locality_work_avoided_vs_broad_control_count > 0
            || self.locality_executor_rediscovery_count > 0
            || self.locality_unsupported_family_rejection_count > 0
            || self.locality_unsupported_predicate_rejection_count > 0
    }

    pub fn digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!(
                "{label}_invalidation_event_count:{}",
                self.live_invalidation_event_count
            ),
            format!(
                "{label}_relevance_match_count:{}",
                self.live_relevance_match_count
            ),
            format!(
                "{label}_irrelevant_suppression_count:{}",
                self.live_irrelevant_suppression_count
            ),
            format!(
                "{label}_threshold_suppression_count:{}",
                self.live_threshold_suppression_count
            ),
            format!("{label}_patch_count:{}", self.live_patch_count),
            format!(
                "{label}_patch_delivery_count:{}",
                self.live_patch_delivery_count
            ),
            format!(
                "{label}_suppressed_update_count:{}",
                self.live_suppressed_update_count
            ),
            format!(
                "{label}_patch_field_delta_count:{}",
                self.live_patch_field_delta_count
            ),
            format!(
                "{label}_collection_membership_change_count:{}",
                self.live_collection_membership_change_count
            ),
            format!(
                "{label}_collection_reorder_count:{}",
                self.live_collection_reorder_count
            ),
            format!(
                "{label}_materialization_patch_count:{}",
                self.live_materialization_patch_count
            ),
            format!(
                "{label}_refresh_fallback_count:{}",
                self.live_refresh_fallback_count
            ),
            format!(
                "{label}_refresh_denial_count:{}",
                self.live_refresh_denial_count
            ),
            format!(
                "{label}_replay_change_count:{}",
                self.live_replay_change_count
            ),
            format!(
                "{label}_change_sequence_gap_count:{}",
                self.live_change_sequence_gap_count
            ),
            format!(
                "{label}_coalesced_change_bundle_count:{}",
                self.live_coalesced_change_bundle_count
            ),
            format!(
                "{label}_coalescing_denial_count:{}",
                self.live_coalescing_denial_count
            ),
            format!("{label}_delivery_width:{}", self.live_delivery_width),
            format!(
                "{label}_patch_width_overflow_count:{}",
                self.live_patch_width_overflow_count
            ),
            format!(
                "{label}_refresh_cost_class_count:{}",
                self.live_refresh_cost_class_count
            ),
            format!(
                "{label}_work_avoided_by_irrelevance_count:{}",
                self.live_work_avoided_by_irrelevance_count
            ),
            format!(
                "{label}_work_avoided_by_stable_ordering_count:{}",
                self.live_work_avoided_by_stable_ordering_count
            ),
            format!(
                "{label}_work_avoided_by_scope_proof_count:{}",
                self.live_work_avoided_by_scope_proof_count
            ),
            format!(
                "{label}_executor_rediscovery_count:{}",
                self.live_executor_rediscovery_count
            ),
            format!(
                "{label}_progress_advance_count:{}",
                self.live_progress_advance_count
            ),
            format!(
                "{label}_non_monotonic_sequence_rejection_count:{}",
                self.live_non_monotonic_sequence_rejection_count
            ),
            format!(
                "{label}_invalid_promotion_rejection_count:{}",
                self.live_invalid_promotion_rejection_count
            ),
            format!(
                "{label}_unsupported_patch_family_rejection_count:{}",
                self.live_unsupported_patch_family_rejection_count
            ),
            format!(
                "{label}_locality_region_match_count:{}",
                self.locality_region_match_count
            ),
            format!(
                "{label}_locality_partition_match_count:{}",
                self.locality_partition_match_count
            ),
            format!(
                "{label}_locality_off_region_suppression_count:{}",
                self.locality_off_region_suppression_count
            ),
            format!(
                "{label}_locality_breadth_budget_cross_count:{}",
                self.locality_breadth_budget_cross_count
            ),
            format!(
                "{label}_locality_widening_budget_cross_count:{}",
                self.locality_widening_budget_cross_count
            ),
            format!(
                "{label}_locality_widening_denial_count:{}",
                self.locality_widening_denial_count
            ),
            format!(
                "{label}_locality_bridge_slice_incompatibility_count:{}",
                self.locality_bridge_slice_incompatibility_count
            ),
            format!(
                "{label}_stream_contract_admission_count:{}",
                self.stream_contract_admission_count
            ),
            format!(
                "{label}_stream_contract_denial_count:{}",
                self.stream_contract_denial_count
            ),
            format!(
                "{label}_stream_lowered_delivery_count:{}",
                self.stream_lowered_delivery_count
            ),
            format!(
                "{label}_stream_lowered_delivery_member_count:{}",
                self.stream_lowered_delivery_member_count
            ),
            format!(
                "{label}_stream_lowered_delivery_width:{}",
                self.stream_lowered_delivery_width
            ),
            format!(
                "{label}_stream_member_width_budget_cross_count:{}",
                self.stream_member_width_budget_cross_count
            ),
            format!(
                "{label}_locality_work_avoided_by_region_narrowing_count:{}",
                self.locality_work_avoided_by_region_narrowing_count
            ),
            format!(
                "{label}_locality_work_avoided_vs_broad_control_count:{}",
                self.locality_work_avoided_vs_broad_control_count
            ),
            format!(
                "{label}_locality_executor_rediscovery_count:{}",
                self.locality_executor_rediscovery_count
            ),
            format!(
                "{label}_locality_unsupported_family_rejection_count:{}",
                self.locality_unsupported_family_rejection_count
            ),
            format!(
                "{label}_locality_unsupported_predicate_rejection_count:{}",
                self.locality_unsupported_predicate_rejection_count
            ),
        ]
    }

    pub(crate) fn absorb(&mut self, other: &Self) {
        self.live_invalidation_event_count += other.live_invalidation_event_count;
        self.live_relevance_match_count += other.live_relevance_match_count;
        self.live_irrelevant_suppression_count += other.live_irrelevant_suppression_count;
        self.live_threshold_suppression_count += other.live_threshold_suppression_count;
        self.live_patch_count += other.live_patch_count;
        self.live_patch_delivery_count += other.live_patch_delivery_count;
        self.live_suppressed_update_count += other.live_suppressed_update_count;
        self.live_patch_field_delta_count += other.live_patch_field_delta_count;
        self.live_collection_membership_change_count +=
            other.live_collection_membership_change_count;
        self.live_collection_reorder_count += other.live_collection_reorder_count;
        self.live_materialization_patch_count += other.live_materialization_patch_count;
        self.live_refresh_fallback_count += other.live_refresh_fallback_count;
        self.live_refresh_denial_count += other.live_refresh_denial_count;
        self.live_replay_change_count += other.live_replay_change_count;
        self.live_change_sequence_gap_count += other.live_change_sequence_gap_count;
        self.live_coalesced_change_bundle_count += other.live_coalesced_change_bundle_count;
        self.live_coalescing_denial_count += other.live_coalescing_denial_count;
        self.live_delivery_width += other.live_delivery_width;
        self.live_patch_width_overflow_count += other.live_patch_width_overflow_count;
        self.live_refresh_cost_class_count += other.live_refresh_cost_class_count;
        self.live_work_avoided_by_irrelevance_count += other.live_work_avoided_by_irrelevance_count;
        self.live_work_avoided_by_stable_ordering_count +=
            other.live_work_avoided_by_stable_ordering_count;
        self.live_work_avoided_by_scope_proof_count += other.live_work_avoided_by_scope_proof_count;
        self.live_executor_rediscovery_count += other.live_executor_rediscovery_count;
        self.live_progress_advance_count += other.live_progress_advance_count;
        self.live_non_monotonic_sequence_rejection_count +=
            other.live_non_monotonic_sequence_rejection_count;
        self.live_invalid_promotion_rejection_count += other.live_invalid_promotion_rejection_count;
        self.live_unsupported_patch_family_rejection_count +=
            other.live_unsupported_patch_family_rejection_count;
        self.locality_region_match_count += other.locality_region_match_count;
        self.locality_partition_match_count += other.locality_partition_match_count;
        self.locality_off_region_suppression_count += other.locality_off_region_suppression_count;
        self.locality_breadth_budget_cross_count += other.locality_breadth_budget_cross_count;
        self.locality_widening_budget_cross_count += other.locality_widening_budget_cross_count;
        self.locality_widening_denial_count += other.locality_widening_denial_count;
        self.locality_bridge_slice_incompatibility_count +=
            other.locality_bridge_slice_incompatibility_count;
        self.stream_contract_admission_count += other.stream_contract_admission_count;
        self.stream_contract_denial_count += other.stream_contract_denial_count;
        self.stream_lowered_delivery_count += other.stream_lowered_delivery_count;
        self.stream_lowered_delivery_member_count += other.stream_lowered_delivery_member_count;
        self.stream_lowered_delivery_width += other.stream_lowered_delivery_width;
        self.stream_member_width_budget_cross_count += other.stream_member_width_budget_cross_count;
        self.locality_work_avoided_by_region_narrowing_count +=
            other.locality_work_avoided_by_region_narrowing_count;
        self.locality_work_avoided_vs_broad_control_count +=
            other.locality_work_avoided_vs_broad_control_count;
        self.locality_executor_rediscovery_count += other.locality_executor_rediscovery_count;
        self.locality_unsupported_family_rejection_count +=
            other.locality_unsupported_family_rejection_count;
        self.locality_unsupported_predicate_rejection_count +=
            other.locality_unsupported_predicate_rejection_count;
    }

    pub fn from_detail_outcome(outcome: &DetailLiveOutcome) -> Self {
        match outcome {
            DetailLiveOutcome::Patch(patch) => Self {
                live_invalidation_event_count: 1,
                live_relevance_match_count: 1,
                live_patch_count: 1,
                live_patch_delivery_count: 1,
                live_patch_field_delta_count: patch.field_deltas().len(),
                live_delivery_width: patch.field_deltas().len(),
                ..Self::default()
            },
            DetailLiveOutcome::Suppressed(_) => Self {
                live_invalidation_event_count: 1,
                live_irrelevant_suppression_count: 1,
                live_suppressed_update_count: 1,
                live_work_avoided_by_irrelevance_count: 1,
                ..Self::default()
            },
            DetailLiveOutcome::Refresh(_) => Self {
                live_invalidation_event_count: 1,
                live_relevance_match_count: 1,
                live_refresh_fallback_count: 1,
                live_refresh_cost_class_count: 1,
                ..Self::default()
            },
        }
    }

    pub fn from_ordered_collection_outcome(outcome: &OrderedCollectionLiveOutcome) -> Self {
        match outcome {
            OrderedCollectionLiveOutcome::Patch(patch) => {
                let mut counters = Self {
                    live_invalidation_event_count: 1,
                    live_relevance_match_count: 1,
                    live_patch_count: 1,
                    live_patch_delivery_count: 1,
                    live_patch_field_delta_count: patch.projected_field_deltas().len(),
                    live_delivery_width: patch.projected_field_deltas().len() + 1,
                    ..Self::default()
                };
                match patch.kind() {
                    OrderedCollectionPatchKind::Membership(_) => {
                        counters.live_collection_membership_change_count = 1;
                    }
                    OrderedCollectionPatchKind::Reordered(_) => {
                        counters.live_collection_reorder_count = 1;
                        counters.live_work_avoided_by_stable_ordering_count = 1;
                    }
                    OrderedCollectionPatchKind::RowUpdated => {}
                }
                counters
            }
            OrderedCollectionLiveOutcome::Suppressed(_) => Self {
                live_invalidation_event_count: 1,
                live_irrelevant_suppression_count: 1,
                live_suppressed_update_count: 1,
                live_work_avoided_by_irrelevance_count: 1,
                ..Self::default()
            },
            OrderedCollectionLiveOutcome::Refresh(_) => Self {
                live_invalidation_event_count: 1,
                live_relevance_match_count: 1,
                live_refresh_fallback_count: 1,
                live_refresh_cost_class_count: 1,
                ..Self::default()
            },
        }
    }

    pub fn from_bounded_materialization_outcome(
        outcome: &BoundedMaterializationLiveOutcome,
    ) -> Self {
        match outcome {
            BoundedMaterializationLiveOutcome::Patch(patch) => {
                let mut counters = Self {
                    live_invalidation_event_count: 1,
                    live_relevance_match_count: 1,
                    live_patch_count: 1,
                    live_patch_delivery_count: 1,
                    live_patch_field_delta_count: patch.projected_field_deltas().len(),
                    live_materialization_patch_count: 1,
                    live_delivery_width: patch.projected_field_deltas().len()
                        + patch.relation_deltas().len()
                        + 1,
                    ..Self::default()
                };
                match patch.kind() {
                    BoundedMaterializationPatchKind::Scope(_) => {
                        counters.live_work_avoided_by_scope_proof_count = 1;
                    }
                    BoundedMaterializationPatchKind::Membership(_) => {
                        counters.live_collection_membership_change_count = 1;
                    }
                    BoundedMaterializationPatchKind::Reordered(_) => {
                        counters.live_collection_reorder_count = 1;
                        counters.live_work_avoided_by_stable_ordering_count = 1;
                    }
                    BoundedMaterializationPatchKind::RowUpdated => {}
                }
                counters
            }
            BoundedMaterializationLiveOutcome::Suppressed(_) => Self {
                live_invalidation_event_count: 1,
                live_irrelevant_suppression_count: 1,
                live_suppressed_update_count: 1,
                live_work_avoided_by_irrelevance_count: 1,
                ..Self::default()
            },
            BoundedMaterializationLiveOutcome::Refresh(_) => Self {
                live_invalidation_event_count: 1,
                live_relevance_match_count: 1,
                live_refresh_fallback_count: 1,
                live_refresh_cost_class_count: 1,
                ..Self::default()
            },
        }
    }

    pub fn from_width_assessment(assessment: &PatchWidthAssessment) -> Self {
        match assessment.resolution() {
            PatchWidthResolution::Deliver => Self {
                live_delivery_width: assessment.measured_width(),
                ..Self::default()
            },
            PatchWidthResolution::Coalesce => Self {
                live_patch_width_overflow_count: 1,
                live_delivery_width: assessment.measured_width(),
                ..Self::default()
            },
            PatchWidthResolution::Refresh(_) => Self {
                live_patch_width_overflow_count: 1,
                live_refresh_fallback_count: 1,
                live_delivery_width: assessment.measured_width(),
                ..Self::default()
            },
            PatchWidthResolution::Reject => Self {
                live_patch_width_overflow_count: 1,
                live_delivery_width: assessment.measured_width(),
                ..Self::default()
            },
        }
    }

    pub fn from_coalescing_decision(decision: &CoalescingDecision) -> Self {
        match decision {
            CoalescingDecision::NotNeeded => Self::default(),
            CoalescingDecision::Admitted { bundle_count } => Self {
                live_coalesced_change_bundle_count: *bundle_count,
                ..Self::default()
            },
        }
    }

    pub fn from_coalescing_error(_error: &LiveCoalescingError) -> Self {
        Self {
            live_coalescing_denial_count: 1,
            ..Self::default()
        }
    }

    pub fn from_refresh_fallback(_fallback: &RefreshFallback) -> Self {
        Self {
            live_refresh_fallback_count: 1,
            live_refresh_cost_class_count: 1,
            ..Self::default()
        }
    }

    pub fn from_refresh_error(_error: &LiveRefreshError) -> Self {
        Self {
            live_refresh_denial_count: 1,
            ..Self::default()
        }
    }

    pub fn from_progress_advance() -> Self {
        Self {
            live_progress_advance_count: 1,
            ..Self::default()
        }
    }

    pub fn from_progress_error(error: &LiveProgressError) -> Self {
        match error {
            LiveProgressError::ChangeSequenceMismatch => Self::default(),
            LiveProgressError::ChangeSequenceGap { .. } => Self {
                live_change_sequence_gap_count: 1,
                ..Self::default()
            },
            LiveProgressError::NonMonotonicOrdinal { .. } => Self {
                live_non_monotonic_sequence_rejection_count: 1,
                ..Self::default()
            },
        }
    }

    pub fn from_promotion_error(error: &LivePromotionError) -> Self {
        match error {
            LivePromotionError::UnsupportedLiveCollectionFamily => Self {
                locality_unsupported_family_rejection_count: 1,
                ..Self::default()
            },
            LivePromotionError::UnsupportedPreflightRoute
            | LivePromotionError::PlanDescriptorMismatch
            | LivePromotionError::BasisPreflight(_)
            | LivePromotionError::Execution(_) => Self {
                live_invalid_promotion_rejection_count: 1,
                ..Self::default()
            },
        }
    }

    pub fn from_unsupported_patch_family() -> Self {
        Self {
            live_unsupported_patch_family_rejection_count: 1,
            ..Self::default()
        }
    }

    pub fn from_locality_match(kind: &LocalityMatchKind) -> Self {
        match kind {
            LocalityMatchKind::InRegionRegionScope => Self {
                locality_region_match_count: 1,
                locality_work_avoided_by_region_narrowing_count: 1,
                locality_work_avoided_vs_broad_control_count: 1,
                ..Self::default()
            },
            LocalityMatchKind::InRegionPartitionScope => Self {
                locality_partition_match_count: 1,
                locality_work_avoided_by_region_narrowing_count: 1,
                locality_work_avoided_vs_broad_control_count: 1,
                ..Self::default()
            },
            LocalityMatchKind::OffRegionSuppressed => Self {
                locality_off_region_suppression_count: 1,
                live_suppressed_update_count: 1,
                locality_work_avoided_by_region_narrowing_count: 1,
                locality_work_avoided_vs_broad_control_count: 1,
                ..Self::default()
            },
        }
    }

    pub fn from_region_scoped_error(error: &RegionScopedLiveError) -> Self {
        match error {
            RegionScopedLiveError::UnsupportedLocalityFamily => Self {
                locality_unsupported_family_rejection_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::UnsupportedLocalityPredicate => Self {
                locality_unsupported_predicate_rejection_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::LocalityBreadthBudgetExceeded { .. } => Self {
                locality_breadth_budget_cross_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::WideningDenied { .. } => Self {
                locality_widening_denial_count: 1,
                locality_widening_budget_cross_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::StreamMemberWidthBudgetExceeded { .. } => Self {
                stream_contract_denial_count: 1,
                stream_member_width_budget_cross_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::BridgeSliceIncompatibility => Self {
                locality_bridge_slice_incompatibility_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::UnsupportedStreamConsumerShape => Self {
                stream_contract_denial_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::LiveExecution(_) => Self::default(),
        }
    }

    pub fn from_stream_lowered_delivery(contract: &StreamLoweredDeliveryContract) -> Self {
        Self {
            stream_contract_admission_count: 1,
            stream_lowered_delivery_count: 1,
            stream_lowered_delivery_member_count: contract.member_count(),
            stream_lowered_delivery_width: contract.delivery_width(),
            ..Self::default()
        }
    }

    pub(crate) fn add_replay_change_count(&mut self, replay_change_count: usize) {
        self.live_replay_change_count += replay_change_count;
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LivePatchDigest(String);

impl LivePatchDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionFieldDelta {
    field: QueryFieldKey,
    old_value: Option<String>,
    new_value: Option<String>,
}

impl ProjectionFieldDelta {
    pub fn field(&self) -> &QueryFieldKey {
        &self.field
    }

    pub fn old_value(&self) -> Option<&str> {
        self.old_value.as_deref()
    }

    pub fn new_value(&self) -> Option<&str> {
        self.new_value.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderingFieldDelta {
    field: QueryFieldKey,
    old_value: Option<String>,
    new_value: Option<String>,
}

impl OrderingFieldDelta {
    pub fn field(&self) -> &QueryFieldKey {
        &self.field
    }

    pub fn old_value(&self) -> Option<&str> {
        self.old_value.as_deref()
    }

    pub fn new_value(&self) -> Option<&str> {
        self.new_value.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailPatch {
    digest: LivePatchDigest,
    field_deltas: Vec<ProjectionFieldDelta>,
}

impl DetailPatch {
    pub fn digest(&self) -> &LivePatchDigest {
        &self.digest
    }

    pub fn field_deltas(&self) -> &[ProjectionFieldDelta] {
        &self.field_deltas
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CollectionMembershipChange {
    EnteredCollection,
    LeftCollection,
}

impl CollectionMembershipChange {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EnteredCollection => "entered_collection",
            Self::LeftCollection => "left_collection",
        }
    }

    fn try_from_transition(transition: &MembershipTransition) -> Option<Self> {
        match (transition.was_member(), transition.is_member()) {
            (false, true) => Some(Self::EnteredCollection),
            (true, false) => Some(Self::LeftCollection),
            (false, false) | (true, true) => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionOrderingChange {
    ordering_field_deltas: Vec<OrderingFieldDelta>,
}

impl CollectionOrderingChange {
    pub fn ordering_field_deltas(&self) -> &[OrderingFieldDelta] {
        &self.ordering_field_deltas
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrderedCollectionPatchKind {
    Membership(CollectionMembershipChange),
    Reordered(CollectionOrderingChange),
    RowUpdated,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderedCollectionPatch {
    digest: LivePatchDigest,
    kind: OrderedCollectionPatchKind,
    projected_field_deltas: Vec<ProjectionFieldDelta>,
}

impl OrderedCollectionPatch {
    pub fn digest(&self) -> &LivePatchDigest {
        &self.digest
    }

    pub fn kind(&self) -> &OrderedCollectionPatchKind {
        &self.kind
    }

    pub fn projected_field_deltas(&self) -> &[ProjectionFieldDelta] {
        &self.projected_field_deltas
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrderedCollectionLiveOutcome {
    Patch(OrderedCollectionPatch),
    Suppressed(SuppressionReason),
    Refresh(RefreshFallback),
}

impl OrderedCollectionLiveOutcome {
    pub fn suppression_decision(&self) -> SuppressionDecision {
        match self {
            Self::Patch(_) => SuppressionDecision::Deliver,
            Self::Refresh(_) => SuppressionDecision::Deliver,
            Self::Suppressed(reason) => SuppressionDecision::Suppress(reason.clone()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MaterializationScopeChange {
    EnteredScope,
    LeftScope,
}

impl MaterializationScopeChange {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EnteredScope => "entered_scope",
            Self::LeftScope => "left_scope",
        }
    }

    fn try_from_transition(transition: &MaterializationScopeTransition) -> Option<Self> {
        match (transition.was_in_scope(), transition.is_in_scope()) {
            (false, true) => Some(Self::EnteredScope),
            (true, false) => Some(Self::LeftScope),
            (false, false) | (true, true) => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundedMaterializationPatchKind {
    Scope(MaterializationScopeChange),
    Membership(CollectionMembershipChange),
    Reordered(CollectionOrderingChange),
    RowUpdated,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedMaterializationPatch {
    digest: LivePatchDigest,
    kind: BoundedMaterializationPatchKind,
    projected_field_deltas: Vec<ProjectionFieldDelta>,
    relation_deltas: Vec<String>,
}

impl BoundedMaterializationPatch {
    pub fn digest(&self) -> &LivePatchDigest {
        &self.digest
    }

    pub fn kind(&self) -> &BoundedMaterializationPatchKind {
        &self.kind
    }

    pub fn projected_field_deltas(&self) -> &[ProjectionFieldDelta] {
        &self.projected_field_deltas
    }

    pub fn relation_deltas(&self) -> &[String] {
        &self.relation_deltas
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundedMaterializationLiveOutcome {
    Patch(BoundedMaterializationPatch),
    Suppressed(SuppressionReason),
    Refresh(RefreshFallback),
}

impl BoundedMaterializationLiveOutcome {
    pub fn suppression_decision(&self) -> SuppressionDecision {
        match self {
            Self::Patch(_) => SuppressionDecision::Deliver,
            Self::Refresh(_) => SuppressionDecision::Deliver,
            Self::Suppressed(reason) => SuppressionDecision::Suppress(reason.clone()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DetailLiveOutcome {
    Patch(DetailPatch),
    Suppressed(SuppressionReason),
    Refresh(RefreshFallback),
}

impl DetailLiveOutcome {
    pub fn suppression_decision(&self) -> SuppressionDecision {
        match self {
            Self::Patch(_) => SuppressionDecision::Deliver,
            Self::Refresh(_) => SuppressionDecision::Deliver,
            Self::Suppressed(reason) => SuppressionDecision::Suppress(reason.clone()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LivePatchPayload {
    Detail(DetailPatch),
    OrderedCollection(OrderedCollectionPatch),
    BoundedMaterialization(BoundedMaterializationPatch),
    Suppressed(SuppressionReason),
    Refresh(RefreshFallback),
    ProgressAdvance { ordinal: u64 },
    Coalesced(CoalescingDecision),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LivePatchEnvelope {
    query_digest: String,
    result_digest: String,
    delivery_digest: String,
    replay_digest: String,
    basis_digest: String,
    subscription_digest: String,
    family: LiveQueryFamily,
    payload: LivePatchPayload,
}

impl LivePatchEnvelope {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn subscription_digest(&self) -> &str {
        &self.subscription_digest
    }

    pub fn family(&self) -> &LiveQueryFamily {
        &self.family
    }

    pub fn payload(&self) -> &LivePatchPayload {
        &self.payload
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveReplayBundle {
    query_digest: String,
    result_digest: String,
    delivery_digest: String,
    replay_digest: String,
    basis_digest: String,
    subscription_digest: String,
    counter_snapshot: LivePolicyCounters,
    patch_envelope: LivePatchEnvelope,
}

impl LiveReplayBundle {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn subscription_digest(&self) -> &str {
        &self.subscription_digest
    }

    pub fn counter_snapshot(&self) -> &LivePolicyCounters {
        &self.counter_snapshot
    }

    pub fn patch_envelope(&self) -> &LivePatchEnvelope {
        &self.patch_envelope
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveReplayStepInput {
    change_summary: BridgeChangeSummary,
    next_ordinal: LiveChangeOrdinal,
    next_basis: ResolvedSnapshotBasis,
}

impl LiveReplayStepInput {
    pub fn new(
        change_summary: BridgeChangeSummary,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Self {
        Self {
            change_summary,
            next_ordinal,
            next_basis,
        }
    }

    pub fn change_summary(&self) -> &BridgeChangeSummary {
        &self.change_summary
    }

    pub fn next_ordinal(&self) -> &LiveChangeOrdinal {
        &self.next_ordinal
    }

    pub fn next_basis(&self) -> &ResolvedSnapshotBasis {
        &self.next_basis
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveReplayRun {
    final_plan: LiveQueryPlan,
    bundles: Vec<LiveReplayBundle>,
}

impl LiveReplayRun {
    pub fn final_plan(&self) -> &LiveQueryPlan {
        &self.final_plan
    }

    pub fn bundles(&self) -> &[LiveReplayBundle] {
        &self.bundles
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveDetailPatchError {
    UnsupportedFamily,
    UnsupportedRelevantClass(RelevantChangeClass),
    RelevantChangeWithoutProjectedDelta,
    WidthBudgetExceeded { limit: usize, actual: usize },
    CoalescingRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveCollectionPatchError {
    UnsupportedFamily,
    UnsupportedRelevantClass(RelevantChangeClass),
    MissingMembershipTransition,
    NoMembershipDelta,
    MissingOrderingDelta,
    MissingProjectedDelta,
    WidthBudgetExceeded { limit: usize, actual: usize },
    CoalescingRequired { limit: usize, actual: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveBoundedMaterializationPatchError {
    UnsupportedFamily,
    MissingMaterializationScopeTransition,
    NoMaterializationScopeDelta,
    MissingMembershipTransition,
    NoMembershipDelta,
    MissingOrderingDelta,
    MissingProjectedDelta,
    WidthBudgetExceeded { limit: usize, actual: usize },
    CoalescingRequired { limit: usize, actual: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveExecutionReport {
    query_digest: String,
    result_digest: String,
    delivery_digest: String,
    replay_digest: String,
    family: LiveQueryFamily,
    outcome_kind: String,
    outcome_digest: String,
    basis_digest: String,
    subscription_digest: String,
}

impl LiveExecutionReport {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn family(&self) -> &LiveQueryFamily {
        &self.family
    }

    pub fn outcome_kind(&self) -> &str {
        &self.outcome_kind
    }

    pub fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn subscription_digest(&self) -> &str {
        &self.subscription_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveExecutionEnvelope {
    report: LiveExecutionReport,
    patch_envelope: LivePatchEnvelope,
    replay_bundle: LiveReplayBundle,
    counters: LivePolicyCounters,
}

impl LiveExecutionEnvelope {
    pub fn report(&self) -> &LiveExecutionReport {
        &self.report
    }

    pub fn patch_envelope(&self) -> &LivePatchEnvelope {
        &self.patch_envelope
    }

    pub fn replay_bundle(&self) -> &LiveReplayBundle {
        &self.replay_bundle
    }

    pub fn counters(&self) -> &LivePolicyCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveExecutionError {
    Detail(LiveDetailPatchError),
    OrderedCollection(LiveCollectionPatchError),
    BoundedMaterialization(LiveBoundedMaterializationPatchError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveReplayError {
    Progress(LiveProgressError),
    Execution(LiveExecutionError),
}

impl From<LiveProgressError> for LiveReplayError {
    fn from(value: LiveProgressError) -> Self {
        Self::Progress(value)
    }
}

impl From<LiveExecutionError> for LiveReplayError {
    fn from(value: LiveExecutionError) -> Self {
        Self::Execution(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveCertificationLane {
    lane_name: String,
    execution: LiveExecutionEnvelope,
}

impl LiveCertificationLane {
    pub fn lane_name(&self) -> &str {
        &self.lane_name
    }

    pub fn execution(&self) -> &LiveExecutionEnvelope {
        &self.execution
    }

    pub fn new(lane_name: impl Into<String>, execution: LiveExecutionEnvelope) -> Self {
        Self {
            lane_name: lane_name.into(),
            execution,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveCertificationRejectionLane {
    lane_name: String,
    failure_class: String,
    failure_digest: String,
    counters: LivePolicyCounters,
}

impl LiveCertificationRejectionLane {
    pub fn lane_name(&self) -> &str {
        &self.lane_name
    }

    pub fn failure_class(&self) -> &str {
        &self.failure_class
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub fn counters(&self) -> &LivePolicyCounters {
        &self.counters
    }

    pub fn new(
        lane_name: impl Into<String>,
        failure_class: impl Into<String>,
        failure_digest: impl Into<String>,
        counters: LivePolicyCounters,
    ) -> Self {
        Self {
            lane_name: lane_name.into(),
            failure_class: failure_class.into(),
            failure_digest: failure_digest.into(),
            counters,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneFiveLiveArtifact {
    suite_name: String,
    certification_digest: String,
    coverage_digest: String,
    counter_snapshot: LivePolicyCounters,
    canonical_lane_count: usize,
    rejection_lane_count: usize,
}

impl MilestoneFiveLiveArtifact {
    pub fn suite_name(&self) -> &str {
        &self.suite_name
    }

    pub fn certification_digest(&self) -> &str {
        &self.certification_digest
    }

    pub fn coverage_digest(&self) -> &str {
        &self.coverage_digest
    }

    pub fn counter_snapshot(&self) -> &LivePolicyCounters {
        &self.counter_snapshot
    }

    pub fn canonical_lane_count(&self) -> usize {
        self.canonical_lane_count
    }

    pub fn rejection_lane_count(&self) -> usize {
        self.rejection_lane_count
    }
}

pub struct MilestoneFiveLiveAdapter;

impl MilestoneFiveLiveAdapter {
    pub fn detail_patch_lane(
        live: &LiveQueryPlan,
        change: &BridgeChangeSummary,
    ) -> Result<LiveCertificationLane, LiveExecutionError> {
        Self::canonical_lane("detail-live-patch-parity", live, change)
    }

    pub fn suppression_lane(
        live: &LiveQueryPlan,
        change: &BridgeChangeSummary,
    ) -> Result<LiveCertificationLane, LiveExecutionError> {
        Self::canonical_lane("irrelevant-update-suppression", live, change)
    }

    pub fn ordered_collection_patch_lane(
        live: &LiveQueryPlan,
        change: &BridgeChangeSummary,
    ) -> Result<LiveCertificationLane, LiveExecutionError> {
        Self::canonical_lane("ordered-collection-live-patch-parity", live, change)
    }

    pub fn bounded_materialization_patch_lane(
        live: &LiveQueryPlan,
        change: &BridgeChangeSummary,
    ) -> Result<LiveCertificationLane, LiveExecutionError> {
        Self::canonical_lane("bounded-materialization-live-patch-parity", live, change)
    }

    pub fn progress_advance_lane(
        live: &LiveQueryPlan,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Result<LiveCertificationLane, LiveProgressError> {
        let progress = live.progress_basis().advance(
            live.progress_basis().change_sequence_id(),
            next_ordinal,
            next_basis,
        )?;
        Ok(LiveCertificationLane::new(
            "live-progress-basis-parity",
            LiveExecutionEnvelope {
                report: {
                    let outcome_kind = "progress_advance".to_string();
                    let outcome_digest = format!(
                        "ordinal:{}:replay:{}",
                        progress.last_ordinal().value(),
                        progress.replay_digest().as_str()
                    );
                    live_execution_report(live, outcome_kind, outcome_digest)
                },
                patch_envelope: {
                    let outcome_kind = "progress_advance".to_string();
                    let outcome_digest = format!(
                        "ordinal:{}:replay:{}",
                        progress.last_ordinal().value(),
                        progress.replay_digest().as_str()
                    );
                    patch_envelope_from_payload(
                        live,
                        LivePatchPayload::ProgressAdvance {
                            ordinal: progress.last_ordinal().value(),
                        },
                        outcome_kind,
                        outcome_digest,
                        progress
                            .current_basis()
                            .proof()
                            .digest()
                            .as_str()
                            .to_string(),
                        progress.replay_digest().as_str().to_string(),
                    )
                },
                replay_bundle: {
                    let patch_envelope = patch_envelope_from_payload(
                        live,
                        LivePatchPayload::ProgressAdvance {
                            ordinal: progress.last_ordinal().value(),
                        },
                        "progress_advance".to_string(),
                        format!(
                            "ordinal:{}:replay:{}",
                            progress.last_ordinal().value(),
                            progress.replay_digest().as_str()
                        ),
                        progress
                            .current_basis()
                            .proof()
                            .digest()
                            .as_str()
                            .to_string(),
                        progress.replay_digest().as_str().to_string(),
                    );
                    replay_bundle_from_patch_envelope(
                        patch_envelope,
                        LivePolicyCounters::from_progress_advance(),
                    )
                },
                counters: LivePolicyCounters::from_progress_advance(),
            },
        ))
    }

    pub fn refresh_fallback_lane(
        live: &LiveQueryPlan,
        admission_class: RefreshAdmissionClass,
    ) -> Result<LiveCertificationLane, LiveRefreshError> {
        let fallback = live.request_refresh_fallback(admission_class)?;
        Ok(LiveCertificationLane::new(
            "refresh-fallback-equivalence",
            LiveExecutionEnvelope {
                report: live_execution_report(
                    live,
                    "refresh".to_string(),
                    format!(
                        "refresh:{}:{}",
                        fallback.admission_class().as_str(),
                        fallback.admission_status().as_str()
                    ),
                ),
                patch_envelope: patch_envelope_from_payload(
                    live,
                    LivePatchPayload::Refresh(fallback.clone()),
                    "refresh".to_string(),
                    format!(
                        "refresh:{}:{}",
                        fallback.admission_class().as_str(),
                        fallback.admission_status().as_str()
                    ),
                    live.progress_basis()
                        .current_basis()
                        .proof()
                        .digest()
                        .as_str()
                        .to_string(),
                    live.progress_basis().replay_digest().as_str().to_string(),
                ),
                replay_bundle: replay_bundle_from_patch_envelope(
                    patch_envelope_from_payload(
                        live,
                        LivePatchPayload::Refresh(fallback.clone()),
                        "refresh".to_string(),
                        format!(
                            "refresh:{}:{}",
                            fallback.admission_class().as_str(),
                            fallback.admission_status().as_str()
                        ),
                        live.progress_basis()
                            .current_basis()
                            .proof()
                            .digest()
                            .as_str()
                            .to_string(),
                        live.progress_basis().replay_digest().as_str().to_string(),
                    ),
                    LivePolicyCounters::from_refresh_fallback(&fallback),
                ),
                counters: LivePolicyCounters::from_refresh_fallback(&fallback),
            },
        ))
    }

    pub fn coalesced_delivery_lane(
        live: &LiveQueryPlan,
        bundle_count: usize,
    ) -> Result<LiveCertificationLane, LiveCoalescingError> {
        let decision = live.request_coalesced_delivery(bundle_count)?;
        Ok(LiveCertificationLane::new(
            "coalesced-sequence-replay-parity",
            LiveExecutionEnvelope {
                report: live_execution_report(
                    live,
                    "coalesced_delivery".to_string(),
                    format!("{decision:?}"),
                ),
                patch_envelope: patch_envelope_from_payload(
                    live,
                    LivePatchPayload::Coalesced(decision.clone()),
                    "coalesced_delivery".to_string(),
                    format!("{decision:?}"),
                    live.progress_basis()
                        .current_basis()
                        .proof()
                        .digest()
                        .as_str()
                        .to_string(),
                    live.progress_basis().replay_digest().as_str().to_string(),
                ),
                replay_bundle: replay_bundle_from_patch_envelope(
                    patch_envelope_from_payload(
                        live,
                        LivePatchPayload::Coalesced(decision.clone()),
                        "coalesced_delivery".to_string(),
                        format!("{decision:?}"),
                        live.progress_basis()
                            .current_basis()
                            .proof()
                            .digest()
                            .as_str()
                            .to_string(),
                        live.progress_basis().replay_digest().as_str().to_string(),
                    ),
                    LivePolicyCounters::from_coalescing_decision(&decision),
                ),
                counters: LivePolicyCounters::from_coalescing_decision(&decision),
            },
        ))
    }

    pub fn canonical_lane(
        lane_name: impl Into<String>,
        live: &LiveQueryPlan,
        change: &BridgeChangeSummary,
    ) -> Result<LiveCertificationLane, LiveExecutionError> {
        let execution = execute_live_change(live, change)?;
        Ok(LiveCertificationLane::new(lane_name, execution))
    }

    pub fn refresh_rejection_lane(
        lane_name: impl Into<String>,
        live: &LiveQueryPlan,
        admission_class: RefreshAdmissionClass,
    ) -> Result<LiveCertificationRejectionLane, LiveExpectedRejectionError> {
        match live.request_refresh_fallback(admission_class.clone()) {
            Ok(fallback) => Err(LiveExpectedRejectionError::UnexpectedRefreshAdmission {
                admission_class: fallback.admission_class().clone(),
                admission_status: fallback.admission_status().clone(),
            }),
            Err(error) => Ok(LiveCertificationRejectionLane::new(
                lane_name,
                "forbidden-refresh-escape-hatch",
                format!("{error:?}"),
                LivePolicyCounters::from_refresh_error(&error),
            )),
        }
    }

    pub fn coalescing_rejection_lane(
        lane_name: impl Into<String>,
        live: &LiveQueryPlan,
        bundle_count: usize,
    ) -> Result<LiveCertificationRejectionLane, LiveExpectedRejectionError> {
        match live.request_coalesced_delivery(bundle_count) {
            Ok(decision) => {
                Err(LiveExpectedRejectionError::UnexpectedCoalescingAdmission { decision })
            }
            Err(error) => Ok(LiveCertificationRejectionLane::new(
                lane_name,
                "forbidden-coalescing-class",
                format!("{error:?}"),
                LivePolicyCounters::from_coalescing_error(&error),
            )),
        }
    }

    pub fn progress_rejection_lane(
        lane_name: impl Into<String>,
        live: &LiveQueryPlan,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Result<LiveCertificationRejectionLane, LiveExpectedRejectionError> {
        match live.progress_basis().advance(
            live.progress_basis().change_sequence_id(),
            next_ordinal,
            next_basis,
        ) {
            Ok(progress) => Err(LiveExpectedRejectionError::UnexpectedProgressAdvance {
                ordinal: progress.last_ordinal().value(),
                replay_digest: progress.replay_digest().as_str().to_string(),
            }),
            Err(error) => Ok(LiveCertificationRejectionLane::new(
                lane_name,
                "non-monotonic-change-sequence",
                format!("{error:?}"),
                LivePolicyCounters::from_progress_error(&error),
            )),
        }
    }

    pub fn artifact(
        suite_name: impl Into<String>,
        canonical_lanes: &[LiveCertificationLane],
        rejection_lanes: &[LiveCertificationRejectionLane],
    ) -> MilestoneFiveLiveArtifact {
        build_milestone_five_live_artifact(suite_name, canonical_lanes, rejection_lanes)
    }

    pub fn forbidden_refresh_rejection_lane(
        live: &LiveQueryPlan,
        admission_class: RefreshAdmissionClass,
    ) -> Result<LiveCertificationRejectionLane, LiveExpectedRejectionError> {
        Self::refresh_rejection_lane("forbidden-refresh-escape-hatch", live, admission_class)
    }

    pub fn forbidden_coalescing_rejection_lane(
        live: &LiveQueryPlan,
        bundle_count: usize,
    ) -> Result<LiveCertificationRejectionLane, LiveExpectedRejectionError> {
        Self::coalescing_rejection_lane("forbidden-coalescing-class", live, bundle_count)
    }

    pub fn non_monotonic_progress_rejection_lane(
        live: &LiveQueryPlan,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Result<LiveCertificationRejectionLane, LiveExpectedRejectionError> {
        Self::progress_rejection_lane(
            "non-monotonic-change-sequence",
            live,
            next_ordinal,
            next_basis,
        )
    }
}

pub fn promote_preflight_bundle_to_live(
    preflight: &ExecutionPreflightBundle,
) -> Result<LiveQueryPlan, LivePromotionError> {
    let route = preflight.plan().query().route();
    match route {
        crate::planning::PlannedExecutionRoute::RuntimeSnapshotRead
        | crate::planning::PlannedExecutionRoute::RuntimeExpandedSnapshotRead => {}
        crate::planning::PlannedExecutionRoute::StoreSnapshotRead => {
            return Err(LivePromotionError::UnsupportedPreflightRoute);
        }
    }

    let descriptor = preflight.plan().live_promotion().clone();
    let plan_digest = preflight.plan().query().plan_digest();
    if descriptor.plan_digest() != plan_digest {
        return Err(LivePromotionError::PlanDescriptorMismatch);
    }
    if preflight.plan().collection().is_some_and(|collection| {
        collection.planning_context().result_family()
            != &crate::collection::CollectionResultFamily::OrdinaryCollection
    }) {
        return Err(LivePromotionError::UnsupportedLiveCollectionFamily);
    }

    let start_basis = LiveStartBasis::new(preflight.basis().clone());
    let mut parts = vec![
        format!("plan:{}", descriptor.plan_digest().as_str()),
        format!("family:{}", descriptor.family().as_str()),
        format!("basis:{}", preflight.basis().proof().digest().as_str()),
        format!(
            "incremental:{}",
            descriptor
                .incremental_eligibility()
                .maintenance_class()
                .as_str()
        ),
    ];
    parts.extend(descriptor.performance_report().digest_parts());
    if let Some(collection_digest) = descriptor.collection_digest() {
        parts.push(format!("collection:{}", collection_digest.as_str()));
    }
    let subscription_digest = LiveSubscriptionDigest::from_parts(&parts);
    let progress_basis = LiveProgressBasis::initial(&subscription_digest, &start_basis);
    let baseline_result_digest = execute_preflight_bundle(preflight)?
        .report()
        .result_digest()
        .as_str()
        .to_string();

    Ok(LiveQueryPlan {
        descriptor,
        start_basis,
        progress_basis,
        subscription_digest,
        baseline_result_digest,
    })
}

pub fn execute_live_change(
    live: &LiveQueryPlan,
    change: &BridgeChangeSummary,
) -> Result<LiveExecutionEnvelope, LiveExecutionError> {
    match live.descriptor().family() {
        LiveQueryFamily::Detail => {
            let outcome = live
                .detail_live_outcome(change)
                .map_err(LiveExecutionError::Detail)?;
            let (outcome_kind, outcome_digest) = detail_outcome_digest(&outcome);
            let payload = match &outcome {
                DetailLiveOutcome::Patch(patch) => LivePatchPayload::Detail(patch.clone()),
                DetailLiveOutcome::Suppressed(reason) => {
                    LivePatchPayload::Suppressed(reason.clone())
                }
                DetailLiveOutcome::Refresh(fallback) => LivePatchPayload::Refresh(fallback.clone()),
            };
            let counters = LivePolicyCounters::from_detail_outcome(&outcome);
            let patch_envelope = patch_envelope_from_payload(
                live,
                payload,
                outcome_kind.clone(),
                outcome_digest.clone(),
                live.progress_basis()
                    .current_basis()
                    .proof()
                    .digest()
                    .as_str()
                    .to_string(),
                live.progress_basis().replay_digest().as_str().to_string(),
            );
            Ok(LiveExecutionEnvelope {
                report: live_execution_report(live, outcome_kind, outcome_digest),
                patch_envelope: patch_envelope.clone(),
                replay_bundle: replay_bundle_from_patch_envelope(patch_envelope, counters.clone()),
                counters,
            })
        }
        LiveQueryFamily::OrderedCollection => {
            let outcome = live
                .ordered_collection_live_outcome(change)
                .map_err(LiveExecutionError::OrderedCollection)?;
            let (outcome_kind, outcome_digest) = ordered_collection_outcome_digest(&outcome);
            let payload = match &outcome {
                OrderedCollectionLiveOutcome::Patch(patch) => {
                    LivePatchPayload::OrderedCollection(patch.clone())
                }
                OrderedCollectionLiveOutcome::Suppressed(reason) => {
                    LivePatchPayload::Suppressed(reason.clone())
                }
                OrderedCollectionLiveOutcome::Refresh(fallback) => {
                    LivePatchPayload::Refresh(fallback.clone())
                }
            };
            let counters = LivePolicyCounters::from_ordered_collection_outcome(&outcome);
            let patch_envelope = patch_envelope_from_payload(
                live,
                payload,
                outcome_kind.clone(),
                outcome_digest.clone(),
                live.progress_basis()
                    .current_basis()
                    .proof()
                    .digest()
                    .as_str()
                    .to_string(),
                live.progress_basis().replay_digest().as_str().to_string(),
            );
            Ok(LiveExecutionEnvelope {
                report: live_execution_report(live, outcome_kind, outcome_digest),
                patch_envelope: patch_envelope.clone(),
                replay_bundle: replay_bundle_from_patch_envelope(patch_envelope, counters.clone()),
                counters,
            })
        }
        LiveQueryFamily::BoundedMaterialization => {
            let outcome = live
                .bounded_materialization_live_outcome(change)
                .map_err(LiveExecutionError::BoundedMaterialization)?;
            let (outcome_kind, outcome_digest) = bounded_outcome_digest(&outcome);
            let payload = match &outcome {
                BoundedMaterializationLiveOutcome::Patch(patch) => {
                    LivePatchPayload::BoundedMaterialization(patch.clone())
                }
                BoundedMaterializationLiveOutcome::Suppressed(reason) => {
                    LivePatchPayload::Suppressed(reason.clone())
                }
                BoundedMaterializationLiveOutcome::Refresh(fallback) => {
                    LivePatchPayload::Refresh(fallback.clone())
                }
            };
            let counters = LivePolicyCounters::from_bounded_materialization_outcome(&outcome);
            let patch_envelope = patch_envelope_from_payload(
                live,
                payload,
                outcome_kind.clone(),
                outcome_digest.clone(),
                live.progress_basis()
                    .current_basis()
                    .proof()
                    .digest()
                    .as_str()
                    .to_string(),
                live.progress_basis().replay_digest().as_str().to_string(),
            );
            Ok(LiveExecutionEnvelope {
                report: live_execution_report(live, outcome_kind, outcome_digest),
                patch_envelope: patch_envelope.clone(),
                replay_bundle: replay_bundle_from_patch_envelope(patch_envelope, counters.clone()),
                counters,
            })
        }
    }
}

pub fn admit_region_scoped_live_plan(
    live: &LiveQueryPlan,
    locality: LocalityPredicateContract,
) -> Result<RegionScopedLivePlan, RegionScopedLiveError> {
    let admission_class = match (live.descriptor().family(), locality.scope_kind()) {
        (LiveQueryFamily::Detail, LocalityScopeKind::Region) => {
            LocalityAdmissionClass::DetailRegion
        }
        (LiveQueryFamily::Detail, LocalityScopeKind::Partition) => {
            LocalityAdmissionClass::DetailPartition
        }
        (LiveQueryFamily::OrderedCollection, LocalityScopeKind::Partition) => {
            LocalityAdmissionClass::OrderedCollectionPartition
        }
        (LiveQueryFamily::BoundedMaterialization, LocalityScopeKind::Region) => {
            LocalityAdmissionClass::BoundedMaterializationRegion
        }
        (LiveQueryFamily::OrderedCollection, LocalityScopeKind::Region)
        | (LiveQueryFamily::BoundedMaterialization, LocalityScopeKind::Partition) => {
            return Err(RegionScopedLiveError::UnsupportedLocalityPredicate);
        }
    };

    let locality_subscription_digest = hash_parts(&[
        format!("subscription:{}", live.subscription_digest().as_str()),
        format!("locality:{}", locality.digest().as_str()),
        format!("admission:{}", admission_class.as_str()),
    ]);

    let (
        locality_cost_posture,
        locality_breadth_budget,
        locality_widening_budget,
        stream_lowering_cost_posture,
        stream_member_width_budget,
    ) = match admission_class {
        LocalityAdmissionClass::DetailRegion | LocalityAdmissionClass::DetailPartition => (
            LocalityCostPosture::SingleSliceNarrowing,
            LocalityBreadthBudget::single_surface(),
            LocalityWideningBudget::deny_all(),
            StreamLoweringCostPosture::SingleDetailCurrentStateMember,
            StreamMemberWidthBudget::single_member(),
        ),
        LocalityAdmissionClass::OrderedCollectionPartition => (
            LocalityCostPosture::PartitionScopedMembershipNarrowing,
            LocalityBreadthBudget::single_surface(),
            LocalityWideningBudget::deny_all(),
            StreamLoweringCostPosture::CdcPatchWithProjectedDeltas,
            StreamMemberWidthBudget::cdc_projected_patch(),
        ),
        LocalityAdmissionClass::BoundedMaterializationRegion => (
            LocalityCostPosture::BoundedTraversalRegionNarrowing,
            LocalityBreadthBudget::single_surface(),
            LocalityWideningBudget::deny_all(),
            StreamLoweringCostPosture::BoundedMaterializationDeferred,
            StreamMemberWidthBudget::single_member(),
        ),
    };

    Ok(RegionScopedLivePlan {
        live: live.clone(),
        locality,
        admission_class,
        locality_subscription_digest,
        locality_cost_posture,
        locality_breadth_budget,
        locality_widening_budget,
        stream_lowering_cost_posture,
        stream_member_width_budget,
    })
}

fn classify_locality_match(
    plan: &RegionScopedLivePlan,
    change: &BridgeChangeSummary,
) -> Result<LocalityMatchKind, RegionScopedLiveError> {
    let expected_category = match plan.locality.scope_kind() {
        LocalityScopeKind::Region => BridgeSliceCategory::EntityRegion,
        LocalityScopeKind::Partition => BridgeSliceCategory::EntityPartition,
    };

    let exact_match_count = change
        .locality_slices()
        .iter()
        .filter(|slice| {
            slice.category() == &expected_category && slice.scope() == plan.locality.scope()
        })
        .count();
    if exact_match_count > plan.locality_breadth_budget().limit() {
        return Err(RegionScopedLiveError::LocalityBreadthBudgetExceeded {
            limit: plan.locality_breadth_budget().limit(),
            actual: exact_match_count,
        });
    }
    if exact_match_count > 0 {
        return Ok(match plan.locality.scope_kind() {
            LocalityScopeKind::Region => LocalityMatchKind::InRegionRegionScope,
            LocalityScopeKind::Partition => LocalityMatchKind::InRegionPartitionScope,
        });
    }

    let has_expected_category = change
        .locality_slices()
        .iter()
        .any(|slice| slice.category() == &expected_category);
    if has_expected_category {
        return Ok(LocalityMatchKind::OffRegionSuppressed);
    }

    let has_coarse_fallback = change
        .locality_slices()
        .iter()
        .any(|slice| slice.category() == &BridgeSliceCategory::CoarseFallback);
    if has_coarse_fallback {
        let received = change
            .locality_slices()
            .iter()
            .map(|slice| format!("{}:{}", slice.category().as_str(), slice.scope()))
            .collect();
        return Err(RegionScopedLiveError::WideningDenied {
            expected: format!("{}:{}", expected_category.as_str(), plan.locality.scope()),
            received,
        });
    }

    Err(RegionScopedLiveError::BridgeSliceIncompatibility)
}

pub fn execute_region_scoped_live_change(
    plan: &RegionScopedLivePlan,
    change: &BridgeChangeSummary,
) -> Result<RegionScopedLiveExecutionEnvelope, RegionScopedLiveError> {
    let locality_match = classify_locality_match(plan, change)?;
    let locality_counters = LivePolicyCounters::from_locality_match(&locality_match);

    match locality_match {
        LocalityMatchKind::InRegionRegionScope | LocalityMatchKind::InRegionPartitionScope => {
            let mut execution = execute_live_change(plan.live(), change)?;
            let mut counters = execution.counters().clone();
            counters.absorb(&locality_counters);
            let report = RegionScopedExecutionReport {
                query_digest: execution.report().query_digest().to_string(),
                locality_digest: plan.locality().digest().as_str().to_string(),
                locality_outcome: match locality_match {
                    LocalityMatchKind::InRegionRegionScope => "in_region_region".to_string(),
                    LocalityMatchKind::InRegionPartitionScope => "in_region_partition".to_string(),
                    LocalityMatchKind::OffRegionSuppressed => unreachable!(),
                },
                result_digest: execution.report().result_digest().to_string(),
                delivery_digest: execution.report().delivery_digest().to_string(),
                replay_digest: execution.report().replay_digest().to_string(),
            };
            execution.counters = counters.clone();
            let mut replay_bundle = execution.replay_bundle().clone();
            replay_bundle.counter_snapshot = counters.clone();
            Ok(RegionScopedLiveExecutionEnvelope {
                report,
                patch_envelope: execution.patch_envelope().clone(),
                replay_bundle,
                counters,
            })
        }
        LocalityMatchKind::OffRegionSuppressed => {
            let payload = LivePatchPayload::Suppressed(SuppressionReason::OffRegionChange {
                scope_kind: plan.locality().scope_kind().clone(),
                scope: plan.locality().scope().to_string(),
                locality_digest: plan.locality().digest().as_str().to_string(),
            });
            let patch_envelope = patch_envelope_from_payload(
                plan.live(),
                payload,
                "off_region_suppressed".to_string(),
                format!("off_region:{}", plan.locality().digest().as_str()),
                plan.live()
                    .progress_basis()
                    .current_basis()
                    .proof()
                    .digest()
                    .as_str()
                    .to_string(),
                plan.live()
                    .progress_basis()
                    .replay_digest()
                    .as_str()
                    .to_string(),
            );
            let replay_bundle = replay_bundle_from_patch_envelope(
                patch_envelope.clone(),
                locality_counters.clone(),
            );
            Ok(RegionScopedLiveExecutionEnvelope {
                report: RegionScopedExecutionReport {
                    query_digest: plan.live().descriptor().query_digest().as_str().to_string(),
                    locality_digest: plan.locality().digest().as_str().to_string(),
                    locality_outcome: "off_region_suppressed".to_string(),
                    result_digest: patch_envelope.result_digest().to_string(),
                    delivery_digest: patch_envelope.delivery_digest().to_string(),
                    replay_digest: patch_envelope.replay_digest().to_string(),
                },
                patch_envelope,
                replay_bundle,
                counters: locality_counters,
            })
        }
    }
}

pub fn lower_region_scoped_execution_to_stream_contract(
    plan: &RegionScopedLivePlan,
    execution: &RegionScopedLiveExecutionEnvelope,
    consumer_shape: StreamConsumerShape,
) -> Result<StreamLoweredDeliveryContract, RegionScopedLiveError> {
    match (plan.live().descriptor().family(), &consumer_shape) {
        (LiveQueryFamily::Detail, StreamConsumerShape::DetailCurrentState)
        | (LiveQueryFamily::OrderedCollection, StreamConsumerShape::CdcCollectionPatch) => {}
        _ => return Err(RegionScopedLiveError::UnsupportedStreamConsumerShape),
    }

    let (member_count, delivery_width) =
        stream_contract_widths(execution.patch_envelope().payload(), &consumer_shape);
    if delivery_width > plan.stream_member_width_budget().limit() {
        return Err(RegionScopedLiveError::StreamMemberWidthBudgetExceeded {
            limit: plan.stream_member_width_budget().limit(),
            actual: delivery_width,
        });
    }

    Ok(StreamLoweredDeliveryContract {
        query_digest: execution.report().query_digest().to_string(),
        locality_digest: plan.locality().digest().as_str().to_string(),
        delivery_digest: execution.report().delivery_digest().to_string(),
        stream_contract_digest: hash_parts(&[
            format!("query:{}", execution.report().query_digest()),
            format!("locality:{}", plan.locality().digest().as_str()),
            format!("delivery:{}", execution.report().delivery_digest()),
            format!("consumer_shape:{}", consumer_shape.as_str()),
            format!("members:{member_count}"),
            format!("width:{delivery_width}"),
            format!(
                "cost_posture:{}",
                plan.stream_lowering_cost_posture().as_str()
            ),
        ]),
        consumer_shape,
        member_count,
        delivery_width,
        cost_posture: plan.stream_lowering_cost_posture().clone(),
    })
}

fn stream_contract_widths(
    payload: &LivePatchPayload,
    consumer_shape: &StreamConsumerShape,
) -> (usize, usize) {
    match (payload, consumer_shape) {
        (LivePatchPayload::Detail(_), StreamConsumerShape::DetailCurrentState) => (1, 1),
        (LivePatchPayload::OrderedCollection(patch), StreamConsumerShape::CdcCollectionPatch) => {
            (1, 1 + patch.projected_field_deltas().len())
        }
        (LivePatchPayload::Suppressed(_), _) => (1, 1),
        _ => (1, 1),
    }
}

pub fn replay_live_sequence(
    live: &LiveQueryPlan,
    steps: &[LiveReplayStepInput],
) -> Result<LiveReplayRun, LiveReplayError> {
    let mut current = live.clone();
    let mut bundles = Vec::with_capacity(steps.len());

    for step in steps {
        current =
            current.advance_progress(step.next_ordinal().clone(), step.next_basis().clone())?;
        let execution = execute_live_change(&current, step.change_summary())?;
        let mut replay_bundle = execution.replay_bundle().clone();
        replay_bundle.counter_snapshot.add_replay_change_count(1);
        bundles.push(replay_bundle);
    }

    Ok(LiveReplayRun {
        final_plan: current,
        bundles,
    })
}

pub fn build_milestone_five_live_artifact(
    suite_name: impl Into<String>,
    canonical_lanes: &[LiveCertificationLane],
    rejection_lanes: &[LiveCertificationRejectionLane],
) -> MilestoneFiveLiveArtifact {
    let suite_name = suite_name.into();
    let mut certification_parts = vec![format!("suite:{suite_name}")];
    let mut coverage_parts = vec![format!("suite:{suite_name}")];
    let mut counter_snapshot = LivePolicyCounters::default();

    for lane in canonical_lanes {
        certification_parts.push(format!("canonical:{}", lane.lane_name()));
        coverage_parts.push(format!("canonical:{}", lane.lane_name()));
        certification_parts.push(format!(
            "report:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            lane.execution().report().query_digest(),
            lane.execution().report().result_digest(),
            lane.execution().report().delivery_digest(),
            lane.execution().report().replay_digest(),
            lane.execution().report().family().as_str(),
            lane.execution().report().outcome_kind(),
            lane.execution().report().outcome_digest(),
            lane.execution().report().basis_digest(),
            lane.execution().report().subscription_digest()
        ));
        certification_parts.push(format!(
            "patch_envelope:{}:{}:{}",
            lane.execution().patch_envelope().delivery_digest(),
            lane.execution().patch_envelope().replay_digest(),
            lane.execution().patch_envelope().basis_digest()
        ));
        certification_parts.extend(lane.execution().counters().digest_parts("canonical"));
        counter_snapshot.absorb(lane.execution().counters());
    }

    for lane in rejection_lanes {
        certification_parts.push(format!("rejection:{}", lane.lane_name()));
        coverage_parts.push(format!("rejection:{}", lane.lane_name()));
        certification_parts.push(format!(
            "failure:{}:{}",
            lane.failure_class(),
            lane.failure_digest()
        ));
        certification_parts.extend(lane.counters().digest_parts("rejection"));
        counter_snapshot.absorb(lane.counters());
    }

    MilestoneFiveLiveArtifact {
        suite_name,
        certification_digest: hash_parts(&certification_parts),
        coverage_digest: hash_parts(&coverage_parts),
        counter_snapshot,
        canonical_lane_count: canonical_lanes.len(),
        rejection_lane_count: rejection_lanes.len(),
    }
}

fn live_execution_report(
    live: &LiveQueryPlan,
    outcome_kind: String,
    outcome_digest: String,
) -> LiveExecutionReport {
    let replay_digest = live.progress_basis().replay_digest().as_str().to_string();
    let result_digest = semantic_result_digest(
        live,
        live.progress_basis()
            .current_basis()
            .proof()
            .digest()
            .as_str(),
        &outcome_kind,
        &outcome_digest,
    );
    LiveExecutionReport {
        query_digest: live.descriptor().query_digest().as_str().to_string(),
        result_digest,
        delivery_digest: outcome_digest.clone(),
        replay_digest,
        family: live.descriptor().family().clone(),
        outcome_kind,
        outcome_digest,
        basis_digest: live
            .progress_basis()
            .current_basis()
            .proof()
            .digest()
            .as_str()
            .to_string(),
        subscription_digest: live.subscription_digest().as_str().to_string(),
    }
}

fn patch_envelope_from_payload(
    live: &LiveQueryPlan,
    payload: LivePatchPayload,
    outcome_kind: String,
    outcome_digest: String,
    basis_digest: String,
    replay_digest: String,
) -> LivePatchEnvelope {
    let result_digest = semantic_result_digest(live, &basis_digest, &outcome_kind, &outcome_digest);

    LivePatchEnvelope {
        query_digest: live.descriptor().query_digest().as_str().to_string(),
        result_digest,
        delivery_digest: outcome_digest,
        replay_digest,
        basis_digest,
        subscription_digest: live.subscription_digest().as_str().to_string(),
        family: live.descriptor().family().clone(),
        payload,
    }
}

fn semantic_result_digest(
    live: &LiveQueryPlan,
    basis_digest: &str,
    outcome_kind: &str,
    outcome_digest: &str,
) -> String {
    hash_parts(&[
        format!("query:{}", live.descriptor().query_digest().as_str()),
        format!("family:{}", live.descriptor().family().as_str()),
        format!("basis:{basis_digest}"),
        format!("outcome_kind:{outcome_kind}"),
        format!("delivery:{outcome_digest}"),
    ])
}

fn replay_bundle_from_patch_envelope(
    patch_envelope: LivePatchEnvelope,
    counter_snapshot: LivePolicyCounters,
) -> LiveReplayBundle {
    LiveReplayBundle {
        query_digest: patch_envelope.query_digest().to_string(),
        result_digest: patch_envelope.result_digest().to_string(),
        delivery_digest: patch_envelope.delivery_digest().to_string(),
        replay_digest: patch_envelope.replay_digest().to_string(),
        basis_digest: patch_envelope.basis_digest().to_string(),
        subscription_digest: patch_envelope.subscription_digest().to_string(),
        counter_snapshot,
        patch_envelope,
    }
}

fn detail_outcome_digest(outcome: &DetailLiveOutcome) -> (String, String) {
    match outcome {
        DetailLiveOutcome::Patch(patch) => {
            ("patch".to_string(), patch.digest().as_str().to_string())
        }
        DetailLiveOutcome::Suppressed(reason) => ("suppressed".to_string(), format!("{reason:?}")),
        DetailLiveOutcome::Refresh(fallback) => (
            "refresh".to_string(),
            format!(
                "{}:{}",
                fallback.admission_class().as_str(),
                fallback.admission_status().as_str()
            ),
        ),
    }
}

fn ordered_collection_outcome_digest(outcome: &OrderedCollectionLiveOutcome) -> (String, String) {
    match outcome {
        OrderedCollectionLiveOutcome::Patch(patch) => {
            ("patch".to_string(), patch.digest().as_str().to_string())
        }
        OrderedCollectionLiveOutcome::Suppressed(reason) => {
            ("suppressed".to_string(), format!("{reason:?}"))
        }
        OrderedCollectionLiveOutcome::Refresh(fallback) => (
            "refresh".to_string(),
            format!(
                "{}:{}",
                fallback.admission_class().as_str(),
                fallback.admission_status().as_str()
            ),
        ),
    }
}

fn bounded_outcome_digest(outcome: &BoundedMaterializationLiveOutcome) -> (String, String) {
    match outcome {
        BoundedMaterializationLiveOutcome::Patch(patch) => {
            ("patch".to_string(), patch.digest().as_str().to_string())
        }
        BoundedMaterializationLiveOutcome::Suppressed(reason) => {
            ("suppressed".to_string(), format!("{reason:?}"))
        }
        BoundedMaterializationLiveOutcome::Refresh(fallback) => (
            "refresh".to_string(),
            format!(
                "{}:{}",
                fallback.admission_class().as_str(),
                fallback.admission_status().as_str()
            ),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detail_preflight_promotes_to_detail_live_plan() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

        assert_eq!(live.descriptor().family(), &LiveQueryFamily::Detail);
        assert_eq!(live.performance_status(), "verified");
        assert_eq!(live.progress_basis().last_ordinal().value(), 0);
    }

    #[test]
    fn collection_with_traversal_promotes_to_bounded_materialization_live_plan() {
        let preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");

        assert_eq!(
            live.descriptor().family(),
            &LiveQueryFamily::BoundedMaterialization
        );
        assert_eq!(live.performance_status(), "debt");
        assert!(live.subscription_digest().as_str().len() > 10);
    }

    #[test]
    fn collection_without_traversal_promotes_to_ordered_collection_live_plan() {
        let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");

        assert_eq!(
            live.descriptor().family(),
            &LiveQueryFamily::OrderedCollection
        );
    }

    #[test]
    fn cdc_collection_preflight_is_rejected_for_live_promotion() {
        let preflight = crate::harness::fixtures::execution_preflights::cdc_collection_preflight();
        let error = promote_preflight_bundle_to_live(&preflight)
            .expect_err("cdc-shaped collection should not admit live promotion");

        assert_eq!(error, LivePromotionError::UnsupportedLiveCollectionFamily);
    }

    #[test]
    fn live_progress_basis_advances_monotonically() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

        let next = live
            .progress_basis()
            .advance(
                live.progress_basis().change_sequence_id(),
                LiveChangeOrdinal(1),
                preflight.basis().clone(),
            )
            .expect("monotonic advance should succeed");

        assert_eq!(next.last_ordinal().value(), 1);
        assert_ne!(
            next.replay_digest().as_str(),
            live.progress_basis().replay_digest().as_str()
        );
    }

    #[test]
    fn live_progress_basis_rejects_non_monotonic_ordinal() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

        let error = live
            .progress_basis()
            .advance(
                live.progress_basis().change_sequence_id(),
                LiveChangeOrdinal(2),
                preflight.basis().clone(),
            )
            .expect_err("ordinal gap should fail");

        assert_eq!(
            error,
            LiveProgressError::ChangeSequenceGap {
                expected: 1,
                received: 2,
            }
        );
    }

    #[test]
    fn detail_live_outcome_emits_patch_for_projected_field_change() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
        let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ));

        let outcome = live
            .detail_live_outcome(&change)
            .expect("projected detail change should produce a patch");

        match outcome {
            DetailLiveOutcome::Patch(patch) => {
                assert_eq!(patch.field_deltas().len(), 1);
                let delta = &patch.field_deltas()[0];
                assert_eq!(delta.field().aspect(), "identity");
                assert_eq!(delta.field().field(), "id");
                assert_eq!(delta.old_value(), Some("user-1"));
                assert_eq!(delta.new_value(), Some("user-2"));
                assert!(!patch.digest().as_str().is_empty());
            }
            DetailLiveOutcome::Refresh(fallback) => {
                panic!("expected patch, got refresh fallback: {fallback:?}");
            }
            DetailLiveOutcome::Suppressed(reason) => {
                panic!("expected patch, got suppression: {reason:?}");
            }
        }
    }

    #[test]
    fn detail_live_outcome_suppresses_irrelevant_change() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
        let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ));

        let outcome = live
            .detail_live_outcome(&change)
            .expect("irrelevant detail change should suppress");

        assert_eq!(
            outcome.suppression_decision(),
            SuppressionDecision::Suppress(SuppressionReason::IrrelevantChange(
                IrrelevantChangeClass::NoProjectedFieldOverlap
            ))
        );
    }

    #[test]
    fn ordered_collection_live_outcome_emits_reorder_patch() {
        let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");
        let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Avery"),
            Some("Zoey"),
        ));

        let outcome = live
            .ordered_collection_live_outcome(&change)
            .expect("ordering-key change should produce a collection patch");

        match outcome {
            OrderedCollectionLiveOutcome::Patch(patch) => {
                match patch.kind() {
                    OrderedCollectionPatchKind::Reordered(ordering) => {
                        assert_eq!(ordering.ordering_field_deltas().len(), 1);
                        assert_eq!(
                            ordering.ordering_field_deltas()[0].field().field(),
                            "display_name"
                        );
                    }
                    other => panic!("expected reorder patch, got {other:?}"),
                }
                assert!(!patch.digest().as_str().is_empty());
            }
            OrderedCollectionLiveOutcome::Refresh(fallback) => {
                panic!("expected patch, got refresh fallback: {fallback:?}");
            }
            OrderedCollectionLiveOutcome::Suppressed(reason) => {
                panic!("expected patch, got suppression: {reason:?}");
            }
        }
    }

    #[test]
    fn ordered_collection_live_outcome_suppresses_irrelevant_relation_change() {
        let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");
        let change =
            BridgeChangeSummary::default().with_relation_delta(BridgeRelationDelta::new("manager"));

        let outcome = live
            .ordered_collection_live_outcome(&change)
            .expect("relation-only change should suppress");

        assert_eq!(
            outcome.suppression_decision(),
            SuppressionDecision::Suppress(SuppressionReason::IrrelevantChange(
                IrrelevantChangeClass::NoProjectedFieldOverlap
            ))
        );
    }

    #[test]
    fn bounded_materialization_live_outcome_emits_scope_patch() {
        let preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");
        let change = BridgeChangeSummary::default()
            .with_relation_delta(BridgeRelationDelta::new("manager"))
            .with_materialization_scope_transition(false, true)
            .with_field_delta(BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Old Manager"),
                Some("New Manager"),
            ));

        let outcome = live
            .bounded_materialization_live_outcome(&change)
            .expect("scope transition should produce a bounded materialization patch");

        match outcome {
            BoundedMaterializationLiveOutcome::Patch(patch) => {
                match patch.kind() {
                    BoundedMaterializationPatchKind::Scope(scope) => {
                        assert_eq!(scope, &MaterializationScopeChange::EnteredScope);
                    }
                    other => panic!("expected scope patch, got {other:?}"),
                }
                assert_eq!(patch.relation_deltas(), &["manager".to_string()]);
                assert_eq!(patch.projected_field_deltas().len(), 1);
            }
            BoundedMaterializationLiveOutcome::Refresh(fallback) => {
                panic!("expected patch, got refresh fallback: {fallback:?}");
            }
            BoundedMaterializationLiveOutcome::Suppressed(reason) => {
                panic!("expected patch, got suppression: {reason:?}");
            }
        }
    }

    #[test]
    fn ordered_collection_live_outcome_suppresses_noop_membership_transition() {
        let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");
        let change = BridgeChangeSummary::default().with_membership_transition(true, true);

        let outcome = live
            .ordered_collection_live_outcome(&change)
            .expect("no-op membership transition should suppress");

        assert_eq!(
            outcome.suppression_decision(),
            SuppressionDecision::Suppress(SuppressionReason::IrrelevantChange(
                IrrelevantChangeClass::NoProjectedFieldOverlap
            ))
        );
    }

    #[test]
    fn noop_membership_transition_is_irrelevant_before_patch_construction() {
        let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");
        let change = BridgeChangeSummary::default().with_membership_transition(true, true);

        let relevance = live.classify_change(&change);

        assert_eq!(
            relevance,
            ChangeRelevance::Irrelevant(IrrelevantChangeClass::NoProjectedFieldOverlap)
        );
    }

    #[test]
    fn detail_width_overflow_is_rejected() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

        let width = live.evaluate_delivery_width(33);

        assert_eq!(width.budget_limit(), 32);
        assert_eq!(width.measured_width(), 33);
        assert_eq!(width.resolution(), &PatchWidthResolution::Reject);
        let counters = LivePolicyCounters::from_width_assessment(&width);
        assert_eq!(counters.live_patch_width_overflow_count(), 1);
        assert_eq!(counters.live_refresh_denial_count(), 0);
    }

    #[test]
    fn ordered_collection_width_overflow_requests_coalescing() {
        let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");

        let width = live.evaluate_delivery_width(65);

        assert_eq!(width.budget_limit(), 64);
        assert_eq!(width.resolution(), &PatchWidthResolution::Coalesce);
    }

    #[test]
    fn bounded_materialization_width_overflow_requests_refresh() {
        let preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");

        let width = live.evaluate_delivery_width(97);

        assert_eq!(width.budget_limit(), 96);
        match width.resolution() {
            PatchWidthResolution::Refresh(fallback) => {
                assert_eq!(
                    fallback.admission_class(),
                    &RefreshAdmissionClass::WidthOverflow
                );
                assert_eq!(
                    fallback.admission_status(),
                    &crate::live_performance::RefreshAdmissionStatus::Debt
                );
            }
            other => panic!("expected refresh resolution, got {other:?}"),
        }
    }

    #[test]
    fn multi_bundle_delivery_requires_admitted_coalescing_class() {
        let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");

        let decision = live
            .request_coalesced_delivery(3)
            .expect("ordered collection should admit basis-stable coalescing");

        assert_eq!(decision, CoalescingDecision::Admitted { bundle_count: 3 });
    }

    #[test]
    fn explicit_refresh_request_is_denied_for_detail_family() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

        let error = live
            .request_refresh_fallback(RefreshAdmissionClass::WidthOverflow)
            .expect_err("detail family should forbid refresh admission");

        assert_eq!(
            error,
            LiveRefreshError::ForbiddenAdmissionClass(RefreshAdmissionClass::WidthOverflow)
        );
    }

    #[test]
    fn rejection_helpers_fail_loudly_when_operation_is_actually_admitted() {
        let ordered_preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
        let ordered_live = promote_preflight_bundle_to_live(&ordered_preflight)
            .expect("ordered collection preflight should promote");
        let bounded_preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
        let bounded_live = promote_preflight_bundle_to_live(&bounded_preflight)
            .expect("bounded materialization preflight should promote");
        let detail_preflight =
            crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let detail_live = promote_preflight_bundle_to_live(&detail_preflight)
            .expect("detail preflight should promote");

        let refresh_error = MilestoneFiveLiveAdapter::refresh_rejection_lane(
            "refresh-should-fail",
            &bounded_live,
            RefreshAdmissionClass::WidthOverflow,
        )
        .expect_err("admitted refresh should not be encoded as a rejection lane");
        let coalescing_error = MilestoneFiveLiveAdapter::coalescing_rejection_lane(
            "coalescing-should-fail",
            &ordered_live,
            3,
        )
        .expect_err("admitted coalescing should not be encoded as a rejection lane");
        let progress_error = MilestoneFiveLiveAdapter::progress_rejection_lane(
            "progress-should-fail",
            &detail_live,
            LiveChangeOrdinal::from_value(1),
            detail_preflight.basis().clone(),
        )
        .expect_err("monotonic progress should not be encoded as a rejection lane");

        assert_eq!(
            refresh_error,
            LiveExpectedRejectionError::UnexpectedRefreshAdmission {
                admission_class: RefreshAdmissionClass::WidthOverflow,
                admission_status: crate::live_performance::RefreshAdmissionStatus::Debt,
            }
        );
        assert_eq!(
            coalescing_error,
            LiveExpectedRejectionError::UnexpectedCoalescingAdmission {
                decision: CoalescingDecision::Admitted { bundle_count: 3 },
            }
        );
        assert!(matches!(
            progress_error,
            LiveExpectedRejectionError::UnexpectedProgressAdvance { ordinal: 1, .. }
        ));
    }

    #[test]
    fn public_live_artifact_builder_summarizes_execution_and_rejection_lanes() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
        let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ));
        let artifact = MilestoneFiveLiveAdapter::artifact(
            "Public Live Artifact Test",
            &[
                MilestoneFiveLiveAdapter::canonical_lane("detail-patch", &live, &change)
                    .expect("canonical lane should build"),
            ],
            &[MilestoneFiveLiveAdapter::refresh_rejection_lane(
                "forbidden-refresh",
                &live,
                RefreshAdmissionClass::WidthOverflow,
            )
            .expect("detail family should reject refresh admission")],
        );

        assert_eq!(artifact.suite_name(), "Public Live Artifact Test");
        assert_eq!(artifact.canonical_lane_count(), 1);
        assert_eq!(artifact.rejection_lane_count(), 1);
        assert!(!artifact.certification_digest().is_empty());
        assert!(!artifact.coverage_digest().is_empty());
        assert_eq!(artifact.counter_snapshot().live_patch_delivery_count(), 1);
        assert_eq!(artifact.counter_snapshot().live_refresh_denial_count(), 1);
    }

    #[test]
    fn public_live_artifact_digest_binds_counter_evidence() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
        let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ));
        let canonical_lane = MilestoneFiveLiveAdapter::detail_patch_lane(&live, &change)
            .expect("detail patch lane should build");
        let low_counter_rejection = LiveCertificationRejectionLane::new(
            "forbidden-refresh",
            "forbidden-refresh-escape-hatch",
            "LiveRefreshError::ForbiddenAdmissionClass(WidthOverflow)",
            LivePolicyCounters::from_refresh_error(&LiveRefreshError::ForbiddenAdmissionClass(
                RefreshAdmissionClass::WidthOverflow,
            )),
        );
        let high_counter_rejection = LiveCertificationRejectionLane::new(
            "forbidden-refresh",
            "forbidden-refresh-escape-hatch",
            "LiveRefreshError::ForbiddenAdmissionClass(WidthOverflow)",
            LivePolicyCounters::from_width_assessment(&PatchWidthAssessment {
                measured_width: 33,
                budget_limit: 32,
                resolution: PatchWidthResolution::Reject,
            }),
        );

        let low_artifact = build_milestone_five_live_artifact(
            "Counter Digest Binding",
            std::slice::from_ref(&canonical_lane),
            std::slice::from_ref(&low_counter_rejection),
        );
        let high_artifact = build_milestone_five_live_artifact(
            "Counter Digest Binding",
            std::slice::from_ref(&canonical_lane),
            std::slice::from_ref(&high_counter_rejection),
        );

        assert_ne!(
            low_artifact.certification_digest(),
            high_artifact.certification_digest()
        );
    }

    #[test]
    fn live_execution_report_emits_milestone_five_digest_fields() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
        let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ));

        let execution = execute_live_change(&live, &change).expect("detail change should execute");

        assert!(!execution.report().query_digest().is_empty());
        assert!(!execution.report().result_digest().is_empty());
        assert!(!execution.report().delivery_digest().is_empty());
        assert!(!execution.report().replay_digest().is_empty());
    }

    #[test]
    fn live_execution_envelope_carries_patch_and_replay_artifacts() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
        let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ));

        let execution = execute_live_change(&live, &change).expect("detail change should execute");

        match execution.patch_envelope().payload() {
            LivePatchPayload::Detail(patch) => {
                assert_eq!(patch.field_deltas().len(), 1);
            }
            other => panic!("expected detail payload, got {other:?}"),
        }
        assert_eq!(
            execution.patch_envelope().delivery_digest(),
            execution.replay_bundle().delivery_digest()
        );
        assert_eq!(
            execution.patch_envelope().replay_digest(),
            execution.replay_bundle().replay_digest()
        );
    }

    #[test]
    fn replay_live_sequence_emits_step_bundles_and_advances_plan() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
        let steps = vec![
            LiveReplayStepInput::new(
                BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
                    "identity",
                    "id",
                    Some("user-1"),
                    Some("user-2"),
                )),
                LiveChangeOrdinal::from_value(1),
                crate::harness::fixtures::resolved_bases::runtime_basis(
                    &crate::harness::fixtures::validated_bundles::runtime_detail_bundle(),
                    "snapshot-2",
                ),
            ),
            LiveReplayStepInput::new(
                BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
                    "identity",
                    "id",
                    Some("user-2"),
                    Some("user-3"),
                )),
                LiveChangeOrdinal::from_value(2),
                crate::harness::fixtures::resolved_bases::runtime_basis(
                    &crate::harness::fixtures::validated_bundles::runtime_detail_bundle(),
                    "snapshot-3",
                ),
            ),
        ];

        let replay = replay_live_sequence(&live, &steps).expect("replay sequence should succeed");

        assert_eq!(replay.bundles().len(), 2);
        assert_eq!(
            replay.final_plan().progress_basis().last_ordinal().value(),
            2
        );
        assert_eq!(
            replay
                .final_plan()
                .progress_basis()
                .replay_digest()
                .as_str(),
            replay.bundles()[1].replay_digest()
        );
    }

    #[test]
    fn standard_named_adapter_helpers_build_expected_lanes() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
        let patch_change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ));

        let patch_lane = MilestoneFiveLiveAdapter::detail_patch_lane(&live, &patch_change)
            .expect("detail patch lane should build");
        let refresh_lane = MilestoneFiveLiveAdapter::forbidden_refresh_rejection_lane(
            &live,
            RefreshAdmissionClass::WidthOverflow,
        )
        .expect("detail family should reject refresh admission");

        assert_eq!(patch_lane.lane_name(), "detail-live-patch-parity");
        assert_eq!(refresh_lane.lane_name(), "forbidden-refresh-escape-hatch");
    }

    #[test]
    fn detail_live_plan_admits_region_scope() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

        let region_plan =
            admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
                .expect("detail live plan should admit region scope");

        assert_eq!(
            region_plan.admission_class(),
            &LocalityAdmissionClass::DetailRegion
        );
        assert!(!region_plan.locality_subscription_digest().is_empty());
        assert_eq!(
            region_plan.locality_cost_posture(),
            &LocalityCostPosture::SingleSliceNarrowing
        );
        assert_eq!(region_plan.locality_breadth_budget().limit(), 1);
        assert_eq!(region_plan.locality_widening_budget().limit(), 0);
        assert_eq!(
            region_plan.stream_lowering_cost_posture(),
            &StreamLoweringCostPosture::SingleDetailCurrentStateMember
        );
        assert_eq!(region_plan.stream_member_width_budget().limit(), 1);
    }

    #[test]
    fn ordered_collection_live_plan_rejects_region_scope() {
        let preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");

        let error =
            admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
                .expect_err("ordered collection should reject region scope in milestone 5.1");

        assert_eq!(error, RegionScopedLiveError::UnsupportedLocalityPredicate);
    }

    #[test]
    fn off_region_change_suppresses_before_delivery() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
        let region_plan =
            admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
                .expect("detail live plan should admit region scope");
        let off_region_change = BridgeChangeSummary::default()
            .with_field_delta(BridgeFieldDelta::new(
                "identity",
                "id",
                Some("user-1"),
                Some("user-2"),
            ))
            .with_region_slice("assembly-b");

        let execution = execute_region_scoped_live_change(&region_plan, &off_region_change)
            .expect("off-region change should suppress, not fail");

        assert_eq!(
            execution.report().locality_outcome(),
            "off_region_suppressed"
        );
        assert_eq!(
            execution.counters().locality_off_region_suppression_count(),
            1
        );
        match execution.patch_envelope().payload() {
            LivePatchPayload::Suppressed(SuppressionReason::OffRegionChange {
                scope_kind,
                scope,
                ..
            }) => {
                assert_eq!(scope_kind, &LocalityScopeKind::Region);
                assert_eq!(scope, "assembly-a");
            }
            other => panic!("expected off-region suppression payload, got {other:?}"),
        }
    }

    #[test]
    fn coarse_fallback_slice_is_a_typed_widening_denial() {
        let preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");
        let partition_plan =
            admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
                .expect("ordered collection should admit partition scope");
        let coarse_change = BridgeChangeSummary::default()
            .with_field_delta(BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Esther"),
                Some("Ess"),
            ))
            .with_coarse_fallback_slice("tenant-a");

        let error = execute_region_scoped_live_change(&partition_plan, &coarse_change)
            .expect_err("coarse fallback should deny widening");

        match error {
            RegionScopedLiveError::WideningDenied { expected, received } => {
                assert!(expected.contains("entity_partition"));
                assert!(!received.is_empty());
            }
            other => panic!("expected widening denial, got {other:?}"),
        }
    }

    #[test]
    fn duplicate_exact_locality_slices_cross_the_breadth_budget() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
        let region_plan =
            admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
                .expect("detail live plan should admit region scope");
        let duplicate_slice_change = BridgeChangeSummary::default()
            .with_field_delta(BridgeFieldDelta::new(
                "identity",
                "id",
                Some("user-1"),
                Some("user-2"),
            ))
            .with_region_slice("assembly-a")
            .with_region_slice("assembly-a");

        let error = execute_region_scoped_live_change(&region_plan, &duplicate_slice_change)
            .expect_err("duplicate exact locality slices should exceed the breadth budget");
        let counters = LivePolicyCounters::from_region_scoped_error(&error);

        assert_eq!(
            error,
            RegionScopedLiveError::LocalityBreadthBudgetExceeded {
                limit: 1,
                actual: 2
            }
        );
        assert_eq!(counters.locality_breadth_budget_cross_count(), 1);
    }

    #[test]
    fn in_region_detail_execution_can_lower_to_stream_contract() {
        let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
        let live =
            promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
        let region_plan =
            admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
                .expect("detail live plan should admit region scope");
        let in_region_change = BridgeChangeSummary::default()
            .with_field_delta(BridgeFieldDelta::new(
                "identity",
                "id",
                Some("user-1"),
                Some("user-2"),
            ))
            .with_region_slice("assembly-a");

        let execution = execute_region_scoped_live_change(&region_plan, &in_region_change)
            .expect("in-region change should execute");
        let contract = lower_region_scoped_execution_to_stream_contract(
            &region_plan,
            &execution,
            StreamConsumerShape::DetailCurrentState,
        )
        .expect("detail execution should lower to stream contract");
        let counters = LivePolicyCounters::from_stream_lowered_delivery(&contract);

        assert_eq!(
            contract.consumer_shape(),
            &StreamConsumerShape::DetailCurrentState
        );
        assert_eq!(contract.member_count(), 1);
        assert_eq!(contract.delivery_width(), 1);
        assert_eq!(
            contract.cost_posture(),
            &StreamLoweringCostPosture::SingleDetailCurrentStateMember
        );
        assert_eq!(counters.stream_contract_admission_count(), 1);
        assert_eq!(counters.stream_lowered_delivery_member_count(), 1);
        assert_eq!(counters.stream_lowered_delivery_width(), 1);
    }

    #[test]
    fn ordered_collection_partition_execution_admits_cdc_collection_stream_shape() {
        let preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");
        let partition_plan =
            admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
                .expect("ordered collection should admit partition scope");
        let in_partition_change = BridgeChangeSummary::default()
            .with_field_delta(BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Esther"),
                Some("Ess"),
            ))
            .with_partition_slice("tenant-a");

        let execution = execute_region_scoped_live_change(&partition_plan, &in_partition_change)
            .expect("in-partition change should execute");
        let contract = lower_region_scoped_execution_to_stream_contract(
            &partition_plan,
            &execution,
            StreamConsumerShape::CdcCollectionPatch,
        )
        .expect("ordered collection execution should lower to cdc collection stream shape");

        assert_eq!(
            contract.consumer_shape(),
            &StreamConsumerShape::CdcCollectionPatch
        );
        assert_eq!(contract.delivery_width(), 2);
        assert_eq!(
            contract.cost_posture(),
            &StreamLoweringCostPosture::CdcPatchWithProjectedDeltas
        );
    }

    #[test]
    fn cdc_stream_shape_rejects_member_width_overflow() {
        let preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
        let live = promote_preflight_bundle_to_live(&preflight)
            .expect("collection preflight should promote");
        let partition_plan =
            admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
                .expect("ordered collection should admit partition scope");
        let wide_partition_change = BridgeChangeSummary::default()
            .with_field_delta(BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Esther"),
                Some("Ess"),
            ))
            .with_field_delta(BridgeFieldDelta::new(
                "identity",
                "id",
                Some("user-1"),
                Some("user-2"),
            ))
            .with_partition_slice("tenant-a");

        let execution = execute_region_scoped_live_change(&partition_plan, &wide_partition_change)
            .expect("in-partition change should execute");
        let error = lower_region_scoped_execution_to_stream_contract(
            &partition_plan,
            &execution,
            StreamConsumerShape::CdcCollectionPatch,
        )
        .expect_err("two projected deltas should overflow the stream member width budget");
        let counters = LivePolicyCounters::from_region_scoped_error(&error);

        assert_eq!(
            error,
            RegionScopedLiveError::StreamMemberWidthBudgetExceeded {
                limit: 2,
                actual: 3
            }
        );
        assert_eq!(counters.stream_contract_denial_count(), 1);
        assert_eq!(counters.stream_member_width_budget_cross_count(), 1);
    }
}
