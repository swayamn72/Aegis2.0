use sea_orm_migration::prelude::*;
use sea_orm_migration::prelude::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create ENUMs first
        manager.create_type(
            Type::create()
                .as_enum(GameType::Table)
                .values([
                    GameType::BGMI, GameType::VALORANT, GameType::CS2, 
                    GameType::APEX, GameType::FORTNITE, GameType::LOL, 
                    GameType::DOTA2, GameType::PUBG, GameType::COD
                ])
                .to_owned(),
        ).await?;

        manager.create_type(
            Type::create()
                .as_enum(TournamentStatus::Table)
                .values([
                    TournamentStatus::Announced, TournamentStatus::RegistrationOpen,
                    TournamentStatus::RegistrationClosed, TournamentStatus::InProgress,
                    TournamentStatus::Completed, TournamentStatus::Cancelled,
                    TournamentStatus::Postponed
                ])
                .to_owned(),
        ).await?;

        manager.create_type(
            Type::create()
                .as_enum(TeamStatus::Table)
                .values([
                    TeamStatus::Active, TeamStatus::Inactive,
                    TeamStatus::Disbanded, TeamStatus::LookingForPlayers
                ])
                .to_owned(),
        ).await?;

        manager.create_type(
            Type::create()
                .as_enum(BattleStatus::Table)
                .values([
                    BattleStatus::Scheduled, BattleStatus::InProgress,
                    BattleStatus::Completed, BattleStatus::Cancelled
                ])
                .to_owned(),
        ).await?;

        manager.create_type(
            Type::create()
                .as_enum(ApprovalStatus::Table)
                .values([
                    ApprovalStatus::Pending, ApprovalStatus::Approved,
                    ApprovalStatus::Rejected, ApprovalStatus::NotApplicable
                ])
                .to_owned(),
        ).await?;

        manager.create_type(
            Type::create()
                .as_enum(AdminRole::Table)
                .values([AdminRole::SuperAdmin, AdminRole::Admin, AdminRole::Moderator])
                .to_owned(),
        ).await?;

        // Create tables in dependency order
        self.create_players_table(manager).await?;
        self.create_organizations_table(manager).await?;
        self.create_teams_table(manager).await?;
        self.create_admins_table(manager).await?;
        self.create_tournaments_table(manager).await?;
        self.create_player_game_stats_table(manager).await?;
        self.create_battles_table(manager).await?;
        self.create_tournament_teams_table(manager).await?;
        self.create_tournament_team_invites_table(manager).await?;
        self.create_transactions_table(manager).await?;
        self.create_rewards_table(manager).await?;

        // Add foreign key constraints
        self.add_foreign_keys(manager).await?;
        
        // Create performance indexes
        self.create_indexes(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse dependency order
        manager.drop_table(Table::drop().table(Rewards::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Transactions::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(TournamentTeamInvites::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(TournamentTeams::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Battles::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(PlayerGameStats::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Tournaments::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Admins::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Teams::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Organizations::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Players::Table).to_owned()).await?;

        // Drop ENUMs
        manager.drop_type(Type::drop().name(AdminRole::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(ApprovalStatus::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(BattleStatus::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(TeamStatus::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(TournamentStatus::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(GameType::Table).to_owned()).await?;

        Ok(())
    }
}

impl Migration {
    async fn create_players_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Players::Table)
                .if_not_exists()
                .col(ColumnDef::new(Players::Id).uuid().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(Players::CognitoSub).string_len(255).unique_key())
                .col(ColumnDef::new(Players::Username).string_len(50).unique_key().not_null())
                .col(ColumnDef::new(Players::InGameName).string_len(100))
                .col(ColumnDef::new(Players::RealName).string_len(100))
                .col(ColumnDef::new(Players::Email).string_len(255).unique_key().not_null())
                .col(ColumnDef::new(Players::Password).string_len(255).not_null())
                .col(ColumnDef::new(Players::ResetPasswordToken).string_len(255))
                .col(ColumnDef::new(Players::ResetPasswordExpiry).timestamp_with_time_zone())
                .col(ColumnDef::new(Players::Verified).boolean().default(false))
                .col(ColumnDef::new(Players::Country).string_len(100))
                .col(ColumnDef::new(Players::Bio).text().default(""))
                .col(ColumnDef::new(Players::ProfilePicture).text().default(""))
                .col(ColumnDef::new(Players::PrimaryGame).custom(GameType::Table))
                .col(ColumnDef::new(Players::Earnings).decimal_len(15, 2).default(0))
                .col(ColumnDef::new(Players::InGameRole).array(ColumnType::Text))
                .col(ColumnDef::new(Players::Location).string_len(100))
                .col(ColumnDef::new(Players::Age).integer())
                .col(ColumnDef::new(Players::Languages).array(ColumnType::Text))
                .col(ColumnDef::new(Players::AegisRating).integer().default(0))
                .col(ColumnDef::new(Players::TournamentsPlayed).integer().default(0))
                .col(ColumnDef::new(Players::BattlesPlayed).integer().default(0))
                .col(ColumnDef::new(Players::QualifiedEvents).boolean().default(false))
                .col(ColumnDef::new(Players::QualifiedEventDetails).array(ColumnType::Text))
                .col(ColumnDef::new(Players::TeamStatus).string_len(50))
                .col(ColumnDef::new(Players::TeamId).uuid())
                .col(ColumnDef::new(Players::Availability).string_len(50))
                .col(ColumnDef::new(Players::DiscordTag).string_len(100).default(""))
                .col(ColumnDef::new(Players::Twitch).string_len(255).default(""))
                .col(ColumnDef::new(Players::Youtube).string_len(255).default(""))
                .col(ColumnDef::new(Players::Twitter).string_len(255).default(""))
                .col(ColumnDef::new(Players::ProfileVisibility).string_len(20).default("public"))
                .col(ColumnDef::new(Players::CardTheme).string_len(20).default("orange"))
                .col(ColumnDef::new(Players::Coins).big_integer().default(0))
                .col(ColumnDef::new(Players::LastCheckIn).timestamp_with_time_zone())
                .col(ColumnDef::new(Players::CheckInStreak).integer().default(0))
                .col(ColumnDef::new(Players::TotalCheckIns).integer().default(0))
                .col(ColumnDef::new(Players::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Players::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .to_owned(),
        ).await
    }

    async fn create_organizations_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Organizations::Table)
                .if_not_exists()
                .col(ColumnDef::new(Organizations::Id).uuid().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(Organizations::CognitoSub).string_len(255).unique_key())
                .col(ColumnDef::new(Organizations::OrgName).string_len(200).unique_key().not_null())
                .col(ColumnDef::new(Organizations::OwnerName).string_len(100).not_null())
                .col(ColumnDef::new(Organizations::Email).string_len(255).unique_key().not_null())
                .col(ColumnDef::new(Organizations::GoogleId).string_len(255))
                .col(ColumnDef::new(Organizations::Password).string_len(255).not_null())
                .col(ColumnDef::new(Organizations::Country).string_len(100).not_null())
                .col(ColumnDef::new(Organizations::Headquarters).string_len(200))
                .col(ColumnDef::new(Organizations::Description).text().default(""))
                .col(ColumnDef::new(Organizations::Logo).text().default(""))
                .col(ColumnDef::new(Organizations::EstablishedDate).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Organizations::ActiveGames).array(ColumnType::Text))
                .col(ColumnDef::new(Organizations::TotalEarnings).decimal_len(15, 2).default(0))
                .col(ColumnDef::new(Organizations::ContactPhone).string_len(20).default(""))
                .col(ColumnDef::new(Organizations::Discord).string_len(255).default(""))
                .col(ColumnDef::new(Organizations::Twitter).string_len(255).default(""))
                .col(ColumnDef::new(Organizations::Twitch).string_len(255).default(""))
                .col(ColumnDef::new(Organizations::Youtube).string_len(255).default(""))
                .col(ColumnDef::new(Organizations::Website).string_len(255).default(""))
                .col(ColumnDef::new(Organizations::Linkedin).string_len(255).default(""))
                .col(ColumnDef::new(Organizations::ProfileVisibility).string_len(20).default("public"))
                .col(ColumnDef::new(Organizations::ApprovalStatus).custom(ApprovalStatus::Table).default("pending"))
                .col(ColumnDef::new(Organizations::ApprovedBy).uuid())
                .col(ColumnDef::new(Organizations::ApprovalDate).timestamp_with_time_zone())
                .col(ColumnDef::new(Organizations::RejectionReason).text())
                .col(ColumnDef::new(Organizations::EmailVerified).boolean().default(false))
                .col(ColumnDef::new(Organizations::VerificationToken).string_len(255))
                .col(ColumnDef::new(Organizations::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Organizations::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .to_owned(),
        ).await
    }

    async fn create_teams_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Teams::Table)
                .if_not_exists()
                .col(ColumnDef::new(Teams::Id).uuid().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(Teams::TeamName).string_len(100).unique_key().not_null())
                .col(ColumnDef::new(Teams::TeamTag).string_len(5).unique_key())
                .col(ColumnDef::new(Teams::Logo).text().default("https://placehold.co/200x200/1a1a1a/ffffff?text=TEAM"))
                .col(ColumnDef::new(Teams::Captain).uuid())
                .col(ColumnDef::new(Teams::PrimaryGame).custom(GameType::Table).default("BGMI"))
                .col(ColumnDef::new(Teams::Region).string_len(50).default("India"))
                .col(ColumnDef::new(Teams::Country).string_len(100))
                .col(ColumnDef::new(Teams::Bio).text().default(""))
                .col(ColumnDef::new(Teams::EstablishedDate).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Teams::TotalEarnings).decimal_len(15, 2).default(0))
                .col(ColumnDef::new(Teams::AegisRating).integer().default(0))
                .col(ColumnDef::new(Teams::OrganizationId).uuid())
                .col(ColumnDef::new(Teams::Discord).string_len(255).default(""))
                .col(ColumnDef::new(Teams::Twitter).string_len(255).default(""))
                .col(ColumnDef::new(Teams::Twitch).string_len(255).default(""))
                .col(ColumnDef::new(Teams::Youtube).string_len(255).default(""))
                .col(ColumnDef::new(Teams::Website).string_len(255).default(""))
                .col(ColumnDef::new(Teams::ProfileVisibility).string_len(20).default("public"))
                .col(ColumnDef::new(Teams::Status).custom(TeamStatus::Table).default("active"))
                .col(ColumnDef::new(Teams::LookingForPlayers).boolean().default(false))
                .col(ColumnDef::new(Teams::OpenRoles).array(ColumnType::Text))
                .col(ColumnDef::new(Teams::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Teams::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .to_owned(),
        ).await
    }

    async fn create_admins_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Admins::Table)
                .if_not_exists()
                .col(ColumnDef::new(Admins::Id).uuid().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(Admins::Username).string_len(50).unique_key().not_null())
                .col(ColumnDef::new(Admins::Email).string_len(255).unique_key().not_null())
                .col(ColumnDef::new(Admins::Password).string_len(255).not_null())
                .col(ColumnDef::new(Admins::Role).custom(AdminRole::Table).default("admin"))
                .col(ColumnDef::new(Admins::Permissions).json().default("{}"))
                .col(ColumnDef::new(Admins::IsActive).boolean().default(true))
                .col(ColumnDef::new(Admins::LastLogin).timestamp_with_time_zone())
                .col(ColumnDef::new(Admins::LoginAttempts).integer().default(0))
                .col(ColumnDef::new(Admins::LockUntil).timestamp_with_time_zone())
                .col(ColumnDef::new(Admins::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Admins::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .to_owned(),
        ).await
    }

    async fn create_tournaments_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Tournaments::Table)
                .if_not_exists()
                .col(ColumnDef::new(Tournaments::Id).uuid().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(Tournaments::TournamentName).string_len(150).unique_key().not_null())
                .col(ColumnDef::new(Tournaments::ShortName).string_len(50))
                .col(ColumnDef::new(Tournaments::Slug).string_len(200).unique_key())
                .col(ColumnDef::new(Tournaments::GameTitle).string_len(50).default("BGMI"))
                .col(ColumnDef::new(Tournaments::Tier).string_len(20).default("Community"))
                .col(ColumnDef::new(Tournaments::Region).string_len(50).default("India"))
                .col(ColumnDef::new(Tournaments::SubRegion).string_len(100))
                .col(ColumnDef::new(Tournaments::Organizer).json().default("{}"))
                .col(ColumnDef::new(Tournaments::Sponsors).json().default("[]"))
                .col(ColumnDef::new(Tournaments::AnnouncementDate).timestamp_with_time_zone())
                .col(ColumnDef::new(Tournaments::IsOpenForAll).boolean().default(false))
                .col(ColumnDef::new(Tournaments::RegistrationStartDate).timestamp_with_time_zone())
                .col(ColumnDef::new(Tournaments::RegistrationEndDate).timestamp_with_time_zone())
                .col(ColumnDef::new(Tournaments::StartDate).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Tournaments::EndDate).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Tournaments::Status).custom(TournamentStatus::Table).default("announced"))
                .col(ColumnDef::new(Tournaments::Format).string_len(100))
                .col(ColumnDef::new(Tournaments::FormatDetails).text())
                .col(ColumnDef::new(Tournaments::Slots).json().default("{}"))
                .col(ColumnDef::new(Tournaments::ParticipatingTeams).json().default("[]"))
                .col(ColumnDef::new(Tournaments::Phases).json().default("[]"))
                .col(ColumnDef::new(Tournaments::FinalStandings).json().default("[]"))
                .col(ColumnDef::new(Tournaments::PrizePool).json().default("{}"))
                .col(ColumnDef::new(Tournaments::Statistics).json().default("{}"))
                .col(ColumnDef::new(Tournaments::Awards).json().default("[]"))
                .col(ColumnDef::new(Tournaments::Media).json().default("{}"))
                .col(ColumnDef::new(Tournaments::StreamLinks).json().default("[]"))
                .col(ColumnDef::new(Tournaments::SocialMedia).json().default("{}"))
                .col(ColumnDef::new(Tournaments::Description).text())
                .col(ColumnDef::new(Tournaments::RulesetDocument).text())
                .col(ColumnDef::new(Tournaments::WebsiteLink).text())
                .col(ColumnDef::new(Tournaments::GameSettings).json().default("{}"))
                .col(ColumnDef::new(Tournaments::Visibility).string_len(20).default("public"))
                .col(ColumnDef::new(Tournaments::Featured).boolean().default(false))
                .col(ColumnDef::new(Tournaments::Verified).boolean().default(false))
                .col(ColumnDef::new(Tournaments::ParentSeries).uuid())
                .col(ColumnDef::new(Tournaments::QualifiesFor).json().default("[]"))
                .col(ColumnDef::new(Tournaments::Tags).array(ColumnType::Text))
                .col(ColumnDef::new(Tournaments::Notes).text())
                .col(ColumnDef::new(Tournaments::ExternalIds).json().default("{}"))
                .col(ColumnDef::new(Tournaments::ApprovalStatus).custom(ApprovalStatus::Table).default("not_applicable"))
                .col(ColumnDef::new(Tournaments::SubmittedBy).uuid())
                .col(ColumnDef::new(Tournaments::SubmittedAt).timestamp_with_time_zone())
                .col(ColumnDef::new(Tournaments::ApprovedBy).uuid())
                .col(ColumnDef::new(Tournaments::ApprovedAt).timestamp_with_time_zone())
                .col(ColumnDef::new(Tournaments::RejectedBy).uuid())
                .col(ColumnDef::new(Tournaments::RejectedAt).timestamp_with_time_zone())
                .col(ColumnDef::new(Tournaments::RejectionReason).text())
                .col(ColumnDef::new(Tournaments::PendingInvitations).json().default("[]"))
                .col(ColumnDef::new(Tournaments::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Tournaments::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .to_owned(),
        ).await
    }

    async fn create_player_game_stats_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(PlayerGameStats::Table)
                .if_not_exists()
                .col(ColumnDef::new(PlayerGameStats::Id).uuid().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(PlayerGameStats::PlayerId).uuid().not_null())
                .col(ColumnDef::new(PlayerGameStats::GameType).custom(GameType::Table).not_null())
                .col(ColumnDef::new(PlayerGameStats::RankTier).string_len(50))
                .col(ColumnDef::new(PlayerGameStats::BattlesPlayed).integer().default(0))
                .col(ColumnDef::new(PlayerGameStats::Wins).integer().default(0))
                .col(ColumnDef::new(PlayerGameStats::Kills).integer().default(0))
                .col(ColumnDef::new(PlayerGameStats::GameSpecificStats).json().default("{}"))
                .col(ColumnDef::new(PlayerGameStats::LastUpdated).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .to_owned(),
        ).await
    }

    async fn create_battles_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Battles::Table)
                .if_not_exists()
                .col(ColumnDef::new(Battles::Id).uuid().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(Battles::BattleNumber).integer().not_null())
                .col(ColumnDef::new(Battles::Tournament).uuid().not_null())
                .col(ColumnDef::new(Battles::TournamentPhase).string_len(200))
                .col(ColumnDef::new(Battles::ScheduledStartTime).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Battles::Status).custom(BattleStatus::Table).default("scheduled"))
                .col(ColumnDef::new(Battles::Map).string_len(50))
                .col(ColumnDef::new(Battles::ParticipatingGroups).array(ColumnType::Text))
                .col(ColumnDef::new(Battles::ParticipatingTeams).json().default("[]"))
                .col(ColumnDef::new(Battles::BattleStats).json().default("{}"))
                .col(ColumnDef::new(Battles::StreamUrls).json().default("[]"))
                .col(ColumnDef::new(Battles::RoomCredentials).json().default("{}"))
                .col(ColumnDef::new(Battles::PointsSystem).json().default("{}"))
                .col(ColumnDef::new(Battles::Tags).array(ColumnType::Text))
                .col(ColumnDef::new(Battles::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Battles::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .to_owned(),
        ).await
    }

    async fn create_tournament_teams_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(TournamentTeams::Table)
                .if_not_exists()
                .col(ColumnDef::new(TournamentTeams::Id).uuid().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(TournamentTeams::TournamentId).uuid().not_null())
                .col(ColumnDef::new(TournamentTeams::TeamId).uuid().not_null())
                .col(ColumnDef::new(TournamentTeams::QualifiedThrough).string_len(50))
                .col(ColumnDef::new(TournamentTeams::CurrentStage).string_len(100))
                .col(ColumnDef::new(TournamentTeams::TotalTournamentPoints).integer().default(0))
                .col(ColumnDef::new(TournamentTeams::TotalTournamentKills).integer().default(0))
                .col(ColumnDef::new(TournamentTeams::FinalPlacement).integer())
                .col(ColumnDef::new(TournamentTeams::PrizeAmount).decimal_len(15, 2))
                .col(ColumnDef::new(TournamentTeams::JoinedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .to_owned(),
        ).await
    }

    async fn create_tournament_team_invites_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(TournamentTeamInvites::Table)
                .if_not_exists()
                .col(ColumnDef::new(TournamentTeamInvites::Id).uuid().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(TournamentTeamInvites::Tournament).uuid().not_null())
                .col(ColumnDef::new(TournamentTeamInvites::Team).uuid().not_null())
                .col(ColumnDef::new(TournamentTeamInvites::Phase).string_len(200).not_null())
                .col(ColumnDef::new(TournamentTeamInvites::Organizer).uuid().not_null())
                .col(ColumnDef::new(TournamentTeamInvites::Status).string_len(20).default("pending"))
                .col(ColumnDef::new(TournamentTeamInvites::Message).text())
                .col(ColumnDef::new(TournamentTeamInvites::ExpiresAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(TournamentTeamInvites::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .col(ColumnDef::new(TournamentTeamInvites::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .to_owned(),
        ).await
    }

    async fn create_transactions_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Transactions::Table)
                .if_not_exists()
                .col(ColumnDef::new(Transactions::Id).uuid().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(Transactions::PlayerId).uuid().not_null())
                .col(ColumnDef::new(Transactions::TournamentId).uuid())
                .col(ColumnDef::new(Transactions::TransactionType).string_len(50).not_null())
                .col(ColumnDef::new(Transactions::Amount).decimal_len(15, 2).not_null())
                .col(ColumnDef::new(Transactions::Currency).string_len(3).default("INR"))
                .col(ColumnDef::new(Transactions::Status).string_len(20).default("pending"))
                .col(ColumnDef::new(Transactions::Description).text())
                .col(ColumnDef::new(Transactions::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Transactions::ProcessedAt).timestamp_with_time_zone())
                .to_owned(),
        ).await
    }

    async fn create_rewards_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Rewards::Table)
                .if_not_exists()
                .col(ColumnDef::new(Rewards::Id).uuid().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                .col(ColumnDef::new(Rewards::Name).string_len(255).not_null())
                .col(ColumnDef::new(Rewards::Points).integer().not_null())
                .col(ColumnDef::new(Rewards::Description).text().default(""))
                .col(ColumnDef::new(Rewards::Image).text().default(""))
                .col(ColumnDef::new(Rewards::IsActive).boolean().default(true))
                .col(ColumnDef::new(Rewards::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .col(ColumnDef::new(Rewards::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                .to_owned(),
        ).await
    }

    async fn add_foreign_keys(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        // Players -> Teams
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_players_team")
                .from(Players::Table, Players::TeamId)
                .to(Teams::Table, Teams::Id)
                .on_delete(ForeignKeyAction::SetNull)
                .to_owned(),
        ).await?;

        // Teams -> Players (captain)
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_teams_captain")
                .from(Teams::Table, Teams::Captain)
                .to(Players::Table, Players::Id)
                .on_delete(ForeignKeyAction::SetNull)
                .to_owned(),
        ).await?;

        // Teams -> Organizations
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_teams_organization")
                .from(Teams::Table, Teams::OrganizationId)
                .to(Organizations::Table, Organizations::Id)
                .on_delete(ForeignKeyAction::SetNull)
                .to_owned(),
        ).await?;

        // PlayerGameStats -> Players
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_player_game_stats_player")
                .from(PlayerGameStats::Table, PlayerGameStats::PlayerId)
                .to(Players::Table, Players::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        ).await?;

        // Battles -> Tournaments
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_battles_tournament")
                .from(Battles::Table, Battles::Tournament)
                .to(Tournaments::Table, Tournaments::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        ).await?;

        // TournamentTeams -> Tournaments
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_tournament_teams_tournament")
                .from(TournamentTeams::Table, TournamentTeams::TournamentId)
                .to(Tournaments::Table, Tournaments::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        ).await?;

        // TournamentTeams -> Teams
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_tournament_teams_team")
                .from(TournamentTeams::Table, TournamentTeams::TeamId)
                .to(Teams::Table, Teams::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        ).await?;

        // TournamentTeamInvites -> Tournaments
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_tournament_team_invites_tournament")
                .from(TournamentTeamInvites::Table, TournamentTeamInvites::Tournament)
                .to(Tournaments::Table, Tournaments::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        ).await?;

        // TournamentTeamInvites -> Teams
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_tournament_team_invites_team")
                .from(TournamentTeamInvites::Table, TournamentTeamInvites::Team)
                .to(Teams::Table, Teams::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        ).await?;

        // TournamentTeamInvites -> Organizations
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_tournament_team_invites_organizer")
                .from(TournamentTeamInvites::Table, TournamentTeamInvites::Organizer)
                .to(Organizations::Table, Organizations::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        ).await?;

        // Transactions -> Players
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_transactions_player")
                .from(Transactions::Table, Transactions::PlayerId)
                .to(Players::Table, Players::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        ).await?;

        // Transactions -> Tournaments
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_transactions_tournament")
                .from(Transactions::Table, Transactions::TournamentId)
                .to(Tournaments::Table, Tournaments::Id)
                .on_delete(ForeignKeyAction::SetNull)
                .to_owned(),
        ).await?;

        Ok(())
    }

    async fn create_indexes(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        // Players indexes
        manager.create_index(
            Index::create()
                .name("idx_players_game_rating")
                .table(Players::Table)
                .col(Players::PrimaryGame)
                .col((Players::AegisRating, IndexOrder::Desc))
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .name("idx_players_team")
                .table(Players::Table)
                .col(Players::TeamId)
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .name("idx_players_email")
                .table(Players::Table)
                .col(Players::Email)
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .name("idx_players_username")
                .table(Players::Table)
                .col(Players::Username)
                .to_owned(),
        ).await?;

        // Teams indexes
        manager.create_index(
            Index::create()
                .name("idx_teams_game_status")
                .table(Teams::Table)
                .col(Teams::PrimaryGame)
                .col(Teams::Status)
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .name("idx_teams_captain")
                .table(Teams::Table)
                .col(Teams::Captain)
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .name("idx_teams_rating")
                .table(Teams::Table)
                .col((Teams::AegisRating, IndexOrder::Desc))
                .to_owned(),
        ).await?;

        // Tournaments indexes
        manager.create_index(
            Index::create()
                .name("idx_tournaments_game_status")
                .table(Tournaments::Table)
                .col(Tournaments::GameTitle)
                .col(Tournaments::Status)
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .name("idx_tournaments_dates")
                .table(Tournaments::Table)
                .col(Tournaments::StartDate)
                .col(Tournaments::EndDate)
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .name("idx_tournaments_featured")
                .table(Tournaments::Table)
                .col(Tournaments::Featured)
                .to_owned(),
        ).await?;

        // Battles indexes
        manager.create_index(
            Index::create()
                .name("idx_battles_tournament")
                .table(Battles::Table)
                .col(Battles::Tournament)
                .col(Battles::BattleNumber)
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .name("idx_battles_status")
                .table(Battles::Table)
                .col(Battles::Status)
                .col(Battles::ScheduledStartTime)
                .to_owned(),
        ).await?;

        // PlayerGameStats indexes
        manager.create_index(
            Index::create()
                .name("idx_player_stats_game")
                .table(PlayerGameStats::Table)
                .col(PlayerGameStats::GameType)
                .col(PlayerGameStats::PlayerId)
                .to_owned(),
        ).await?;

        // Transactions indexes
        manager.create_index(
            Index::create()
                .name("idx_transactions_player")
                .table(Transactions::Table)
                .col(Transactions::PlayerId)
                .col((Transactions::CreatedAt, IndexOrder::Desc))
                .to_owned(),
        ).await?;

        // TournamentTeams unique constraint
        manager.create_index(
            Index::create()
                .name("idx_tournament_teams_unique")
                .table(TournamentTeams::Table)
                .col(TournamentTeams::TournamentId)
                .col(TournamentTeams::TeamId)
                .unique()
                .to_owned(),
        ).await?;

        Ok(())
    }
}

