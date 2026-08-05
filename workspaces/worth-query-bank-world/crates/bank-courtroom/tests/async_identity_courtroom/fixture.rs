use rand::distributions::{Alphanumeric, DistString};
use rand::rngs::OsRng;

pub struct IdentityFixture {
    slug: String,
    participants: Vec<IdentityParticipant>,
    client_id: String,
    client_secret: String,
    alternate_slug: String,
    alternate_client_id: String,
    alternate_client_secret: String,
    postgres_password: String,
    authentik_secret: String,
    bootstrap_token: String,
    redirect_url: String,
}

impl IdentityFixture {
    pub fn dynamic(redirect_url: String) -> Self {
        let suffix = random_text(12).to_ascii_lowercase();
        let participants = CourtroomIdentityRole::ALL
            .into_iter()
            .enumerate()
            .map(|(index, role)| IdentityParticipant {
                role,
                username: format!("user-{suffix}-{}", index + 1),
                password: random_text(32),
            })
            .collect();
        Self {
            slug: format!("worth-bank-{suffix}"),
            participants,
            client_id: format!("client-{suffix}"),
            client_secret: random_text(48),
            alternate_slug: format!("worth-bank-alternate-{suffix}"),
            alternate_client_id: format!("alternate-client-{suffix}"),
            alternate_client_secret: random_text(48),
            postgres_password: random_text(36),
            authentik_secret: random_text(72),
            bootstrap_token: random_text(48),
            redirect_url,
        }
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn participants(&self) -> &[IdentityParticipant] {
        &self.participants
    }

    pub fn primary_participant(&self) -> &IdentityParticipant {
        self.participants
            .first()
            .expect("courtroom actor inventory is nonempty")
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }

    pub fn alternate_slug(&self) -> &str {
        &self.alternate_slug
    }

    pub fn alternate_client_id(&self) -> &str {
        &self.alternate_client_id
    }

    pub fn alternate_client_secret(&self) -> &str {
        &self.alternate_client_secret
    }

    pub fn postgres_password(&self) -> &str {
        &self.postgres_password
    }

    pub fn authentik_secret(&self) -> &str {
        &self.authentik_secret
    }

    pub fn bootstrap_token(&self) -> &str {
        &self.bootstrap_token
    }

    pub fn redirect_url(&self) -> &str {
        &self.redirect_url
    }
}

pub struct IdentityParticipant {
    role: CourtroomIdentityRole,
    username: String,
    password: String,
}

impl IdentityParticipant {
    pub fn role(&self) -> CourtroomIdentityRole {
        self.role
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CourtroomIdentityRole {
    PersonalCustomerPrimary,
    PersonalCustomerPeer,
    BusinessOwner,
    BusinessInitiator,
    BusinessApprover,
    BusinessReader,
    BankTeller,
    BankAuditor,
    CustomerEmployee,
}

impl CourtroomIdentityRole {
    pub const ALL: [Self; 9] = [
        Self::PersonalCustomerPrimary,
        Self::PersonalCustomerPeer,
        Self::BusinessOwner,
        Self::BusinessInitiator,
        Self::BusinessApprover,
        Self::BusinessReader,
        Self::BankTeller,
        Self::BankAuditor,
        Self::CustomerEmployee,
    ];
}

fn random_text(length: usize) -> String {
    Alphanumeric.sample_string(&mut OsRng, length)
}
