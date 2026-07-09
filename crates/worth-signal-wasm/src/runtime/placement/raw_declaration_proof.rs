use worth_proof::{Recipe, Unresolved};
use serde::Serialize;

use super::declaration_candidate::{PlacementDeclarationCandidate, PlacementDeclarationOrigin};
use crate::runtime::core::WebSignalKind;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawPlacementDeclaration {
    id: String,
    signal_kind: String,
    declaration_origin: String,
}

impl RawPlacementDeclaration {
    fn new(
        id: String,
        signal_kind: Option<WebSignalKind>,
        origin: PlacementDeclarationOrigin,
    ) -> Self {
        Self {
            id,
            signal_kind: signal_kind_label(signal_kind).to_owned(),
            declaration_origin: declaration_origin_label(origin).to_owned(),
        }
    }

    pub(in crate::runtime::placement) fn id(&self) -> &str {
        &self.id
    }

    pub(in crate::runtime::placement) fn signal_kind(&self) -> &str {
        &self.signal_kind
    }

    pub(in crate::runtime::placement) fn declaration_origin(&self) -> &str {
        &self.declaration_origin
    }
}

pub type RawPlacementProof = Recipe<Unresolved, RawPlacementDeclaration>;

pub(in crate::runtime::placement) fn mint_raw_placement_proof(
    declaration: &PlacementDeclarationCandidate,
) -> RawPlacementProof {
    Recipe::<Unresolved, _>::new(RawPlacementDeclaration::new(
        declaration.id.clone(),
        declaration.signal_kind,
        declaration.origin,
    ))
}

fn declaration_origin_label(origin: PlacementDeclarationOrigin) -> &'static str {
    match origin {
        PlacementDeclarationOrigin::ExprSpec => "exprSpec",
        PlacementDeclarationOrigin::CallbackSignalTracked => "callbackSignalTracked",
        PlacementDeclarationOrigin::CallbackConstantizedNoSignalReads => {
            "callbackConstantizedNoSignalReads"
        }
    }
}

fn signal_kind_label(kind: Option<WebSignalKind>) -> &'static str {
    match kind {
        Some(WebSignalKind::Input) => "input",
        Some(WebSignalKind::Computed) => "computed",
        Some(WebSignalKind::Output) => "output",
        None => "internalRecipe",
    }
}
