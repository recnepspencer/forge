use crate::declaration::UiAspectFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiAspectContractAdmission {
    Admitted(super::UiAspectContract),
    Denied(UiAspectContractAdmissionDenial),
}

impl UiAspectContractAdmission {
    pub(crate) fn digest_raw(&self) -> u64 {
        match self {
            Self::Admitted(contract) => contract.digest_raw(),
            Self::Denied(denial) => denial.digest_raw(),
        }
    }

    pub fn admitted_contract(
        &self,
    ) -> Result<&super::UiAspectContract, &UiAspectContractAdmissionDenial> {
        match self {
            Self::Admitted(contract) => Ok(contract),
            Self::Denied(denial) => Err(denial),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiAspectContractAdmissionDenial {
    MalformedAspectName {
        authored: String,
    },
    UnsupportedAspectFamily {
        authored: String,
        observed_family: String,
    },
    UnsupportedAspectSemanticSlice {
        family: UiAspectFamily,
        canonical_label: String,
    },
}

impl UiAspectContractAdmissionDenial {
    fn digest_raw(&self) -> u64 {
        match self {
            Self::MalformedAspectName { authored } => {
                stable_text_digest("aspect:malformed") ^ stable_text_digest(authored).rotate_left(7)
            }
            Self::UnsupportedAspectFamily {
                authored,
                observed_family,
            } => {
                stable_text_digest("aspect:family")
                    ^ stable_text_digest(authored).rotate_left(7)
                    ^ stable_text_digest(observed_family).rotate_left(13)
            }
            Self::UnsupportedAspectSemanticSlice {
                family,
                canonical_label,
            } => {
                stable_text_digest("aspect:slice")
                    ^ stable_text_digest(canonical_label).rotate_left(7)
                    ^ stable_text_digest(aspect_family_name(*family)).rotate_left(13)
            }
        }
    }
}

fn aspect_family_name(family: UiAspectFamily) -> &'static str {
    match family {
        UiAspectFamily::Structure => "structure",
        UiAspectFamily::Presence => "presence",
        UiAspectFamily::Participation => "participation",
        UiAspectFamily::Layout => "layout",
        UiAspectFamily::Appearance => "appearance",
        UiAspectFamily::Content => "content",
        UiAspectFamily::Interaction => "interaction",
        UiAspectFamily::Service => "service",
        UiAspectFamily::Diagnostic => "diagnostic",
    }
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
