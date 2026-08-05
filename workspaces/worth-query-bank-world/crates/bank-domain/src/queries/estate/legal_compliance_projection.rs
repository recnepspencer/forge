use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

use crate::{estate::EstateCaseId, reads::EstateLegalAuthorityView, schema::BankSchema};

use super::{legal_compliance::EstateLegalComplianceQuery, legal_compliance_selectors::*};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstateLegalComplianceResult {
    estate: EstateCaseId,
    authorities: Vec<EstateLegalAuthorityView>,
}

impl EstateLegalComplianceResult {
    pub const fn estate(&self) -> EstateCaseId {
        self.estate
    }

    pub fn authorities(&self) -> &[EstateLegalAuthorityView] {
        &self.authorities
    }
}

impl WorthQueryApplicationProjection<BankSchema, EstateLegalComplianceQuery>
    for EstateLegalComplianceResult
{
    fn project(
        row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateLegalComplianceQuery>,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        let authorities = row
            .many(estate_authorities())?
            .iter()
            .map(|authority| {
                Ok(EstateLegalAuthorityView::from_projection(
                    authority.field(authority_identity())?,
                    authority
                        .one(authority_holder())?
                        .field(authority_holder_identity())?,
                    authority.field(authority_kind())?,
                    authority.field(authority_recognized())?,
                ))
            })
            .collect::<Result<Vec<_>, WorthQueryApplicationProjectionDenial>>()?;
        Ok(Self {
            estate: row.field(estate_identity())?,
            authorities,
        })
    }
}
