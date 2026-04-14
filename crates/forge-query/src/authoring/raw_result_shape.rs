use super::{AuthoredResultShapeField, CollectionResultShapeBuilder, DetailResultShapeBuilder};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ResultShapeFamily {
    Detail,
    Collection,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) enum InternalResultShapeFamily {
    Detail,
    Collection,
    #[cfg(test)]
    Unsupported(&'static str),
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

    pub(crate) fn detail() -> Self {
        Self {
            family: InternalResultShapeFamily::Detail,
            fields: Vec::new(),
        }
    }

    pub(crate) fn collection() -> Self {
        Self {
            family: InternalResultShapeFamily::Collection,
            fields: Vec::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn unsupported_for_test(family: &'static str) -> Self {
        Self {
            family: InternalResultShapeFamily::Unsupported(family),
            fields: Vec::new(),
        }
    }

    pub(crate) fn with_field(mut self, field: AuthoredResultShapeField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn family(&self) -> ResultShapeFamily {
        match self.family {
            InternalResultShapeFamily::Detail => ResultShapeFamily::Detail,
            InternalResultShapeFamily::Collection => ResultShapeFamily::Collection,
            #[cfg(test)]
            InternalResultShapeFamily::Unsupported(_) => {
                panic!(
                    "unsupported internal result-shape family must not cross the public boundary"
                )
            }
        }
    }

    pub(crate) fn internal_family(&self) -> &InternalResultShapeFamily {
        &self.family
    }

    pub fn fields(&self) -> &[AuthoredResultShapeField] {
        &self.fields
    }
}
