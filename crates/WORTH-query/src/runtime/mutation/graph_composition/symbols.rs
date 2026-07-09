use crate::runtime::WorthQuerySymbolicTargetReference;
use worth_relational::facade::identity::KindId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphEntitySymbol {
    reference: WorthQuerySymbolicTargetReference,
}

impl WorthQueryGraphEntitySymbol {
    pub fn symbol(&self) -> &str {
        self.reference.symbol()
    }

    pub(crate) fn new(reference: WorthQuerySymbolicTargetReference) -> Self {
        Self { reference }
    }

    pub(crate) fn reference(&self) -> WorthQuerySymbolicTargetReference {
        self.reference.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphRelationSymbol {
    reference: WorthQuerySymbolicTargetReference,
    relation_kind_id: Option<KindId>,
}

impl WorthQueryGraphRelationSymbol {
    pub fn symbol(&self) -> &str {
        self.reference.symbol()
    }

    pub fn relation_kind_id(&self) -> Option<KindId> {
        self.relation_kind_id
    }

    pub(crate) fn new(
        reference: WorthQuerySymbolicTargetReference,
        relation_kind_id: Option<KindId>,
    ) -> Self {
        Self {
            reference,
            relation_kind_id,
        }
    }

    pub(crate) fn reference(&self) -> WorthQuerySymbolicTargetReference {
        self.reference.clone()
    }
}
