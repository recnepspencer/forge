use worth_proof::{
    CanonicalOrder, CanonicalVec, Proof, StructuralProofAuthority, UniqueVec, Uniqueness,
};

use super::{WorthQueryBoundGraphParticipation, WorthQueryBoundRequiredDomain};

pub(super) struct WorthQueryBoundAuthorityShapeProofs {
    _graph_canonical: Proof<CanonicalOrder, StructuralProofAuthority>,
    _graph_unique: Proof<Uniqueness, StructuralProofAuthority>,
    _required_domain_canonical: Proof<CanonicalOrder, StructuralProofAuthority>,
    _required_domain_unique: Proof<Uniqueness, StructuralProofAuthority>,
}

impl WorthQueryBoundAuthorityShapeProofs {
    pub(super) fn admit(
        graphs: &mut [WorthQueryBoundGraphParticipation],
        required_domains: &mut [WorthQueryBoundRequiredDomain],
    ) -> Result<Self, ()> {
        graphs.sort_by(|left, right| left.role.cmp(&right.role));
        required_domains.sort_by(|left, right| left.role.cmp(&right.role));
        let graph_keys = graphs
            .iter()
            .map(|graph| graph.role.clone())
            .collect::<Vec<_>>();
        let required_domain_keys = required_domains
            .iter()
            .map(|domain| domain.role.clone())
            .collect::<Vec<_>>();
        let (_, graph_canonical) = CanonicalVec::try_from_sorted(graph_keys.clone())
            .expect("binding admission sorts graph roles")
            .into_parts();
        let (_, graph_unique) = UniqueVec::try_from_unique(graph_keys)
            .map_err(|_| ())?
            .into_parts();
        let (_, required_domain_canonical) =
            CanonicalVec::try_from_sorted(required_domain_keys.clone())
                .expect("binding admission sorts required-domain roles")
                .into_parts();
        let (_, required_domain_unique) = UniqueVec::try_from_unique(required_domain_keys)
            .map_err(|_| ())?
            .into_parts();
        Ok(Self {
            _graph_canonical: graph_canonical,
            _graph_unique: graph_unique,
            _required_domain_canonical: required_domain_canonical,
            _required_domain_unique: required_domain_unique,
        })
    }
}
