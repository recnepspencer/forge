use crate::identity::hash_parts;
use crate::ordinary::read::{
    declare, WorthQueryDeclaredReadIntent, WorthQueryReadDeclaration, WorthQueryReadDeclarationStop,
};
use crate::runtime::{WorthQueryReadBuilder, WorthQueryReadDenial};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryLiveDeclarationIdentity(String);

impl WorthQueryLiveDeclarationIdentity {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryLiveDeclaration {
    name: String,
    identity: WorthQueryLiveDeclarationIdentity,
    read: WorthQueryReadDeclaration,
}

impl WorthQueryLiveDeclaration {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn identity(&self) -> &WorthQueryLiveDeclarationIdentity {
        &self.identity
    }

    pub(crate) fn into_parts(self) -> (String, WorthQueryReadDeclaration) {
        (self.name, self.read)
    }

    pub(crate) fn from_installed_read(name: String, read: WorthQueryReadDeclaration) -> Self {
        let identity = live_declaration_identity(&name, &read);
        Self {
            name,
            identity,
            read,
        }
    }
}

#[derive(Debug)]
pub enum WorthQueryLiveDeclarationStop {
    EmptyResourceName,
    Read(WorthQueryReadDeclarationStop),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLiveDeclarationStopKind {
    EmptyResourceName,
    ReadDeclarationDenied,
}

impl WorthQueryLiveDeclarationStop {
    pub fn kind(&self) -> WorthQueryLiveDeclarationStopKind {
        match self {
            Self::EmptyResourceName => WorthQueryLiveDeclarationStopKind::EmptyResourceName,
            Self::Read(_) => WorthQueryLiveDeclarationStopKind::ReadDeclarationDenied,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::EmptyResourceName => "managed live resource name may not be empty",
            Self::Read(stop) => stop.denial().message(),
        }
    }

    pub fn next_action(&self) -> crate::ordinary::read::WorthQueryReadNextAction {
        crate::ordinary::read::WorthQueryReadNextAction::ReviseDeclaration
    }

    pub fn read_stop(&self) -> Option<&WorthQueryReadDeclarationStop> {
        match self {
            Self::Read(stop) => Some(stop),
            Self::EmptyResourceName => None,
        }
    }
}

pub fn declare_live(
    name: impl Into<String>,
    author: impl FnOnce(
        WorthQueryReadBuilder<WorthQueryDeclaredReadIntent>,
    ) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial>,
) -> Result<WorthQueryLiveDeclaration, WorthQueryLiveDeclarationStop> {
    let name = name.into();
    if name.trim().is_empty() {
        return Err(WorthQueryLiveDeclarationStop::EmptyResourceName);
    }
    let read = declare(author).map_err(WorthQueryLiveDeclarationStop::Read)?;
    let identity = live_declaration_identity(&name, &read);
    Ok(WorthQueryLiveDeclaration {
        name,
        identity,
        read,
    })
}

fn live_declaration_identity(
    name: &str,
    read: &WorthQueryReadDeclaration,
) -> WorthQueryLiveDeclarationIdentity {
    WorthQueryLiveDeclarationIdentity(hash_parts(&[
        "worth_query_managed_live_declaration_v1".to_string(),
        format!("name:{name}"),
        format!("read:{}", read.identity().as_str()),
    ]))
}
