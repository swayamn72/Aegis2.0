pub mod config;
pub mod handlers;
pub mod middleware;
pub mod migration;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod scripts;
pub mod services;
pub mod utils;

use repositories::{ChatRepository, CommunityRepository, DynamoRepository, PostRepository};
use services::{
    AdminService, ApiKeyService, AuditService, AuthService, BattleService, ChatService,
    CommunityService, EmailService, OrganizationService, PlayerGameStatsService, PlayerService,
    PostService, RateLimitService, RewardService, S3Service, SessionService, TeamService,
    TournamentService, TournamentTeamInviteService, TournamentTeamService, TransactionService,
};

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub aws: config::AwsClients,
    pub settings: config::Settings,
    pub auth_service: AuthService,
    pub player_service: PlayerService,
    pub admin_service: AdminService,
    pub organization_service: OrganizationService,
    pub team_service: TeamService,
    pub tournament_service: TournamentService,
    pub tournament_team_service: TournamentTeamService,
    pub tournament_team_invite_service: TournamentTeamInviteService,
    pub battle_service: BattleService,
    pub player_game_stats_service: PlayerGameStatsService,
    pub reward_service: RewardService,
    pub transaction_service: TransactionService,
    pub email_service: EmailService,
    pub chat_service: ChatService,
    pub post_service: PostService,
    pub community_service: CommunityService,
    pub s3_service: S3Service,
    pub session_service: SessionService,
    pub audit_service: AuditService,
    pub rate_limit_service: RateLimitService,
    pub api_key_service: ApiKeyService,
}

impl AppState {
    pub async fn new(
        db: sea_orm::DatabaseConnection,
        aws: config::AwsClients,
        settings: config::Settings,
    ) -> Self {
        let auth_service = AuthService::new(settings.jwt.secret.clone(), settings.jwt.expiration);
        let email_service =
            EmailService::new(settings.email.clone()).expect("Failed to initialize email service");

        // Core user services
        let player_service = PlayerService::new(db.clone(), auth_service.clone());
        let admin_service = AdminService::new(db.clone(), auth_service.clone());
        let organization_service = OrganizationService::new(db.clone(), auth_service.clone());

        // Gaming services - ADD auth_service where needed
        let team_service = TeamService::new(db.clone());
        let tournament_service = TournamentService::new(db.clone());
        let tournament_team_service = TournamentTeamService::new(db.clone());
        let tournament_team_invite_service = TournamentTeamInviteService::new(db.clone());
        let battle_service = BattleService::new(db.clone());
        let player_game_stats_service = PlayerGameStatsService::new(db.clone());
        let reward_service = RewardService::new(db.clone());
        let transaction_service = TransactionService::new(db.clone());

        // Enterprise security services - ADD auth_service
        let session_service = SessionService::new(db.clone());
        let audit_service = AuditService::new(db.clone());
        let rate_limit_service = RateLimitService::new(db.clone());
        let api_key_service = ApiKeyService::new(db.clone(), auth_service.clone());

        // DynamoDB services
        let dynamo_repo = DynamoRepository::new(aws.dynamodb.clone());
        let chat_repo = ChatRepository::new(dynamo_repo.clone());
        let post_repo = PostRepository::new(dynamo_repo.clone());
        let community_repo = CommunityRepository::new(dynamo_repo);

        let chat_service = ChatService::new(chat_repo);
        let post_service = PostService::new(post_repo);
        let community_service = CommunityService::new(community_repo);
        let s3_service = S3Service::new(aws.s3.clone());

        Self {
            db,
            aws,
            settings,
            auth_service,
            player_service,
            admin_service,
            organization_service,
            team_service,
            tournament_service,
            tournament_team_service,
            tournament_team_invite_service,
            battle_service,
            player_game_stats_service,
            reward_service,
            transaction_service,
            email_service,
            chat_service,
            post_service,
            community_service,
            s3_service,
            session_service,
            audit_service,
            rate_limit_service,
            api_key_service,
        }
    }
}