// ENUM definitions
#[derive(Iden)]
enum GameType {
    Table,
    BGMI, VALORANT, CS2, APEX, FORTNITE, LOL, DOTA2, PUBG, COD,
}

#[derive(Iden)]
enum TournamentStatus {
    Table,
    Announced, RegistrationOpen, RegistrationClosed, 
    InProgress, Completed, Cancelled, Postponed,
}

#[derive(Iden)]
enum TeamStatus {
    Table,
    Active, Inactive, Disbanded, LookingForPlayers,
}

#[derive(Iden)]
enum BattleStatus {
    Table,
    Scheduled, InProgress, Completed, Cancelled,
}

#[derive(Iden)]
enum ApprovalStatus {
    Table,
    Pending, Approved, Rejected, NotApplicable,
}

#[derive(Iden)]
enum AdminRole {
    Table,
    SuperAdmin, Admin, Moderator,
}

// Table definitions
#[derive(Iden)]
enum Players {
    Table,
    Id, CognitoSub, Username, InGameName, RealName, Email, Password,
    ResetPasswordToken, ResetPasswordExpiry, Verified, Country, Bio,
    ProfilePicture, PrimaryGame, Earnings, InGameRole, Location, Age,
    Languages, AegisRating, TournamentsPlayed, BattlesPlayed,
    QualifiedEvents, QualifiedEventDetails, TeamStatus, TeamId,
    Availability, DiscordTag, Twitch, Youtube, Twitter,
    ProfileVisibility, CardTheme, Coins, LastCheckIn, CheckInStreak,
    TotalCheckIns, CreatedAt, UpdatedAt,
}

