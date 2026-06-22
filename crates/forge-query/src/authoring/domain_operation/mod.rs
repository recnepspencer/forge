mod declaration;
mod error;
mod identity;
mod reference;

pub use declaration::ForgeQueryGraphReadDomainOperationDeclaration;
pub use error::DomainGraphOperationDeclarationError;
pub use identity::{
    ForgeQueryDomainOwner, ForgeQueryGraphReadOperationKey, ForgeQueryGraphReadOperationName,
    ForgeQueryGraphReadOperationVersion,
};
pub use reference::ForgeQueryAdmittedGraphReadDomainOperationReference;
