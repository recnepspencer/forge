use super::WorthQueryDecisionRecordContract;

pub(super) fn contract_is_valid(contract: &WorthQueryDecisionRecordContract) -> bool {
    match contract {
        WorthQueryDecisionRecordContract::NotRequired => true,
        WorthQueryDecisionRecordContract::Declared { schemas } => {
            !schemas.is_empty()
                && schemas
                    .iter()
                    .all(|schema| schema.payload_version().get() > 0)
                && !schemas
                    .windows(2)
                    .any(|pair| pair[0].kind() == pair[1].kind())
        }
    }
}

pub(super) fn portable_identity(value: &str) -> bool {
    !value.trim().is_empty() && value.trim() == value && !value.chars().any(char::is_whitespace)
}