#[derive(Iden)]
enum Organizations {
    Table,
    Id, CognitoSub, OrgName, OwnerName, Email, GoogleId, Password,
    Country, Headquarters, Description, Logo, EstablishedDate,
    ActiveGames, TotalEarnings, ContactPhone, Discord, Twitter,
    Twitch, Youtube, Website, Linkedin, ProfileVisibility,
    ApprovalStatus, ApprovedBy, ApprovalDate, RejectionReason,
    EmailVerified, VerificationToken, CreatedAt, UpdatedAt,
}

#[derive(Iden)]
enum Teams {
    Table,
    Id, TeamName, TeamTag, Logo, Captain, PrimaryGame, Region,
    Country, Bio, EstablishedDate, TotalEarnings, AegisRating,
    OrganizationId, Discord, Twitter, Twitch, Youtube, Website,
    ProfileVisibility, Status, LookingForPlayers, OpenRoles,
    CreatedAt, UpdatedAt,
}

#[derive(Iden)]
enum Admins {
    Table,
    Id, Username, Email, Password, Role, Permissions, IsActive,
    LastLogin, LoginAttempts, LockUntil, CreatedAt, UpdatedAt,
}

#[derive(Iden)]
enum Tournaments {
    Table,
    Id, TournamentName, ShortName, Slug, GameTitle, Tier, Region,
    SubRegion, Organizer, Sponsors, AnnouncementDate, IsOpenForAll,
    RegistrationStartDate, RegistrationEndDate, StartDate, EndDate,
    Status, Format, FormatDetails, Slots, ParticipatingTeams, Phases,
    FinalStandings, PrizePool, Statistics, Awards, Media, StreamLinks,
    SocialMedia, Description, RulesetDocument, WebsiteLink, GameSettings,
    Visibility, Featured, Verified, ParentSeries, QualifiesFor, Tags,
    Notes, ExternalIds, ApprovalStatus, SubmittedBy, SubmittedAt,
    ApprovedBy, ApprovedAt, RejectedBy, RejectedAt, RejectionReason,
    PendingInvitations, CreatedAt, UpdatedAt,
}

