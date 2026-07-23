use std::collections::BTreeMap;

use crate::capability::MosaicSizingContractId;
use crate::declaration::{
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementConstraintModifier,
};
use crate::facade::WorthUiDslPackage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiSourceBackedDeclarationClaims {
    mosaic_membership_name: Box<str>,
    measurement_constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    measurement_basis_source: Option<UiDeclaredMeasurementBasisSource>,
    mosaic_sizing_contract_id: MosaicSizingContractId,
}

impl WorthUiSourceBackedDeclarationClaims {
    pub(crate) fn new(
        mosaic_membership_name: impl Into<Box<str>>,
        measurement_constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
        measurement_basis_source: Option<UiDeclaredMeasurementBasisSource>,
        mosaic_sizing_contract_id: MosaicSizingContractId,
    ) -> Self {
        Self {
            mosaic_membership_name: mosaic_membership_name.into(),
            measurement_constraint_modifier,
            measurement_basis_source,
            mosaic_sizing_contract_id,
        }
    }

    pub(crate) fn mosaic_membership_name(&self) -> &str {
        &self.mosaic_membership_name
    }

    pub(crate) fn measurement_constraint_modifier(
        &self,
    ) -> Option<UiDeclaredMeasurementConstraintModifier> {
        self.measurement_constraint_modifier
    }

    pub(crate) fn mosaic_sizing_contract_id(&self) -> &MosaicSizingContractId {
        &self.mosaic_sizing_contract_id
    }

    pub(crate) fn measurement_basis_source(&self) -> Option<UiDeclaredMeasurementBasisSource> {
        self.measurement_basis_source
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiSourceBackedDeclarationWitness {
    declaration_claims: BTreeMap<(String, usize), WorthUiSourceBackedDeclarationClaims>,
}

impl WorthUiSourceBackedDeclarationWitness {
    pub(crate) fn new(
        declaration_claims: BTreeMap<(String, usize), WorthUiSourceBackedDeclarationClaims>,
    ) -> Self {
        Self { declaration_claims }
    }

    pub(crate) fn claims_for(
        &self,
        module_path: &str,
        declaration_index: usize,
    ) -> Option<&WorthUiSourceBackedDeclarationClaims> {
        self.declaration_claims
            .get(&(module_path.to_owned(), declaration_index))
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        self.declaration_claims.iter().fold(
            fold_text("source-backed-declaration-witness"),
            |digest, ((module_path, declaration_index), claims)| {
                digest.rotate_left(7)
                    ^ fold_text(module_path)
                    ^ (*declaration_index as u64).rotate_left(11)
                    ^ fold_text(claims.mosaic_membership_name()).rotate_left(17)
                    ^ claims
                        .measurement_constraint_modifier()
                        .map_or(0, measurement_modifier_digest)
                        .rotate_left(23)
                    ^ claims
                        .measurement_basis_source()
                        .map_or(0, measurement_basis_digest)
                        .rotate_left(27)
                    ^ fold_text(claims.mosaic_sizing_contract_id().as_str()).rotate_left(29)
            },
        )
    }
}

fn measurement_basis_digest(basis: UiDeclaredMeasurementBasisSource) -> u64 {
    match basis {
        UiDeclaredMeasurementBasisSource::ViewportExtent => fold_text("viewport_extent"),
        UiDeclaredMeasurementBasisSource::ScrollViewport => fold_text("scroll_container_viewport"),
        UiDeclaredMeasurementBasisSource::PortalAnchor => fold_text("portal_anchor"),
    }
}

fn measurement_modifier_digest(modifier: UiDeclaredMeasurementConstraintModifier) -> u64 {
    match modifier {
        UiDeclaredMeasurementConstraintModifier::Bounded => fold_text("bounded"),
    }
}

fn fold_text(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xcbf2_9ce4_8422_2325, |digest, byte| {
            (digest ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
        })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiSourceBackedDslPackage {
    dsl_package: WorthUiDslPackage,
    declaration_witness: WorthUiSourceBackedDeclarationWitness,
}

impl WorthUiSourceBackedDslPackage {
    pub(crate) fn new(
        dsl_package: WorthUiDslPackage,
        declaration_witness: WorthUiSourceBackedDeclarationWitness,
    ) -> Self {
        Self {
            dsl_package,
            declaration_witness,
        }
    }

    pub(crate) fn dsl_package(&self) -> &WorthUiDslPackage {
        &self.dsl_package
    }

    pub(crate) fn declaration_witness(&self) -> &WorthUiSourceBackedDeclarationWitness {
        &self.declaration_witness
    }
}
