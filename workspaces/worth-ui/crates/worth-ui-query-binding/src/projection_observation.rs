#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub enum UiProjectionObservation {
    Scalar(UiScalarProjectionObservation),
    Collection(UiCollectionProjectionObservation),
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub struct UiScalarProjectionObservation {
    fact: crate::UiScalarProjectionFactReceipt,
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionObservation {
    fact: crate::UiCollectionProjectionFactReceipt,
}

impl UiProjectionObservation {
    pub fn projection_identity(&self) -> &crate::WorthUiQueryViewIdentity {
        match self {
            Self::Scalar(observation) => observation.projection_identity(),
            Self::Collection(observation) => observation.projection_identity(),
        }
    }

    pub fn owner_order(&self) -> u64 {
        match self {
            Self::Scalar(observation) => observation.owner_order(),
            Self::Collection(observation) => observation.owner_order(),
        }
    }

    pub fn retained_bytes(&self) -> usize {
        match self {
            Self::Scalar(_) => std::mem::size_of::<UiScalarProjectionObservation>(),
            Self::Collection(observation) => {
                std::mem::size_of::<UiCollectionProjectionObservation>()
                    + observation.fact.changes().len()
                        * std::mem::size_of::<crate::UiCollectionProjectionChange>()
            }
        }
    }
}

impl UiScalarProjectionObservation {
    pub(crate) fn query_issued(fact: crate::UiScalarProjectionFactReceipt) -> Self {
        Self { fact }
    }

    pub fn projection_identity(&self) -> &crate::WorthUiQueryViewIdentity {
        self.fact.core().projection_identity()
    }

    pub fn owner_order(&self) -> u64 {
        self.fact.core().observation_order()
    }

    pub fn fact(&self) -> &crate::UiScalarProjectionFactReceipt {
        &self.fact
    }

    pub fn into_fact(self) -> crate::UiScalarProjectionFactReceipt {
        self.fact
    }
}

impl UiCollectionProjectionObservation {
    pub(crate) fn query_issued(fact: crate::UiCollectionProjectionFactReceipt) -> Self {
        Self { fact }
    }

    pub fn projection_identity(&self) -> &crate::WorthUiQueryViewIdentity {
        self.fact.core().projection_identity()
    }

    pub fn owner_order(&self) -> u64 {
        self.fact.core().observation_order()
    }

    pub fn fact(&self) -> &crate::UiCollectionProjectionFactReceipt {
        &self.fact
    }

    pub fn into_fact(self) -> crate::UiCollectionProjectionFactReceipt {
        self.fact
    }
}
