/// Validated descriptive input. It is not a product branch identity and does
/// not select an owner or a current head.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductBranchName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductBranchNameDenial {
    Empty,
    TooLong { maximum: usize, actual: usize },
}

impl ProductBranchName {
    const MAXIMUM_LENGTH: usize = 256;

    pub fn try_new(name: impl Into<String>) -> Result<Self, ProductBranchNameDenial> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(ProductBranchNameDenial::Empty);
        }
        if name.len() > Self::MAXIMUM_LENGTH {
            return Err(ProductBranchNameDenial::TooLong {
                maximum: Self::MAXIMUM_LENGTH,
                actual: name.len(),
            });
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
