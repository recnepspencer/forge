#![allow(dead_code)]

use crate::application::{ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationRouteIntent};

use super::{ForgeQueryBindingSourceKind, ForgeQueryBindingSpecificity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryFamilyBindingContract {
    family_key: &'static str,
    required_aspect_contract: ForgeQueryDeclarationAspectContract,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryFamilyContextExtractorContract {
    family_key: &'static str,
    allowed_sources: Vec<ForgeQueryBindingSourceKind>,
    required_aspect_contract: ForgeQueryDeclarationAspectContract,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryFamilyTargetResolverContract {
    family_key: &'static str,
    required_aspect_contract: ForgeQueryDeclarationAspectContract,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
    specificity_rank: ForgeQueryBindingSpecificity,
}

impl ForgeQueryFamilyBindingContract {
    pub fn family_key(&self) -> &'static str {
        self.family_key
    }

    pub fn required_aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.required_aspect_contract
    }

    pub(crate) fn new(
        family_key: &'static str,
        required_aspect_contract: ForgeQueryDeclarationAspectContract,
    ) -> Self {
        Self {
            family_key,
            required_aspect_contract,
        }
    }
}

impl ForgeQueryFamilyContextExtractorContract {
    pub fn family_key(&self) -> &'static str {
        self.family_key
    }

    pub fn allowed_sources(&self) -> &[ForgeQueryBindingSourceKind] {
        &self.allowed_sources
    }

    pub fn required_aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.required_aspect_contract
    }

    pub(crate) fn new(
        family_key: &'static str,
        allowed_sources: Vec<ForgeQueryBindingSourceKind>,
        required_aspect_contract: ForgeQueryDeclarationAspectContract,
    ) -> Self {
        Self {
            family_key,
            allowed_sources,
            required_aspect_contract,
        }
    }
}

impl ForgeQueryFamilyTargetResolverContract {
    pub fn family_key(&self) -> &'static str {
        self.family_key
    }

    pub fn required_aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.required_aspect_contract
    }

    pub fn route_intent(&self) -> Option<ForgeQueryDeclarationRouteIntent> {
        self.route_intent
    }

    pub fn specificity_rank(&self) -> ForgeQueryBindingSpecificity {
        self.specificity_rank
    }

    pub(crate) fn new(
        family_key: &'static str,
        required_aspect_contract: ForgeQueryDeclarationAspectContract,
        route_intent: Option<ForgeQueryDeclarationRouteIntent>,
        specificity_rank: ForgeQueryBindingSpecificity,
    ) -> Self {
        Self {
            family_key,
            required_aspect_contract,
            route_intent,
            specificity_rank,
        }
    }
}
