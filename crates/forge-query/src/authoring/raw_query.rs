use std::collections::BTreeSet;

use super::{
    AspectFieldKey, AspectFieldSelector, CollectionQueryBuilder, DetailQueryBuilder,
    OrderingSelector, PredicateSelector, TraversalSelector,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QueryFamily {
    Detail,
    Collection,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) enum InternalQueryFamily {
    Detail,
    Collection,
    #[cfg(test)]
    Unsupported(&'static str),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RootEntityKey(String);

impl RootEntityKey {
    pub fn new(key: impl Into<String>) -> Result<Self, super::AuthoringError> {
        let key = key.into();
        if key.trim().is_empty() {
            return Err(super::AuthoringError::EmptyRootEntityKey);
        }
        Ok(Self(key))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawAuthoredQuery {
    family: InternalQueryFamily,
    root: RootEntityKey,
    projection: Vec<AspectFieldSelector>,
    predicates: Vec<PredicateSelector>,
    ordering: Vec<OrderingSelector>,
    traversal: Vec<TraversalSelector>,
}

impl RawAuthoredQuery {
    pub fn detail_builder(root: RootEntityKey) -> DetailQueryBuilder {
        DetailQueryBuilder::new(root)
    }

    pub fn collection_builder(root: RootEntityKey) -> CollectionQueryBuilder {
        CollectionQueryBuilder::new(root)
    }

    pub(crate) fn detail(root: RootEntityKey) -> Self {
        Self {
            family: InternalQueryFamily::Detail,
            root,
            projection: Vec::new(),
            predicates: Vec::new(),
            ordering: Vec::new(),
            traversal: Vec::new(),
        }
    }

    pub(crate) fn collection(root: RootEntityKey) -> Self {
        Self {
            family: InternalQueryFamily::Collection,
            root,
            projection: Vec::new(),
            predicates: Vec::new(),
            ordering: Vec::new(),
            traversal: Vec::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn unsupported_for_test(root: RootEntityKey, family: &'static str) -> Self {
        Self {
            family: InternalQueryFamily::Unsupported(family),
            root,
            projection: Vec::new(),
            predicates: Vec::new(),
            ordering: Vec::new(),
            traversal: Vec::new(),
        }
    }

    pub(crate) fn with_projection(mut self, entry: AspectFieldSelector) -> Self {
        self.projection.push(entry);
        self
    }

    pub(crate) fn with_predicate(mut self, predicate: PredicateSelector) -> Self {
        self.predicates.push(predicate);
        self
    }

    pub(crate) fn with_ordering(mut self, entry: OrderingSelector) -> Self {
        self.ordering.push(entry);
        self
    }

    pub(crate) fn with_traversal(mut self, entry: TraversalSelector) -> Self {
        self.traversal.push(entry);
        self
    }

    pub fn family(&self) -> QueryFamily {
        match self.family {
            InternalQueryFamily::Detail => QueryFamily::Detail,
            InternalQueryFamily::Collection => QueryFamily::Collection,
            #[cfg(test)]
            InternalQueryFamily::Unsupported(_) => {
                panic!("unsupported internal query family must not cross the public boundary")
            }
        }
    }

    pub(crate) fn internal_family(&self) -> &InternalQueryFamily {
        &self.family
    }

    pub fn root(&self) -> &RootEntityKey {
        &self.root
    }

    pub fn projection(&self) -> &[AspectFieldSelector] {
        &self.projection
    }

    pub fn traversal(&self) -> &[TraversalSelector] {
        &self.traversal
    }

    pub fn predicates(&self) -> &[PredicateSelector] {
        &self.predicates
    }

    pub fn ordering(&self) -> &[OrderingSelector] {
        &self.ordering
    }

    pub fn projection_field_set(&self) -> BTreeSet<AspectFieldKey> {
        self.projection
            .iter()
            .map(|entry| {
                AspectFieldKey::from_parts(entry.aspect_name().clone(), entry.field_name().clone())
            })
            .collect()
    }
}
