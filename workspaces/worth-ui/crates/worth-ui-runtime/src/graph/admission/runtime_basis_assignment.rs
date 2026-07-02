use std::collections::BTreeMap;

use crate::declaration::UiDeclarationGraphHandoff;
use crate::graph::{
    UiGraphInstantiationDenial, UiRepeatedInstanceBasis, UiRepeatedInstanceBasisDenial,
    UiRuntimeInstanceBasisAdmission,
};

pub(super) struct UiRuntimeBasisAssignments {
    bases_by_declaration: BTreeMap<u64, Vec<UiRepeatedInstanceBasis>>,
}

impl UiRuntimeBasisAssignments {
    pub(super) fn resolve(
        handoffs: &[UiDeclarationGraphHandoff],
        runtime_basis_admissions: &[UiRuntimeInstanceBasisAdmission],
    ) -> Result<Self, UiGraphInstantiationDenial> {
        let handoff_counts = handoff_counts(handoffs);
        let runtime_basis_groups =
            runtime_basis_groups(runtime_basis_admissions, &handoff_counts)?;
        let mut bases_by_declaration = BTreeMap::new();

        for handoff in handoffs {
            let declaration_identity = handoff.identity().clone();
            let declaration_digest = declaration_identity.digest().raw();
            if bases_by_declaration.contains_key(&declaration_digest) {
                continue;
            }

            let matching_handoffs = handoff_counts.get(&declaration_digest).copied().unwrap_or(0);
            let repeated_instance_bases = match runtime_basis_groups.get(&declaration_digest) {
                Some(admissions) => assign_runtime_bases(
                    declaration_identity,
                    admissions,
                    matching_handoffs,
                )?,
                None if matching_handoffs > 1 => {
                    vec![UiRepeatedInstanceBasis::denied(
                        UiRepeatedInstanceBasisDenial::BasisFreeRuntimeIdentityDenied,
                    ); matching_handoffs]
                }
                None => vec![UiRepeatedInstanceBasis::declaration_keyed(handoff.identity().digest())],
            };
            bases_by_declaration.insert(declaration_digest, repeated_instance_bases);
        }

        Ok(Self { bases_by_declaration })
    }

    pub(super) fn basis_for(
        &self,
        declaration_digest: u64,
        occurrence_index: usize,
    ) -> Option<&UiRepeatedInstanceBasis> {
        self.bases_by_declaration
            .get(&declaration_digest)
            .and_then(|bases| bases.get(occurrence_index))
    }
}

fn assign_runtime_bases(
    declaration_identity: crate::declaration::UiDeclarationIdentity,
    admissions: &[UiRuntimeInstanceBasisAdmission],
    matching_handoffs: usize,
) -> Result<Vec<UiRepeatedInstanceBasis>, UiGraphInstantiationDenial> {
    if admissions.len() != matching_handoffs {
        return Err(UiGraphInstantiationDenial::ContradictoryRuntimeBasisAdmission {
            declaration_identity,
        });
    }

    admissions
        .iter()
        .map(|admission| {
            UiRepeatedInstanceBasis::runtime_data_keyed(admission.runtime_data_key().clone())
                .map_err(|denial| UiGraphInstantiationDenial::RuntimeBasisDenied {
                    declaration_identity: declaration_identity.clone(),
                    denial,
                })
        })
        .collect()
}

fn runtime_basis_groups(
    runtime_basis_admissions: &[UiRuntimeInstanceBasisAdmission],
    handoff_counts: &BTreeMap<u64, usize>,
) -> Result<BTreeMap<u64, Vec<UiRuntimeInstanceBasisAdmission>>, UiGraphInstantiationDenial> {
    let mut basis_by_declaration = BTreeMap::new();

    for admission in runtime_basis_admissions {
        let declaration_identity = admission.declaration_identity().clone();
        let declaration_digest = declaration_identity.digest().raw();
        if !handoff_counts.contains_key(&declaration_digest) {
            return Err(UiGraphInstantiationDenial::RuntimeBasisTargetsUnknownDeclaration {
                declaration_identity,
            });
        }
        basis_by_declaration
            .entry(declaration_digest)
            .or_insert_with(Vec::new)
            .push(admission.clone());
    }

    for admissions in basis_by_declaration.values_mut() {
        admissions.sort_by(|left, right| {
            left.runtime_data_key()
                .as_str()
                .cmp(right.runtime_data_key().as_str())
        });
        if admissions.windows(2).any(|window| {
            window[0].runtime_data_key().as_str() == window[1].runtime_data_key().as_str()
        }) {
            return Err(UiGraphInstantiationDenial::RuntimeBasisDenied {
                declaration_identity: admissions[0].declaration_identity().clone(),
                denial: UiRepeatedInstanceBasisDenial::ContradictoryBasis,
            });
        }
    }

    Ok(basis_by_declaration)
}

fn handoff_counts(handoffs: &[UiDeclarationGraphHandoff]) -> BTreeMap<u64, usize> {
    let mut counts = BTreeMap::new();
    for handoff in handoffs {
        *counts.entry(handoff.identity().digest().raw()).or_insert(0) += 1;
    }
    counts
}
