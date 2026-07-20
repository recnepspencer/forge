#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CollectionPlanningMode {
    Ordinary,
    Cdc,
    CountRows,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AggregateFunctionFamily {
    NoneAdmittedYet,
    CountRows,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AggregateGroupingShape {
    grouping_key_count: usize,
}

impl AggregateGroupingShape {
    pub fn grouping_key_count(&self) -> usize {
        self.grouping_key_count
    }

    fn new(grouping_key_count: usize) -> Self {
        Self { grouping_key_count }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AggregateInputBreadth(usize);

impl AggregateInputBreadth {
    pub fn value(&self) -> usize {
        self.0
    }

    fn new(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AggregateShapeArtifact {
    function_family: AggregateFunctionFamily,
    grouping_shape: AggregateGroupingShape,
    input_breadth: AggregateInputBreadth,
}

impl AggregateShapeArtifact {
    pub fn function_family(&self) -> &AggregateFunctionFamily {
        &self.function_family
    }

    pub fn grouping_shape(&self) -> &AggregateGroupingShape {
        &self.grouping_shape
    }

    pub fn input_breadth(&self) -> &AggregateInputBreadth {
        &self.input_breadth
    }

    fn ordinary(input_breadth: AggregateInputBreadth) -> Self {
        Self {
            function_family: AggregateFunctionFamily::NoneAdmittedYet,
            grouping_shape: AggregateGroupingShape::new(0),
            input_breadth,
        }
    }

    fn count_rows(input_breadth: AggregateInputBreadth) -> Self {
        Self {
            function_family: AggregateFunctionFamily::CountRows,
            grouping_shape: AggregateGroupingShape::new(0),
            input_breadth,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RollupEdgeClass {
    NoneAdmittedYet,
    RootCollection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RollupShapeArtifact {
    edge_class: RollupEdgeClass,
}

impl RollupShapeArtifact {
    pub fn edge_class(&self) -> &RollupEdgeClass {
        &self.edge_class
    }

    fn none_admitted_yet() -> Self {
        Self {
            edge_class: RollupEdgeClass::NoneAdmittedYet,
        }
    }

    fn root_collection() -> Self {
        Self {
            edge_class: RollupEdgeClass::RootCollection,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DerivedFieldComputationClass {
    NoneAdmittedYet,
    DisplayLabelFromIdentityAndProfile,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DerivedFieldPlanArtifact {
    computation_class: DerivedFieldComputationClass,
    derived_field_count: usize,
}

impl DerivedFieldPlanArtifact {
    pub fn computation_class(&self) -> &DerivedFieldComputationClass {
        &self.computation_class
    }

    pub fn derived_field_count(&self) -> usize {
        self.derived_field_count
    }

    fn none_admitted_yet() -> Self {
        Self {
            computation_class: DerivedFieldComputationClass::NoneAdmittedYet,
            derived_field_count: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CollectionResultFamily {
    OrdinaryCollection,
    CdcCollection,
    CountAggregate,
}

impl CollectionResultFamily {
    pub fn digest_label(&self) -> &'static str {
        match self {
            Self::OrdinaryCollection => "ordinary_collection",
            Self::CdcCollection => "cdc_collection",
            Self::CountAggregate => "count_aggregate",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostReadShapingPlan {
    aggregate_shape: AggregateShapeArtifact,
    rollup_shape: RollupShapeArtifact,
    derived_field_plan: DerivedFieldPlanArtifact,
    result_family: CollectionResultFamily,
}

impl PostReadShapingPlan {
    pub fn aggregate_shape(&self) -> &AggregateShapeArtifact {
        &self.aggregate_shape
    }

    pub fn rollup_shape(&self) -> &RollupShapeArtifact {
        &self.rollup_shape
    }

    pub fn derived_field_plan(&self) -> &DerivedFieldPlanArtifact {
        &self.derived_field_plan
    }

    pub fn result_family(&self) -> &CollectionResultFamily {
        &self.result_family
    }

    pub fn digest_parts(&self) -> Vec<String> {
        vec![
            format!(
                "aggregate_family:{}",
                match self.aggregate_shape.function_family {
                    AggregateFunctionFamily::NoneAdmittedYet => "none_admitted_yet",
                    AggregateFunctionFamily::CountRows => "count_rows",
                }
            ),
            format!(
                "aggregate_grouping_key_count:{}",
                self.aggregate_shape.grouping_shape.grouping_key_count()
            ),
            format!(
                "aggregate_input_breadth:{}",
                self.aggregate_shape.input_breadth.value()
            ),
            format!(
                "rollup_edge_class:{}",
                match self.rollup_shape.edge_class {
                    RollupEdgeClass::NoneAdmittedYet => "none_admitted_yet",
                    RollupEdgeClass::RootCollection => "root_collection",
                }
            ),
            format!(
                "derived_field_class:{}",
                match self.derived_field_plan.computation_class {
                    DerivedFieldComputationClass::NoneAdmittedYet => "none_admitted_yet",
                    DerivedFieldComputationClass::DisplayLabelFromIdentityAndProfile => {
                        "display_label_from_identity_and_profile"
                    }
                }
            ),
            format!(
                "derived_field_count:{}",
                self.derived_field_plan.derived_field_count()
            ),
            format!(
                "collection_result_family:{}",
                self.result_family.digest_label()
            ),
        ]
    }

    pub fn for_mode(input_breadth: usize, mode: &CollectionPlanningMode) -> Self {
        let aggregate_input_breadth = AggregateInputBreadth::new(input_breadth);
        match mode {
            CollectionPlanningMode::Ordinary => Self {
                aggregate_shape: AggregateShapeArtifact::ordinary(aggregate_input_breadth),
                rollup_shape: RollupShapeArtifact::none_admitted_yet(),
                derived_field_plan: DerivedFieldPlanArtifact::none_admitted_yet(),
                result_family: CollectionResultFamily::OrdinaryCollection,
            },
            CollectionPlanningMode::Cdc => Self {
                aggregate_shape: AggregateShapeArtifact::ordinary(aggregate_input_breadth),
                rollup_shape: RollupShapeArtifact::none_admitted_yet(),
                derived_field_plan: DerivedFieldPlanArtifact::none_admitted_yet(),
                result_family: CollectionResultFamily::CdcCollection,
            },
            CollectionPlanningMode::CountRows => Self {
                aggregate_shape: AggregateShapeArtifact::count_rows(aggregate_input_breadth),
                rollup_shape: RollupShapeArtifact::root_collection(),
                derived_field_plan: DerivedFieldPlanArtifact::none_admitted_yet(),
                result_family: CollectionResultFamily::CountAggregate,
            },
        }
    }
}
