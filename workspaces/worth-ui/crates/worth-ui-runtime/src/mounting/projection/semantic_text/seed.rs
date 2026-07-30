use std::sync::Arc;

use super::super::static_paint::UiMountedStaticPaintSeed;
use super::super::UiMountedProjectionDenial;

mod collection_patch;

#[derive(Clone)]
pub(in crate::mounting::projection) struct UiMountedSemanticTextSeed {
    content: UiMountedSemanticTextSeedContent,
    posture: Arc<str>,
    color: worth_ui_host_contract::UiMountedRgba8,
    layer_semantic_order: u32,
}

#[derive(Clone)]
pub(in crate::mounting::projection) enum UiMountedSemanticTextSeedContent {
    Scalar(Option<Arc<str>>),
    Collection(Vec<crate::mounting::UiMountedCollectionTextRow>),
}

pub(in crate::mounting::projection) fn lower_semantic_text_seed(
    input: Option<&crate::mounting::UiMountedSemanticTextContent>,
    predecessor: Option<&UiMountedSemanticTextSeed>,
    paint: Option<UiMountedStaticPaintSeed>,
) -> Result<Option<UiMountedSemanticTextSeed>, UiMountedProjectionDenial> {
    let Some((content, posture)) = resolved_content(input, predecessor)? else {
        return Ok(None);
    };
    let paint = paint.ok_or(UiMountedProjectionDenial::MissingSemanticTextColor)?;
    let layer_semantic_order = paint
        .layer_semantic_order()
        .checked_add(1)
        .ok_or(UiMountedProjectionDenial::SemanticTextLayerOrderExceeded)?;
    Ok(Some(UiMountedSemanticTextSeed {
        content,
        posture,
        color: paint.color(),
        layer_semantic_order,
    }))
}

fn resolved_content(
    input: Option<&crate::mounting::UiMountedSemanticTextContent>,
    predecessor: Option<&UiMountedSemanticTextSeed>,
) -> Result<Option<(UiMountedSemanticTextSeedContent, Arc<str>)>, UiMountedProjectionDenial> {
    let Some(input) = input else {
        return Ok(predecessor.map(|seed| (seed.content.clone(), Arc::clone(&seed.posture))));
    };
    match input {
        crate::mounting::UiMountedSemanticTextContent::Scalar(input) => {
            resolve_scalar(input, predecessor).map(Some)
        }
        crate::mounting::UiMountedSemanticTextContent::Collection(input) => {
            resolve_collection(input, predecessor).map(Some)
        }
    }
}

fn resolve_scalar(
    input: &crate::mounting::UiMountedScalarSemanticTextContent,
    predecessor: Option<&UiMountedSemanticTextSeed>,
) -> Result<(UiMountedSemanticTextSeedContent, Arc<str>), UiMountedProjectionDenial> {
    let predecessor = predecessor
        .map(UiMountedSemanticTextSeed::scalar_value)
        .transpose()?
        .flatten();
    let value = match input.value() {
        crate::mounting::UiMountedSemanticTextValueDirective::Replace(value) => {
            Some(Arc::clone(value))
        }
        crate::mounting::UiMountedSemanticTextValueDirective::Preserve => predecessor,
        crate::mounting::UiMountedSemanticTextValueDirective::Clear => None,
    };
    Ok((
        UiMountedSemanticTextSeedContent::Scalar(value),
        Arc::clone(input.posture()),
    ))
}

fn resolve_collection(
    input: &crate::mounting::UiMountedCollectionSemanticTextContent,
    predecessor: Option<&UiMountedSemanticTextSeed>,
) -> Result<(UiMountedSemanticTextSeedContent, Arc<str>), UiMountedProjectionDenial> {
    let predecessor = predecessor
        .map(UiMountedSemanticTextSeed::collection_rows)
        .transpose()?;
    let rows = match input.value() {
        crate::mounting::UiMountedCollectionTextDirective::Replace(rows) => rows.to_vec(),
        crate::mounting::UiMountedCollectionTextDirective::Patch(changes) => {
            collection_patch::apply(
                predecessor
                    .ok_or(UiMountedProjectionDenial::MissingSemanticCollectionPredecessor)?,
                changes,
            )?
        }
        crate::mounting::UiMountedCollectionTextDirective::Preserve => {
            predecessor.unwrap_or_default().to_vec()
        }
        crate::mounting::UiMountedCollectionTextDirective::Clear => Vec::new(),
    };
    Ok((
        UiMountedSemanticTextSeedContent::Collection(rows),
        Arc::clone(input.posture()),
    ))
}

impl UiMountedSemanticTextSeed {
    fn scalar_value(&self) -> Result<Option<Arc<str>>, UiMountedProjectionDenial> {
        match &self.content {
            UiMountedSemanticTextSeedContent::Scalar(value) => Ok(value.clone()),
            UiMountedSemanticTextSeedContent::Collection(_) => {
                Err(UiMountedProjectionDenial::SemanticTextShapeMismatch)
            }
        }
    }

    fn collection_rows(
        &self,
    ) -> Result<&[crate::mounting::UiMountedCollectionTextRow], UiMountedProjectionDenial> {
        match &self.content {
            UiMountedSemanticTextSeedContent::Collection(rows) => Ok(rows),
            UiMountedSemanticTextSeedContent::Scalar(_) => {
                Err(UiMountedProjectionDenial::SemanticTextShapeMismatch)
            }
        }
    }

    pub(in crate::mounting::projection) const fn content(
        &self,
    ) -> &UiMountedSemanticTextSeedContent {
        &self.content
    }

    pub(in crate::mounting::projection) fn posture(&self) -> &Arc<str> {
        &self.posture
    }

    pub(in crate::mounting::projection) const fn color(
        &self,
    ) -> worth_ui_host_contract::UiMountedRgba8 {
        self.color
    }

    pub(in crate::mounting::projection) const fn layer_semantic_order(&self) -> u32 {
        self.layer_semantic_order
    }
}
