use super::{AuthoredResultShapeField, CollectionResultShapeBuilder, DetailResultShapeBuilder};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ResultShapeFamily {
    Detail,
    Collection,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InternalResultShapeFamily {
    Detail,
    Collection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawAuthoredResultShape {
    family: InternalResultShapeFamily,
    fields: Vec<AuthoredResultShapeField>,
}

impl RawAuthoredResultShape {
    pub fn detail_builder() -> DetailResultShapeBuilder {
        DetailResultShapeBuilder::new()
    }

    pub fn collection_builder() -> CollectionResultShapeBuilder {
        CollectionResultShapeBuilder::new()
    }

    pub fn detail() -> Self {
        Self {
            family: InternalResultShapeFamily::Detail,
            fields: Vec::new(),
        }
    }

    pub fn collection() -> Self {
        Self {
            family: InternalResultShapeFamily::Collection,
            fields: Vec::new(),
        }
    }

    pub fn with_field(mut self, field: AuthoredResultShapeField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn family(&self) -> ResultShapeFamily {
        match self.family {
            InternalResultShapeFamily::Detail => ResultShapeFamily::Detail,
            InternalResultShapeFamily::Collection => ResultShapeFamily::Collection,
        }
    }

    pub fn fields(&self) -> &[AuthoredResultShapeField] {
        &self.fields
    }
}
