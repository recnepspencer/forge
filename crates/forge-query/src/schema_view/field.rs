use crate::authoring::{AspectName, FieldName};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SchemaFieldKind {
    String,
    Integer,
    Boolean,
    StructuredContent,
    WorkflowState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaFieldView {
    aspect: AspectName,
    field: FieldName,
    kind: SchemaFieldKind,
    queryable: bool,
    orderable: bool,
    text_predicate_queryable: bool,
    membership_predicate_queryable: bool,
    presence_predicate_queryable: bool,
    workflow_predicate_queryable: bool,
}

impl SchemaFieldView {
    pub fn new(aspect: impl Into<String>, field: impl Into<String>, kind: SchemaFieldKind) -> Self {
        Self {
            aspect: AspectName::new(aspect)
                .expect("schema field aspect must be non-empty at construction"),
            field: FieldName::new(field)
                .expect("schema field name must be non-empty at construction"),
            kind,
            queryable: true,
            orderable: true,
            text_predicate_queryable: false,
            membership_predicate_queryable: false,
            presence_predicate_queryable: false,
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

    pub fn workflow_predicate_queryable(mut self) -> Self {
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

    pub fn aspect(&self) -> &str {
        self.aspect.as_str()
    }

    pub fn field(&self) -> &str {
        self.field.as_str()
    }

    pub fn aspect_name(&self) -> &AspectName {
        &self.aspect
    }

    pub fn field_name(&self) -> &FieldName {
        &self.field
    }

    pub fn kind(&self) -> &SchemaFieldKind {
        &self.kind
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
