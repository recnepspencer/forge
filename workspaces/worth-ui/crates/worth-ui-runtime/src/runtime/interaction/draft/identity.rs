/// Declaration-owned identity for one draft-backed payload field.
///
/// The interaction runtime can carry this identity, but only a payload-typed
/// field handle may mint it.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiDraftFieldIdentity {
    schema: crate::capability::UiIntentSchema,
    field: crate::capability::UiIntentPayloadFieldDescriptor,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiDraftSessionIdentity(u64);

impl UiDraftFieldIdentity {
    pub(crate) const fn from_payload_field<
        P: crate::capability::UiIntentPayload,
        K: crate::capability::UiIntentPayloadValueKind,
    >(
        field: crate::capability::UiIntentPayloadField<P, K>,
    ) -> Self {
        Self {
            schema: P::SCHEMA,
            field: field.descriptor(),
        }
    }

    pub const fn schema(self) -> crate::capability::UiIntentSchema {
        self.schema
    }

    pub const fn field(self) -> crate::capability::UiIntentPayloadFieldDescriptor {
        self.field
    }
}

impl UiDraftSessionIdentity {
    pub(super) const fn mint(value: u64) -> Self {
        Self(value)
    }

    pub const fn diagnostic_value(self) -> u64 {
        self.0
    }
}
