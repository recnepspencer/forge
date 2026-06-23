use crate::capability::{CapabilitySnapshot, ComponentId, SurfaceId};
use crate::runtime::{
    authoring_snapshot::{
        WorthUiAuthoredSurfaceCatalog, WorthUiAuthoredSurfaceEntry,
        WorthUiAuthoredSurfacePropEntry, WorthUiAuthoredSurfacePropValue,
        WorthUiAuthoredSurfacePropsCatalog,
    },
    WorthUiPrimitiveSourceSpan,
};
use crate::source::{
    parse_surface_authoring_tokens, parse_surface_authoring_tokens_with_spans,
    WorthUiParsedSourceDeclaration, WorthUiParsedSourcePackage,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiAuthoredSurfaceCatalogDenial {
    UnknownSurface(String),
    UnknownComponent {
        surface_id: String,
        component_id: String,
    },
    MalformedSurfaceAuthoring(String),
}

pub(crate) fn build_authored_surface_catalogs(
    parsed: &WorthUiParsedSourcePackage,
    snapshot: &CapabilitySnapshot,
) -> Result<
    (
        WorthUiAuthoredSurfaceCatalog,
        WorthUiAuthoredSurfacePropsCatalog,
    ),
    WorthUiAuthoredSurfaceCatalogDenial,
> {
    let mut component_entries = Vec::new();
    let mut prop_entries = Vec::new();

    for module_id in parsed.module_ids() {
        let Some(module) = parsed.module(module_id) else {
            continue;
        };
        for declaration in module.declarations() {
            let WorthUiParsedSourceDeclaration::Surface(surface) = declaration else {
                continue;
            };
            let surface_id = SurfaceId::new(surface.name_text()).map_err(|_| {
                WorthUiAuthoredSurfaceCatalogDenial::UnknownSurface(surface.name_text().to_owned())
            })?;
            if snapshot.surfaces().get(&surface_id).is_none() {
                return Err(WorthUiAuthoredSurfaceCatalogDenial::UnknownSurface(
                    surface.name_text().to_owned(),
                ));
            }
            let authored =
                parse_surface_authoring_tokens(surface.body().tokens()).map_err(|_| {
                    WorthUiAuthoredSurfaceCatalogDenial::MalformedSurfaceAuthoring(
                        surface.name_text().to_owned(),
                    )
                })?;
            if let Some(component_id) = authored.component_id() {
                let component = ComponentId::new(component_id).map_err(|_| {
                    WorthUiAuthoredSurfaceCatalogDenial::UnknownComponent {
                        surface_id: surface.name_text().to_owned(),
                        component_id: component_id.to_owned(),
                    }
                })?;
                if snapshot.components().get(&component).is_none() {
                    return Err(WorthUiAuthoredSurfaceCatalogDenial::UnknownComponent {
                        surface_id: surface.name_text().to_owned(),
                        component_id: component_id.to_owned(),
                    });
                }
                component_entries.push(WorthUiAuthoredSurfaceEntry::new(
                    surface.name_text(),
                    component_id,
                    digest_surface_component_entry(surface.name_text(), component_id),
                ));
            }
            let authored_with_spans = parse_surface_authoring_tokens_with_spans(
                surface.body().tokens(),
            )
            .map_err(|_| {
                WorthUiAuthoredSurfaceCatalogDenial::MalformedSurfaceAuthoring(
                    surface.name_text().to_owned(),
                )
            })?;
            for property in authored.properties() {
                let source_span = authored_with_spans
                    .properties()
                    .iter()
                    .find(|spanned| spanned.key() == property.key())
                    .map(|spanned| {
                        WorthUiPrimitiveSourceSpan::new(
                            spanned.source_span().start_byte(),
                            spanned.source_span().end_byte(),
                        )
                    });
                let value = WorthUiAuthoredSurfacePropValue::from_source_value(property.value());
                prop_entries.push(WorthUiAuthoredSurfacePropEntry::new(
                    surface.name_text(),
                    property.key(),
                    value.clone(),
                    source_span,
                    digest_surface_prop_entry(surface.name_text(), property.key(), &value),
                ));
            }
        }
    }

    Ok((
        WorthUiAuthoredSurfaceCatalog::from_entries(component_entries),
        WorthUiAuthoredSurfacePropsCatalog::from_entries(prop_entries),
    ))
}

fn digest_surface_component_entry(surface_id: &str, component_id: &str) -> u64 {
    fold_bytes(
        fold_bytes(0xcbf2_9ce4_8422_2325, surface_id.as_bytes()),
        component_id.as_bytes(),
    )
}

fn digest_surface_prop_entry(
    surface_id: &str,
    key: &str,
    value: &WorthUiAuthoredSurfacePropValue,
) -> u64 {
    let digest = fold_bytes(0xcbf2_9ce4_8422_2325, surface_id.as_bytes());
    let digest = fold_bytes(digest, key.as_bytes());
    fold_bytes(digest, value.digest_basis().as_bytes())
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
