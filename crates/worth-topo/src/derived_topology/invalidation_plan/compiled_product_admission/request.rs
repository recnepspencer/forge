use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::compiled_product_family::TopologyCompiledProductConsumer;
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum TopologyCompiledProductAdmissionRequest<'a> {
    ReadBasis {
        consumer: TopologyCompiledProductConsumer,
        read_basis: &'a DerivedTopologyReadBasis,
    },
    SelectedPlan {
        consumer: TopologyCompiledProductConsumer,
        read_basis: &'a DerivedTopologyReadBasis,
        touched_closure: &'a DerivedInvalidationTouchedClosure,
        selected_plan: &'a DerivedInvalidationSelectedPlan,
    },
}

impl<'a> TopologyCompiledProductAdmissionRequest<'a> {
    pub(crate) const fn for_historical_read_basis(
        consumer: TopologyCompiledProductConsumer,
        read_basis: &'a DerivedTopologyReadBasis,
    ) -> Self {
        Self::ReadBasis {
            consumer,
            read_basis,
        }
    }

    pub(crate) const fn for_selected_plan(
        consumer: TopologyCompiledProductConsumer,
        read_basis: &'a DerivedTopologyReadBasis,
        touched_closure: &'a DerivedInvalidationTouchedClosure,
        selected_plan: &'a DerivedInvalidationSelectedPlan,
    ) -> Self {
        Self::SelectedPlan {
            consumer,
            read_basis,
            touched_closure,
            selected_plan,
        }
    }

    pub(crate) const fn consumer(self) -> TopologyCompiledProductConsumer {
        match self {
            Self::ReadBasis { consumer, .. } | Self::SelectedPlan { consumer, .. } => consumer,
        }
    }

    pub(crate) const fn read_basis(self) -> &'a DerivedTopologyReadBasis {
        match self {
            Self::ReadBasis { read_basis, .. } | Self::SelectedPlan { read_basis, .. } => {
                read_basis
            }
        }
    }

    pub(crate) const fn selected_plan(self) -> Option<&'a DerivedInvalidationSelectedPlan> {
        match self {
            Self::ReadBasis { .. } => None,
            Self::SelectedPlan { selected_plan, .. } => Some(selected_plan),
        }
    }

    pub(crate) const fn touched_closure(self) -> Option<&'a DerivedInvalidationTouchedClosure> {
        match self {
            Self::ReadBasis { .. } => None,
            Self::SelectedPlan {
                touched_closure, ..
            } => Some(touched_closure),
        }
    }
}
