use sea_orm::DeriveActiveEnum;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, Serialize, Deserialize, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "approval_status")]
pub enum ApprovalStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "approved")]
    Approved,
    #[sea_orm(string_value = "rejected")]
    Rejected,
    #[sea_orm(string_value = "not_applicable")]
    NotApplicable,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, Serialize, Deserialize, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "game_type")]
pub enum GameType {
    #[sea_orm(string_value = "BGMI")]
    Bgmi,
    #[sea_orm(string_value = "VALORANT")]
    Valorant,
    #[sea_orm(string_value = "CS2")]
    Cs2,
    #[sea_orm(string_value = "APEX")]
    Apex,
    #[sea_orm(string_value = "FORTNITE")]
    Fortnite,
    #[sea_orm(string_value = "LOL")]
    Lol,
    #[sea_orm(string_value = "DOTA2")]
    Dota2,
    #[sea_orm(string_value = "PUBG")]
    Pubg,
    #[sea_orm(string_value = "COD")]
    Cod,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, Serialize, Deserialize, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "admin_role")]
pub enum AdminRole {
    #[sea_orm(string_value = "super_admin")]
    SuperAdmin,
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "moderator")]
    Moderator,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, Serialize, Deserialize, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tournament_status")]
pub enum TournamentStatus {
    #[sea_orm(string_value = "announced")]
    Announced,
    #[sea_orm(string_value = "registration_open")]
    RegistrationOpen,
    #[sea_orm(string_value = "registration_closed")]
    RegistrationClosed,
    #[sea_orm(string_value = "in_progress")]
    InProgress,
    #[sea_orm(string_value = "completed")]
    Completed,
    #[sea_orm(string_value = "cancelled")]
    Cancelled,
    #[sea_orm(string_value = "postponed")]
    Postponed,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, Serialize, Deserialize, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "team_status")]
pub enum TeamStatus {
    #[sea_orm(string_value = "active")]
    Active,
    #[sea_orm(string_value = "inactive")]
    Inactive,
    #[sea_orm(string_value = "disbanded")]
    Disbanded,
    #[sea_orm(string_value = "looking_for_players")]
    LookingForPlayers,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, Serialize, Deserialize, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "battle_status")]
pub enum BattleStatus {
    #[sea_orm(string_value = "scheduled")]
    Scheduled,
    #[sea_orm(string_value = "in_progress")]
    InProgress,
    #[sea_orm(string_value = "completed")]
    Completed,
    #[sea_orm(string_value = "cancelled")]
    Cancelled,
}

impl ApprovalStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApprovalStatus::Pending => "pending",
            ApprovalStatus::Approved => "approved",
            ApprovalStatus::Rejected => "rejected",
            ApprovalStatus::NotApplicable => "not_applicable",
        }
    }
}

impl GameType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GameType::Bgmi => "BGMI",
            GameType::Valorant => "VALORANT",
            GameType::Cs2 => "CS2",
            GameType::Apex => "APEX",
            GameType::Fortnite => "FORTNITE",
            GameType::Lol => "LOL",
            GameType::Dota2 => "DOTA2",
            GameType::Pubg => "PUBG",
            GameType::Cod => "COD",
        }
    }
}

impl AdminRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            AdminRole::SuperAdmin => "super_admin",
            AdminRole::Admin => "admin",
            AdminRole::Moderator => "moderator",
        }
    }
}

impl TournamentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TournamentStatus::Announced => "announced",
            TournamentStatus::RegistrationOpen => "registration_open",
            TournamentStatus::RegistrationClosed => "registration_closed",
            TournamentStatus::InProgress => "in_progress",
            TournamentStatus::Completed => "completed",
            TournamentStatus::Cancelled => "cancelled",
            TournamentStatus::Postponed => "postponed",
        }
    }
}

impl TeamStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TeamStatus::Active => "active",
            TeamStatus::Inactive => "inactive",
            TeamStatus::Disbanded => "disbanded",
            TeamStatus::LookingForPlayers => "looking_for_players",
        }
    }
}

impl BattleStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            BattleStatus::Scheduled => "scheduled",
            BattleStatus::InProgress => "in_progress",
            BattleStatus::Completed => "completed",
            BattleStatus::Cancelled => "cancelled",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, Serialize, Deserialize, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum InviteStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "accepted")]
    Accepted,
    #[sea_orm(string_value = "declined")]
    Declined,
    #[sea_orm(string_value = "expired")]
    Expired,
}

// Add as_str() method for InviteStatus
impl InviteStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            InviteStatus::Pending => "pending",
            InviteStatus::Accepted => "accepted",
            InviteStatus::Declined => "declined",
            InviteStatus::Expired => "expired",
        }
    }
}
