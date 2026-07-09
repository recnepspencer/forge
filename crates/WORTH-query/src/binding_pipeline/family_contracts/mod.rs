#![allow(dead_code)]

use crate::application::{WorthQueryDeclarationAspectContract, WorthQueryDeclarationRouteIntent};

use super::{WorthQueryBindingSourceKind, WorthQueryBindingSpecificity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryFamilyBindingContract {
    family_key: &'static str,
    required_aspect_contract: WorthQueryDeclarationAspectContract,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryFamilyContextExtractorContract {
    family_key: &'static str,
    allowed_sources: Vec<WorthQueryBindingSourceKind>,
    required_aspect_contract: WorthQueryDeclarationAspectContract,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryFamilyTargetResolverContract {
    family_key: &'static str,
    required_aspect_contract: WorthQueryDeclarationAspectContract,
    route_intent: Option<WorthQueryDeclarationRouteIntent>,
    specificity_rank: WorthQueryBindingSpecificity,
}

impl WorthQueryFamilyBindingContract {
    pub fn family_key(&self) -> &'static str {
        self.family_key
    }

    pub fn required_aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.required_aspect_contract
    }

    pub(crate) fn new(
        family_key: &'static str,
        required_aspect_contract: WorthQueryDeclarationAspectContract,
    ) -> Self {
        Self {
            family_key,
            required_aspect_contract,
        }
    }
}

impl WorthQueryFamilyContextExtractorContract {
    pub fn family_key(&self) -> &'static str {
        self.family_key
    }

    pub fn allowed_sources(&self) -> &[WorthQueryBindingSourceKind] {
        &self.allowed_sources
    }

    pub fn required_aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.required_aspect_contract
    }

    pub(crate) fn new(
        family_key: &'static str,
        allowed_sources: Vec<WorthQueryBindingSourceKind>,
        required_aspect_contract: WorthQueryDeclarationAspectContract,
    ) -> Self {
        Self {
            family_key,
            allowed_sources,
            required_aspect_contract,
        }
    }
}

impl WorthQueryFamilyTargetResolverContract {
    pub fn family_key(&self) -> &'static str {
        self.family_key
    }

    pub fn required_aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.required_aspect_contract
    }

    pub fn route_intent(&self) -> Option<WorthQueryDeclarationRouteIntent> {
        self.route_intent
    }

    pub fn specificity_rank(&self) -> WorthQueryBindingSpecificity {
        self.specificity_rank
    }

    pub(crate) fn new(
        family_key: &'static str,
        required_aspect_contract: WorthQueryDeclarationAspectContract,
        route_intent: Option<WorthQueryDeclarationRouteIntent>,
        specificity_rank: WorthQueryBindingSpecificity,
    ) -> Self {
        Self {
            family_key,
            required_aspect_contract,
            route_intent,
            specificity_rank,
        }
    }
}