#[derive(Iden)]
enum PlayerGameStats {
    Table,
    Id, PlayerId, GameType, RankTier, BattlesPlayed, Wins, Kills,
    GameSpecificStats, LastUpdated,
}

#[derive(Iden)]
enum Battles {
    Table,
    Id, BattleNumber, Tournament, TournamentPhase, ScheduledStartTime,
    Status, Map, ParticipatingGroups, ParticipatingTeams, BattleStats,
    StreamUrls, RoomCredentials, PointsSystem, Tags, CreatedAt, UpdatedAt,
}

#[derive(Iden)]
enum TournamentTeams {
    Table,
    Id, TournamentId, TeamId, QualifiedThrough, CurrentStage,
    TotalTournamentPoints, TotalTournamentKills, FinalPlacement,
    PrizeAmount, JoinedAt,
}

#[derive(Iden)]
enum TournamentTeamInvites {
    Table,
    Id, Tournament, Team, Phase, Organizer, Status, Message,
    ExpiresAt, CreatedAt, UpdatedAt,
}

#[derive(Iden)]
enum Transactions {
    Table,
    Id, PlayerId, TournamentId, TransactionType, Amount, Currency,
    Status, Description, CreatedAt, ProcessedAt,
}

#[derive(Iden)]
enum Rewards {
    Table,
    Id, Name, Points, Description, Image, IsActive, CreatedAt, UpdatedAt,
}
