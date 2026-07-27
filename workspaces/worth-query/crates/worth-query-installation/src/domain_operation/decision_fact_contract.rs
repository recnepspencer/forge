#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryDecisionFactKind {
    ObservedValue,
    AbsenceOrNonMembership,
    PredicateOrComparison,
    OrderingOrExtremum,
    CardinalityUniquenessOrOwnership,
    TraversalFrontierOrPath,
    AccessProductCoverageOrMembership,
    ArtifactSemanticProjection,
    DomainStructuralProof,
}

impl WorthQueryDecisionFactKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ObservedValue => "observed-value",
            Self::AbsenceOrNonMembership => "absence-or-non-membership",
            Self::PredicateOrComparison => "predicate-or-comparison",
            Self::OrderingOrExtremum => "ordering-or-extremum",
            Self::CardinalityUniquenessOrOwnership => "cardinality-uniqueness-or-ownership",
            Self::TraversalFrontierOrPath => "traversal-frontier-or-path",
            Self::AccessProductCoverageOrMembership => "access-product-coverage-or-membership",
            Self::ArtifactSemanticProjection => "artifact-semantic-projection",
            Self::DomainStructuralProof => "domain-structural-proof",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryDecisionFactFamily {
    identity: String,
    kind: WorthQueryDecisionFactKind,
    exact_fact_count: usize,
}

impl WorthQueryDecisionFactFamily {
    pub fn new(
        identity: impl Into<String>,
        kind: WorthQueryDecisionFactKind,
    ) -> Result<Self, &'static str> {
        let identity = identity.into();
        if identity.trim().is_empty() || identity.trim() != identity {
            return Err("invalid-decision-fact-family-identity");
        }
        Ok(Self {
            identity,
            kind,
            exact_fact_count: 1,
        })
    }

    pub fn with_exact_fact_count(mut self, exact_fact_count: usize) -> Result<Self, &'static str> {
        if exact_fact_count == 0 {
            return Err("invalid-decision-fact-family-cardinality");
        }
        self.exact_fact_count = exact_fact_count;
        Ok(self)
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn kind(&self) -> &WorthQueryDecisionFactKind {
        &self.kind
    }

    pub fn exact_fact_count(&self) -> usize {
        self.exact_fact_count
    }

    pub(crate) fn canonical_token(&self) -> String {
        format!(
            "{}:{}:{}",
            self.kind.as_str(),
            self.identity,
            self.exact_fact_count
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationDecisionFactContract {
    NotRequired,
    Declared {
        required_families: Vec<WorthQueryDecisionFactFamily>,
    },
}

impl WorthQueryOperationDecisionFactContract {
    pub fn declared(
        required_families: impl IntoIterator<Item = WorthQueryDecisionFactFamily>,
    ) -> Result<Self, &'static str> {
        let mut required_families = required_families.into_iter().collect::<Vec<_>>();
        required_families.sort();
        if required_families.is_empty() {
            return Err("empty-decision-fact-family-contract");
        }
        if required_families
            .windows(2)
            .any(|pair| pair[0].identity() == pair[1].identity())
        {
            return Err("duplicate-decision-fact-family");
        }
        Ok(Self::Declared { required_families })
    }

    pub fn required_families(&self) -> &[WorthQueryDecisionFactFamily] {
        match self {
            Self::NotRequired => &[],
            Self::Declared { required_families } => required_families,
        }
    }

    pub fn family(&self, identity: &str) -> Option<&WorthQueryDecisionFactFamily> {
        self.required_families()
            .iter()
            .find(|family| family.identity() == identity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declared_families_are_canonical_and_queryable_by_identity() {
        let contract = WorthQueryOperationDecisionFactContract::declared([
            family(
                "z-family",
                WorthQueryDecisionFactKind::DomainStructuralProof,
            ),
            family("a-family", WorthQueryDecisionFactKind::ObservedValue),
        ])
        .expect("distinct decision families should install");
        assert_eq!(contract.required_families()[0].identity(), "a-family");
        assert_eq!(
            contract.family("z-family").unwrap().kind(),
            &WorthQueryDecisionFactKind::DomainStructuralProof
        );
    }

    #[test]
    fn empty_duplicate_and_noncanonical_contracts_are_rejected() {
        assert!(WorthQueryOperationDecisionFactContract::declared([]).is_err());
        let duplicate = family("same", WorthQueryDecisionFactKind::ObservedValue);
        assert!(
            WorthQueryOperationDecisionFactContract::declared([duplicate.clone(), duplicate])
                .is_err()
        );
        assert!(WorthQueryOperationDecisionFactContract::declared([
            family("same-identity", WorthQueryDecisionFactKind::ObservedValue),
            family(
                "same-identity",
                WorthQueryDecisionFactKind::DomainStructuralProof,
            ),
        ])
        .is_err());
        assert!(WorthQueryDecisionFactFamily::new(
            " padded ",
            WorthQueryDecisionFactKind::ObservedValue
        )
        .is_err());
        assert!(family("counted", WorthQueryDecisionFactKind::ObservedValue)
            .with_exact_fact_count(0)
            .is_err());
    }

    fn family(identity: &str, kind: WorthQueryDecisionFactKind) -> WorthQueryDecisionFactFamily {
        WorthQueryDecisionFactFamily::new(identity, kind).unwrap()
    }
}
