use crate::authoring::AspectFieldKey;
use crate::view_shape::GroupedViewPlanningArtifact;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyInfluencePurpose {
    Grouping,
    DerivedResultField,
    TemplatePredicate,
    Aggregation,
    Cursor,
    ViewMembership,
}

impl PolicyInfluencePurpose {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Grouping => "grouping",
            Self::DerivedResultField => "derived_result_field",
            Self::TemplatePredicate => "template_predicate",
            Self::Aggregation => "aggregation",
            Self::Cursor => "cursor",
            Self::ViewMembership => "view_membership",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyInfluenceEntry {
    purpose: PolicyInfluencePurpose,
    field: AspectFieldKey,
}

impl PolicyInfluenceEntry {
    pub(crate) fn new(purpose: PolicyInfluencePurpose, field: AspectFieldKey) -> Self {
        Self { purpose, field }
    }

    pub fn purpose(&self) -> PolicyInfluencePurpose {
        self.purpose
    }

    pub fn field(&self) -> &AspectFieldKey {
        &self.field
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "{}:{}.{}",
            self.purpose.as_str(),
            self.field.aspect().as_str(),
            self.field.field().as_str()
        )
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PolicyInfluenceSet {
    entries: Vec<PolicyInfluenceEntry>,
}

impl PolicyInfluenceSet {
    pub fn none() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn with_grouping_field(mut self, field: AspectFieldKey) -> Self {
        self.entries.push(PolicyInfluenceEntry::new(
            PolicyInfluencePurpose::Grouping,
            field,
        ));
        self
    }

    pub fn with_derived_result_field(mut self, field: AspectFieldKey) -> Self {
        self.entries.push(PolicyInfluenceEntry::new(
            PolicyInfluencePurpose::DerivedResultField,
            field,
        ));
        self
    }

    pub fn with_template_predicate_field(mut self, field: AspectFieldKey) -> Self {
        self.entries.push(PolicyInfluenceEntry::new(
            PolicyInfluencePurpose::TemplatePredicate,
            field,
        ));
        self
    }

    pub fn with_aggregation_field(mut self, field: AspectFieldKey) -> Self {
        self.entries.push(PolicyInfluenceEntry::new(
            PolicyInfluencePurpose::Aggregation,
            field,
        ));
        self
    }

    pub fn with_cursor_field(mut self, field: AspectFieldKey) -> Self {
        self.entries.push(PolicyInfluenceEntry::new(
            PolicyInfluencePurpose::Cursor,
            field,
        ));
        self
    }

    pub fn with_view_membership_field(mut self, field: AspectFieldKey) -> Self {
        self.entries.push(PolicyInfluenceEntry::new(
            PolicyInfluencePurpose::ViewMembership,
            field,
        ));
        self
    }

    pub fn from_grouped_view_planning(
        grouped: &GroupedViewPlanningArtifact,
    ) -> Result<Self, crate::authoring::AuthoringError> {
        Ok(Self::none().with_grouping_field(grouped.grouping_binding().source_field_key().clone()))
    }

    pub fn entries(&self) -> &[PolicyInfluenceEntry] {
        &self.entries
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec!["policy_influence_set".to_string()];
        parts.extend(self.entries.iter().map(PolicyInfluenceEntry::digest_part));
        parts
    }
}
