use super::WorthUiQueryViewDefinitionDigest;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryBindingContractIdentity(u64);

impl WorthUiQueryBindingContractIdentity {
    pub fn from_definitions(
        definitions: impl IntoIterator<Item = WorthUiQueryViewDefinitionDigest>,
    ) -> Self {
        let mut definitions = definitions.into_iter().collect::<Vec<_>>();
        definitions.sort_unstable();
        definitions.dedup();
        let identity = definitions.into_iter().fold(
            0x776f_7274_6875_6901_u64,
            |identity, definition| {
                identity
                    .rotate_left(13)
                    .wrapping_mul(0x100_0000_01b3)
                    ^ definition.as_u64()
            },
        );
        Self(identity)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }

    #[cfg(feature = "certification-construction")]
    pub fn from_definition_for_certification(definition: WorthUiQueryViewDefinitionDigest) -> Self {
        Self::from_definitions([definition])
    }
}
