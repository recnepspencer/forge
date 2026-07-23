#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPortableConditionalDependencyLocation {
    Declaration(u32),
    AspectFilter(u32),
    DeltaThreshold,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPortableConditionalDependencyPart {
    GraphReadRole,
    Contract,
    ProjectionMask,
    Binding,
    Locality,
    RelevantChangeWidth,
    RelevantChange(u32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPortableConditionalOutputPart {
    Kind,
    Contract,
    Locality,
    ConsequenceWidth,
    ConsequenceKind(u32),
    ConsequenceTouchGraphRole(u32),
    ConsequenceTouchScope(u32),
    ConsequenceEffectFamily(u32),
    ProjectionRole,
    WorkflowValueContract,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPortableConditionalDimension {
    Identity,
    Role,
    DependencyWidth,
    Dependency {
        location: WorthQueryPortableConditionalDependencyLocation,
        part: WorthQueryPortableConditionalDependencyPart,
    },
    OutputWidth,
    Output {
        index: u32,
        part: WorthQueryPortableConditionalOutputPart,
    },
    RequiredContextWidth,
    RequiredContext(u32),
    ConditionClass,
    ConditionDependencyWidth,
    DeltaThresholdValue,
    DeltaThresholdUnit,
    DeltaThresholdValueFamily,
    DeltaThresholdComparisonDomain,
    DeltaThresholdBoundary,
    TemporalCondition,
    DomainConditionFamily,
    DomainConditionParameterWidth,
    DomainConditionParameterName(u32),
    DomainConditionParameterValue(u32),
    Trigger,
    DependencyComparator,
    OutputEquivalence,
    ArtifactReuseEquivalence,
    Maintenance,
    ArtifactPosture,
    OutputRelationship,
}
