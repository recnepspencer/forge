use std::collections::BTreeMap;

use worth_foundational::facade::{
    prepare_aspect_value_identity_basis, prepare_struct_aspect_value_identity_basis, AspectKey,
    AspectValue, CanonicalFieldPath, FieldKey, StructAspectValue,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryRetainedFieldPath {
    locator: WorthQueryRetainedFieldLocator,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum WorthQueryRetainedFieldLocator {
    Canonical(CanonicalFieldPath),
    NativeAspect(AspectKey),
    NativeField(AspectKey, FieldKey),
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use worth_foundational::facade::{
        AspectValue, CanonicalFieldPath, FieldKey, StructAspectValue,
    };

    use super::{
        WorthQueryRetainedFieldPath, WorthQueryRetainedMaterializedRow, WorthQueryRetainedValueView,
    };

    #[test]
    fn retained_row_preserves_scalar_and_struct_shapes() {
        let scalar_path = path("metrics.rank");
        let struct_path = path("profile");
        let profile = StructAspectValue::new([(
            FieldKey::new("name").unwrap(),
            AspectValue::String("Ada".into()),
        )])
        .unwrap();
        let row = WorthQueryRetainedMaterializedRow::from_native_values(
            BTreeMap::from([(scalar_path.clone(), AspectValue::UInt64(7))]),
            BTreeMap::from([(struct_path.clone(), profile.clone())]),
        )
        .unwrap();

        assert!(matches!(
            row.native_value_at(&scalar_path),
            Some(WorthQueryRetainedValueView::Scalar(AspectValue::UInt64(7)))
        ));
        assert_eq!(row.struct_value_at(&struct_path), Some(&profile));
        assert_eq!(row.terminal_digest_parts().len(), 2);
    }

    #[test]
    fn retained_row_rejects_competing_shapes_at_one_path() {
        let shared = path("profile");
        let error = WorthQueryRetainedMaterializedRow::from_native_values(
            BTreeMap::from([(shared.clone(), AspectValue::Null)]),
            BTreeMap::from([(shared, StructAspectValue::new([]).unwrap())]),
        )
        .unwrap_err();
        assert!(error.contains("two shapes"));
    }

    fn path(value: &str) -> WorthQueryRetainedFieldPath {
        WorthQueryRetainedFieldPath::from_canonical_field_path(
            CanonicalFieldPath::new(
                value
                    .split('.')
                    .map(|field| FieldKey::new(field).unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
        )
    }
}

impl WorthQueryRetainedFieldPath {
    pub fn from_canonical_field_path(path: CanonicalFieldPath) -> Self {
        Self {
            locator: WorthQueryRetainedFieldLocator::Canonical(path),
        }
    }

    pub fn from_native_aspect_key(aspect: AspectKey) -> Self {
        Self {
            locator: WorthQueryRetainedFieldLocator::NativeAspect(aspect),
        }
    }

    pub fn from_native_keys(aspect: AspectKey, field: FieldKey) -> Self {
        Self {
            locator: WorthQueryRetainedFieldLocator::NativeField(aspect, field),
        }
    }

    pub fn canonical_field_path(&self) -> Option<&CanonicalFieldPath> {
        match &self.locator {
            WorthQueryRetainedFieldLocator::Canonical(path) => Some(path),
            WorthQueryRetainedFieldLocator::NativeAspect(_)
            | WorthQueryRetainedFieldLocator::NativeField(_, _) => None,
        }
    }

    pub(crate) fn terminal_projection_for_boundary(&self) -> String {
        match &self.locator {
            WorthQueryRetainedFieldLocator::Canonical(path) => path
                .fields()
                .iter()
                .map(FieldKey::as_str)
                .collect::<Vec<_>>()
                .join("."),
            WorthQueryRetainedFieldLocator::NativeAspect(aspect) => aspect.as_str().to_string(),
            WorthQueryRetainedFieldLocator::NativeField(aspect, field) => {
                format!("{}.{}", aspect.as_str(), field.as_str())
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRetainedMaterializedRow {
    scalar_values: BTreeMap<WorthQueryRetainedFieldPath, AspectValue>,
    struct_values: BTreeMap<WorthQueryRetainedFieldPath, StructAspectValue>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryRetainedValueView<'a> {
    Scalar(&'a AspectValue),
    Struct(&'a StructAspectValue),
}

impl WorthQueryRetainedMaterializedRow {
    pub(crate) fn from_scalar_values(
        scalar_values: BTreeMap<WorthQueryRetainedFieldPath, AspectValue>,
    ) -> Result<Self, String> {
        Self::from_native_values(scalar_values, BTreeMap::new())
    }

    pub(crate) fn from_native_values(
        scalar_values: BTreeMap<WorthQueryRetainedFieldPath, AspectValue>,
        struct_values: BTreeMap<WorthQueryRetainedFieldPath, StructAspectValue>,
    ) -> Result<Self, String> {
        if scalar_values.is_empty() && struct_values.is_empty() {
            return Err("retained materialized row requires at least one native value".to_string());
        }
        if scalar_values
            .keys()
            .any(|path| struct_values.contains_key(path))
        {
            return Err("retained materialized row cannot bind two shapes to one path".to_string());
        }
        Ok(Self {
            scalar_values,
            struct_values,
        })
    }

    pub fn scalar_value_at(
        &self,
        field_path: &WorthQueryRetainedFieldPath,
    ) -> Option<&AspectValue> {
        self.scalar_values.get(field_path)
    }

    pub fn struct_value_at(
        &self,
        field_path: &WorthQueryRetainedFieldPath,
    ) -> Option<&StructAspectValue> {
        self.struct_values.get(field_path)
    }

    pub fn native_value_at(
        &self,
        field_path: &WorthQueryRetainedFieldPath,
    ) -> Option<WorthQueryRetainedValueView<'_>> {
        self.scalar_value_at(field_path)
            .map(WorthQueryRetainedValueView::Scalar)
            .or_else(|| {
                self.struct_value_at(field_path)
                    .map(WorthQueryRetainedValueView::Struct)
            })
    }

    pub fn scalar_values(
        &self,
    ) -> impl Iterator<Item = (&WorthQueryRetainedFieldPath, &AspectValue)> {
        self.scalar_values.iter()
    }

    pub fn struct_values(
        &self,
    ) -> impl Iterator<Item = (&WorthQueryRetainedFieldPath, &StructAspectValue)> {
        self.struct_values.iter()
    }

    pub(in crate::runtime) fn terminal_digest_parts(&self) -> Vec<String> {
        self.scalar_values
            .iter()
            .map(|(field_path, value)| {
                format!(
                    "scalar:{}={}",
                    field_path.terminal_projection_for_boundary(),
                    prepare_aspect_value_identity_basis(value).as_str()
                )
            })
            .chain(self.struct_values.iter().map(|(field_path, value)| {
                format!(
                    "struct:{}={}",
                    field_path.terminal_projection_for_boundary(),
                    prepare_struct_aspect_value_identity_basis(value).as_str()
                )
            }))
            .collect()
    }
}
