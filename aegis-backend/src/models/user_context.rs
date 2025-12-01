use crate::models::postgres::{admin, organization, player};

#[derive(Debug, Clone)]
pub enum UserContext {
    Player(player::Model),
    Admin(admin::Model),
    Organization(organization::Model),
}

impl UserContext {
    pub fn get_id(&self) -> uuid::Uuid {
        match self {
            UserContext::Player(p) => p.id,
            UserContext::Admin(a) => a.id,
            UserContext::Organization(o) => o.id,
        }
    }

    pub fn get_user_type(&self) -> &str {
        match self {
            UserContext::Player(_) => "player",
            UserContext::Admin(_) => "admin",
            UserContext::Organization(_) => "organization",
        }
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, UserContext::Admin(_))
    }

    pub fn is_organization(&self) -> bool {
        matches!(self, UserContext::Organization(_))
    }
}
