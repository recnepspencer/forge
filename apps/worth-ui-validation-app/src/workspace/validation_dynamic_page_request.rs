use super::ValidationDynamicPageKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationDynamicPageRequest {
    ProductDetail { product_id: String },
    OrderDetail { order_id: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationDynamicPageRequestDenial {
    EmptyParameter {
        kind: ValidationDynamicPageKind,
        parameter_name: &'static str,
    },
}

impl ValidationDynamicPageRequest {
    pub fn product_detail(
        product_id: impl Into<String>,
    ) -> Result<Self, ValidationDynamicPageRequestDenial> {
        let product_id = normalize_parameter_value(product_id);
        if product_id.is_empty() {
            return Err(ValidationDynamicPageRequestDenial::EmptyParameter {
                kind: ValidationDynamicPageKind::ProductDetail,
                parameter_name: "product_id",
            });
        }
        Ok(Self::ProductDetail { product_id })
    }

    pub fn order_detail(
        order_id: impl Into<String>,
    ) -> Result<Self, ValidationDynamicPageRequestDenial> {
        let order_id = normalize_parameter_value(order_id);
        if order_id.is_empty() {
            return Err(ValidationDynamicPageRequestDenial::EmptyParameter {
                kind: ValidationDynamicPageKind::OrderDetail,
                parameter_name: "order_id",
            });
        }
        Ok(Self::OrderDetail { order_id })
    }

    pub(crate) fn kind(&self) -> ValidationDynamicPageKind {
        match self {
            Self::ProductDetail { .. } => ValidationDynamicPageKind::ProductDetail,
            Self::OrderDetail { .. } => ValidationDynamicPageKind::OrderDetail,
        }
    }

    pub(crate) fn parameter_value(&self) -> &str {
        match self {
            Self::ProductDetail { product_id } => product_id.as_str(),
            Self::OrderDetail { order_id } => order_id.as_str(),
        }
    }
}

fn normalize_parameter_value(value: impl Into<String>) -> String {
    value.into().trim().to_owned()
}
