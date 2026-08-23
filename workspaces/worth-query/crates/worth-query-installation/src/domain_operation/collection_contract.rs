use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationCollectionContract {
    NotCollection,
    Collection {
        row_identity_field: WorthQueryOperationCollectionField,
        ordering_fields: Vec<WorthQueryOperationCollectionField>,
        grouping: WorthQueryOperationGroupingContract,
        window: WorthQueryOperationWindowPolicy,
        continuation: WorthQueryOperationContinuationPosture,
    },
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryOperationCollectionField {
    aspect_key: AspectKey,
    field_path: CanonicalFieldPath,
}

impl WorthQueryOperationCollectionField {
    pub fn new(aspect_key: AspectKey, field_path: CanonicalFieldPath) -> Self {
        Self {
            aspect_key,
            field_path,
        }
    }

    pub fn from_dotted(value: &str) -> Option<Self> {
        let mut parts = value.split('.');
        let aspect_key = AspectKey::new(parts.next()?.to_owned())?;
        let fields = parts
            .map(|part| FieldKey::new(part.to_owned()))
            .collect::<Option<Vec<_>>>()?;
        Some(Self::new(
            aspect_key,
            CanonicalFieldPath::new(fields)?,
        ))
    }

    pub fn aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }

    pub fn field_path(&self) -> &CanonicalFieldPath {
        &self.field_path
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationGroupingContract {
    Ungrouped,
    Grouped {
        grouping_fields: Vec<WorthQueryOperationCollectionField>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationWindowPolicy {
    CompleteCollection,
    ContinuationBounded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationContinuationPosture {
    NotRequired,
    SnapshotCursor,
    LiveCursor,
}
