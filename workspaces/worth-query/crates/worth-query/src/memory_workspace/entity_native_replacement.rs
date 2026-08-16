use worth_foundational::facade::{
    AspectKey, AspectValue, CanonicalFieldPath, FieldKey, StructAspectValue,
};

pub(crate) enum WorthQueryEntityNativeReplacementValue {
    Scalar(AspectValue),
    Struct(StructAspectValue),
    Absent,
}

pub(crate) struct WorthQueryEntityNativeReplacement {
    pub(super) aspect: AspectKey,
    pub(super) field: Option<FieldKey>,
    pub(super) canonical_paths: Vec<CanonicalFieldPath>,
    pub(super) replace_aspect_storage: bool,
    pub(super) value: WorthQueryEntityNativeReplacementValue,
}

impl WorthQueryEntityNativeReplacement {
    pub(crate) fn new(
        aspect: AspectKey,
        field: Option<FieldKey>,
        canonical_paths: impl IntoIterator<Item = CanonicalFieldPath>,
        value: WorthQueryEntityNativeReplacementValue,
    ) -> Self {
        let mut canonical_paths = canonical_paths.into_iter().collect::<Vec<_>>();
        canonical_paths.sort();
        canonical_paths.dedup();
        Self {
            aspect,
            field,
            canonical_paths,
            replace_aspect_storage: true,
            value,
        }
    }

    pub(crate) fn canonical_paths_only(
        aspect: AspectKey,
        canonical_paths: impl IntoIterator<Item = CanonicalFieldPath>,
        value: WorthQueryEntityNativeReplacementValue,
    ) -> Self {
        let mut replacement = Self::new(aspect, None, canonical_paths, value);
        replacement.replace_aspect_storage = false;
        replacement
    }
}
