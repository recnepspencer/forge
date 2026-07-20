use crate::authoring::{AspectName, FieldName};
use worth_foundational::facade::ScalarAspectType;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaFieldView {
    aspect: AspectName,
    field: FieldName,
    kind: ScalarAspectType,
    queryable: bool,
    orderable: bool,
    text_predicate_queryable: bool,
    membership_predicate_queryable: bool,
    presence_predicate_queryable: bool,
    workflow_semantic: bool,
    workflow_predicate_queryable: bool,
}

impl SchemaFieldView {
    pub fn new(aspect: AspectName, field: FieldName, kind: ScalarAspectType) -> Self {
        Self {
            aspect,
            field,
            kind,
            queryable: true,
            orderable: true,
            text_predicate_queryable: false,
            membership_predicate_queryable: false,
            presence_predicate_queryable: false,
            workflow_semantic: false,
            workflow_predicate_queryable: false,
        }
    }

    pub fn non_queryable(mut self) -> Self {
        self.queryable = false;
        self.orderable = false;
        self
    }

    pub fn ordering_only(mut self) -> Self {
        self.queryable = false;
        self.orderable = true;
        self
    }

    pub fn non_orderable(mut self) -> Self {
        self.orderable = false;
        self
    }

    pub fn workflow_semantic(mut self) -> Self {
        self.workflow_semantic = true;
        self
    }

    pub fn workflow_predicate_queryable(mut self) -> Self {
        self.workflow_semantic = true;
        self.workflow_predicate_queryable = true;
        self
    }

    pub fn text_predicate_queryable(mut self) -> Self {
        self.text_predicate_queryable = true;
        self
    }

    pub fn membership_predicate_queryable(mut self) -> Self {
        self.membership_predicate_queryable = true;
        self
    }

    pub fn presence_predicate_queryable(mut self) -> Self {
        self.presence_predicate_queryable = true;
        self
    }

    pub fn aspect_name(&self) -> &AspectName {
        &self.aspect
    }

    pub fn field_name(&self) -> &FieldName {
        &self.field
    }

    pub fn kind(&self) -> &ScalarAspectType {
        &self.kind
    }

    pub fn native_family(&self) -> ScalarAspectType {
        self.kind
    }

    pub fn is_queryable(&self) -> bool {
        self.queryable
    }

    pub fn is_orderable(&self) -> bool {
        self.orderable
    }

    pub fn is_workflow_predicate_queryable(&self) -> bool {
        self.workflow_predicate_queryable
    }

    pub fn is_workflow_semantic(&self) -> bool {
        self.workflow_semantic
    }

    pub fn is_text_predicate_queryable(&self) -> bool {
        self.text_predicate_queryable
    }

    pub fn is_membership_predicate_queryable(&self) -> bool {
        self.membership_predicate_queryable
    }

    pub fn is_presence_predicate_queryable(&self) -> bool {
        self.presence_predicate_queryable
    }
}
