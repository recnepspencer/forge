use worth_ui_dsl::UiDslAspectName;

use crate::declaration::aspect_contract::{
    digest_aspect_names, UiAspectContractAdmissionDenial, UiAspectName,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiConsumedAspectContract {
    aspects: Box<[UiAspectName]>,
}

impl UiConsumedAspectContract {
    pub(crate) fn admit(
        authored: &[UiDslAspectName],
    ) -> Result<Self, UiAspectContractAdmissionDenial> {
        let mut aspects = authored
            .iter()
            .map(UiAspectName::admit)
            .collect::<Result<Vec<_>, _>>()?;
        aspects.sort();
        aspects.dedup();

        Ok(Self {
            aspects: aspects.into_boxed_slice(),
        })
    }

    pub(crate) fn digest_raw(&self) -> u64 {
        digest_aspect_names(&self.aspects)
    }

    pub fn aspects(&self) -> &[UiAspectName] {
        &self.aspects
    }
}
