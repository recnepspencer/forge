use std::collections::BTreeSet;

use super::{
    AspectFieldKey, AspectFieldSelector, CollectionQueryBuilder, DetailQueryBuilder,
    OrderingSelector, PredicateSelector, TraversalSelector,
    WorthQueryGraphReadDomainOperationDeclaration,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QueryFamily {
    Detail,
    Collection,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InternalQueryFamily {
    Detail,
    Collection,
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
    domain_graph_operations: Vec<WorthQueryGraphReadDomainOperationDeclaration>,
}

impl RawAuthoredQuery {
    pub fn detail_builder(root: RootEntityKey) -> DetailQueryBuilder {
        DetailQueryBuilder::new(root)
    }

    pub fn collection_builder(root: RootEntityKey) -> CollectionQueryBuilder {
        CollectionQueryBuilder::new(root)
    }

    pub fn detail(root: RootEntityKey) -> Self {
        Self {
            family: InternalQueryFamily::Detail,
            root,
            projection: Vec::new(),
            predicates: Vec::new(),
            ordering: Vec::new(),
            traversal: Vec::new(),
            domain_graph_operations: Vec::new(),
        }
    }

    pub fn collection(root: RootEntityKey) -> Self {
        Self {
            family: InternalQueryFamily::Collection,
            root,
            projection: Vec::new(),
            predicates: Vec::new(),
            ordering: Vec::new(),
            traversal: Vec::new(),
            domain_graph_operations: Vec::new(),
        }
    }

    pub fn with_projection(mut self, entry: AspectFieldSelector) -> Self {
        self.projection.push(entry);
        self
    }

    pub fn with_predicate(mut self, predicate: PredicateSelector) -> Self {
        self.predicates.push(predicate);
        self
    }

    pub fn with_ordering(mut self, entry: OrderingSelector) -> Self {
        self.ordering.push(entry);
        self
    }

    pub fn with_traversal(mut self, entry: TraversalSelector) -> Self {
        self.traversal.push(entry);
        self
    }

    pub fn with_domain_graph_operation(
        mut self,
        operation: WorthQueryGraphReadDomainOperationDeclaration,
    ) -> Self {
        self.domain_graph_operations.push(operation);
        self.domain_graph_operations
            .sort_by_key(WorthQueryGraphReadDomainOperationDeclaration::digest_part);
        self.domain_graph_operations.dedup();
        self
    }

    pub fn family(&self) -> QueryFamily {
        match self.family {
            InternalQueryFamily::Detail => QueryFamily::Detail,
            InternalQueryFamily::Collection => QueryFamily::Collection,
        }
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

    pub fn domain_graph_operations(&self) -> &[WorthQueryGraphReadDomainOperationDeclaration] {
        &self.domain_graph_operations
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
