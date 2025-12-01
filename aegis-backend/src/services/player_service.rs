use crate::handlers::players::{PlayerListQuery, PlayerProfileResponse};
use crate::models::enums::GameType;
use crate::models::postgres::{player, Player};
use crate::services::auth_service::{AuthService, UserType};
use crate::utils::errors::AppError;
use crate::utils::validation::validate_password;
use anyhow::Result;
use sea_orm::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub in_game_name: Option<String>,
    pub real_name: Option<String>,
    pub bio: Option<String>,
    pub age: Option<i32>,
    pub country: Option<String>,
    pub location: Option<String>,
    pub languages: Option<Vec<String>>,
    pub primary_game: Option<GameType>,
    pub in_game_role: Option<Vec<String>>,
    pub availability: Option<String>,
    pub discord_tag: Option<String>,
    pub twitch: Option<String>,
    pub youtube: Option<String>,
    pub twitter: Option<String>,
    pub profile_visibility: Option<String>,
    pub card_theme: Option<String>,
}

#[derive(Clone)]
pub struct PlayerService {
    db: DatabaseConnection,
    auth_service: AuthService,
}

impl PlayerService {
    pub fn new(db: DatabaseConnection, auth_service: AuthService) -> Self {
        Self { db, auth_service }
    }

    pub async fn create_player(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<(player::Model, String), AppError> {
        println!("DEBUG: Starting create_player");
        println!("DEBUG: Email: {}, Username: {}", email, username);

        // Validate password
        validate_password(&password)?;
        println!("DEBUG: Password validation passed");

        // Check for existing email
        println!("DEBUG: Checking for existing email");
        let existing_email = Player::find()
            .filter(player::Column::Email.eq(&email))
            .one(&self.db)
            .await?;

        if existing_email.is_some() {
            println!("DEBUG: Email already exists");
            return Err(AppError::Validation("Email already exists".to_string()));
        }
        println!("DEBUG: Email check passed");

        // Check for existing username
        println!("DEBUG: Checking for existing username");
        let existing_username = Player::find()
            .filter(player::Column::Username.eq(&username))
            .one(&self.db)
            .await?;

        if existing_username.is_some() {
            println!("DEBUG: Username already exists");
            return Err(AppError::Validation("Username already exists".to_string()));
        }
        println!("DEBUG: Username check passed");

        // Hash password
        println!("DEBUG: Hashing password");
        let hashed_password = match self.auth_service.hash_password(&password) {
            Ok(hash) => {
                println!("DEBUG: Password hashed successfully");
                hash
            }
            Err(e) => {
                println!("DEBUG: Password hashing failed: {:?}", e);
                return Err(e);
            }
        };

        let now = chrono::Utc::now();
        println!("DEBUG: Creating ActiveModel");

        let new_player = player::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(username),
            in_game_name: Set(None),
            real_name: Set(None),
            email: Set(email),
            password: Set(hashed_password),
            verified: Set(false),
            country: Set(None),
            bio: Set(String::new()),
            profile_picture: Set(String::new()),
            primary_game: NotSet,
            earnings: Set(rust_decimal::Decimal::ZERO),
            in_game_role: Set(vec![]),
            location: Set(None),
            age: Set(None),
            languages: Set(vec![]),
            aegis_rating: Set(1000),
            tournaments_played: Set(0),
            battles_played: Set(0),
            qualified_events: Set(false),
            qualified_event_details: Set(vec![]),
            team_status: Set(None),
            team_id: Set(None),
            availability: Set(None),
            discord_tag: Set(String::new()),
            twitch: Set(String::new()),
            youtube: Set(String::new()),
            twitter: Set(String::new()),
            profile_visibility: Set("public".to_string()),
            card_theme: Set("default".to_string()),
            coins: Set(0),
            last_check_in: Set(None),
            check_in_streak: Set(0),
            total_check_ins: Set(0),
            created_at: Set(now),
            updated_at: Set(now),
        };
        println!("DEBUG: ActiveModel created successfully");

        // Attempt database insert with detailed error handling
        println!("DEBUG: Attempting database insert");
        let player = match new_player.insert(&self.db).await {
            Ok(p) => {
                println!("DEBUG: Database insert successful! Player ID: {}", p.id);
                p
            }
            Err(e) => {
                println!("DEBUG: Database insert failed with detailed error:");
                println!("DEBUG: Error type: {:?}", e);
                println!("DEBUG: Error message: {}", e);

                // Check for specific database errors
                match &e {
                    DbErr::Exec(runtime_err) => {
                        println!("DEBUG: Execution error: {:?}", runtime_err);
                    }
                    DbErr::Query(runtime_err) => {
                        println!("DEBUG: Query error: {:?}", runtime_err);
                    }
                    DbErr::Conn(runtime_err) => {
                        println!("DEBUG: Connection error: {:?}", runtime_err);
                    }
                    DbErr::Type(type_err) => {
                        println!("DEBUG: Type conversion error: {}", type_err);
                    }
                    _ => {
                        println!("DEBUG: Other database error: {:?}", e);
                    }
                }

                return Err(AppError::Database(e));
            }
        };

        // Generate JWT token
        println!("DEBUG: Generating JWT token for player ID: {}", player.id);
        let token = match self.auth_service.generate_jwt(
            player.id,
            UserType::Player,
            None,
            Uuid::new_v4().to_string(),
        ) {
            Ok(t) => {
                println!(
                    "DEBUG: JWT token generated successfully (length: {})",
                    t.len()
                );
                t
            }
            Err(e) => {
                println!("DEBUG: JWT token generation failed: {:?}", e);
                return Err(e);
            }
        };

        println!("DEBUG: create_player completed successfully");
        Ok((player, token))
    }

    pub async fn authenticate(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<(player::Model, String)>, AppError> {
        let player = Player::find()
            .filter(player::Column::Email.eq(email))
            .one(&self.db)
            .await?;

        match player {
            Some(p) => {
                if self.auth_service.verify_password(&password, &p.password)? {
                    let token = self.auth_service.generate_jwt(
                        p.id,
                        UserType::Player,
                        None,
                        Uuid::new_v4().to_string(),
                    )?;
                    Ok(Some((p, token)))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<player::Model>, AppError> {
        Ok(Player::find_by_id(id).one(&self.db).await?)
    }

    // pub async fn send_verification_email(&self, player_id: Uuid) -> Result<String, AppError> {
    //     let player = self.get_by_id(player_id).await?.ok_or(AppError::NotFound)?;

    //     if player.verified {
    //         return Err(AppError::Validation("Email already verified".to_string()));
    //     }

    //     let verification_token = uuid::Uuid::new_v4().to_string();
    //     let expiry = chrono::Utc::now() + chrono::Duration::hours(24);

    //     let mut player_update: player::ActiveModel = player.into();
    //     player_update.reset_password_token = Set(Some(format!("verify_{}", verification_token)));
    //     player_update.reset_password_expiry = Set(Some(expiry));

    //     Player::update(player_update).exec(&self.db).await?;
    //     Ok(verification_token)
    // }

    // pub async fn verify_email_by_token(&self, token: String) -> Result<bool, AppError> {
    //     let now = chrono::Utc::now();
    //     let verification_token = format!("verify_{}", token);

    //     let player = Player::find()
    //         .filter(player::Column::ResetPasswordToken.eq(Some(verification_token)))
    //         .filter(player::Column::ResetPasswordExpiry.gt(now))
    //         .one(&self.db)
    //         .await?;

    //     if let Some(p) = player {
    //         let mut player_update: player::ActiveModel = p.into();
    //         player_update.verified = Set(true);
    //         player_update.reset_password_token = Set(None);
    //         player_update.reset_password_expiry = Set(None);
    //         player_update.updated_at = Set(now);

    //         Player::update(player_update).exec(&self.db).await?;
    //         Ok(true)
    //     } else {
    //         Ok(false)
    //     }
    // }

    // pub async fn request_password_reset(&self, email: String) -> Result<Option<String>, AppError> {
    //     let player = Player::find()
    //         .filter(player::Column::Email.eq(email))
    //         .one(&self.db)
    //         .await?;

    //     if let Some(p) = player {
    //         let reset_token = uuid::Uuid::new_v4().to_string();
    //         let expiry = chrono::Utc::now() + chrono::Duration::hours(1);

    //         let mut player_update: player::ActiveModel = p.into();
    //         player_update.reset_password_token = Set(Some(reset_token.clone()));
    //         player_update.reset_password_expiry = Set(Some(expiry));

    //         Player::update(player_update).exec(&self.db).await?;
    //         Ok(Some(reset_token))
    //     } else {
    //         Ok(None)
    //     }
    // }

    // pub async fn reset_password_with_token(
    //     &self,
    //     token: String,
    //     new_password: String,
    // ) -> Result<bool, AppError> {
    //     validate_password(&new_password)?;

    //     let now = chrono::Utc::now();
    //     let player = Player::find()
    //         .filter(player::Column::ResetPasswordToken.eq(Some(token)))
    //         .filter(player::Column::ResetPasswordExpiry.gt(now))
    //         .one(&self.db)
    //         .await?;

    //     if let Some(p) = player {
    //         let hashed_password = self.auth_service.hash_password(&new_password)?;

    //         let mut player_update: player::ActiveModel = p.into();
    //         player_update.password = Set(hashed_password);
    //         player_update.reset_password_token = Set(None);
    //         player_update.reset_password_expiry = Set(None);
    //         player_update.updated_at = Set(now);

    //         Player::update(player_update).exec(&self.db).await?;
    //         Ok(true)
    //     } else {
    //         Ok(false)
    //     }
    // }

    pub async fn get_by_username(
        &self,
        username: String,
    ) -> Result<Option<player::Model>, AppError> {
        Ok(Player::find()
            .filter(player::Column::Username.eq(username))
            .one(&self.db)
            .await?)
    }

    pub async fn update_profile(
        &self,
        player_id: Uuid,
        update_data: UpdateProfileRequest,
    ) -> Result<player::Model, AppError> {
        let player = Player::find_by_id(player_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active_model: player::ActiveModel = player.into();

        // Update only provided fields
        if let Some(in_game_name) = update_data.in_game_name {
            active_model.in_game_name = Set(Some(in_game_name));
        }
        if let Some(real_name) = update_data.real_name {
            active_model.real_name = Set(Some(real_name));
        }
        if let Some(bio) = update_data.bio {
            active_model.bio = Set(bio);
        }
        if let Some(age) = update_data.age {
            active_model.age = Set(Some(age));
        }
        if let Some(country) = update_data.country {
            active_model.country = Set(Some(country));
        }
        if let Some(location) = update_data.location {
            active_model.location = Set(Some(location));
        }
        if let Some(languages) = update_data.languages {
            active_model.languages = Set(languages);
        }
        if let Some(primary_game) = update_data.primary_game {
            active_model.primary_game = Set(Some(primary_game));
        }
        if let Some(in_game_role) = update_data.in_game_role {
            active_model.in_game_role = Set(in_game_role);
        }
        if let Some(availability) = update_data.availability {
            active_model.availability = Set(Some(availability));
        }
        if let Some(discord_tag) = update_data.discord_tag {
            active_model.discord_tag = Set(discord_tag);
        }
        if let Some(twitch) = update_data.twitch {
            active_model.twitch = Set(twitch);
        }
        if let Some(youtube) = update_data.youtube {
            active_model.youtube = Set(youtube);
        }
        if let Some(twitter) = update_data.twitter {
            active_model.twitter = Set(twitter);
        }
        if let Some(profile_visibility) = update_data.profile_visibility {
            active_model.profile_visibility = Set(profile_visibility);
        }
        if let Some(card_theme) = update_data.card_theme {
            active_model.card_theme = Set(card_theme);
        }

        active_model.updated_at = Set(chrono::Utc::now());

        Ok(Player::update(active_model).exec(&self.db).await?)
    }

    pub async fn list_players(
        &self,
        limit: u64,
        offset: u64,
        query: PlayerListQuery,
    ) -> Result<Vec<PlayerProfileResponse>, AppError> {
        let mut select = Player::find();

        // Enterprise filtering
        if let Some(game) = query.game {
            select = select.filter(player::Column::PrimaryGame.eq(game));
        }

        if let Some(country) = query.country {
            select = select.filter(player::Column::Country.eq(country));
        }

        if query.verified_only.unwrap_or(false) {
            select = select.filter(player::Column::Verified.eq(true));
        }

        // Enterprise sorting
        match query.sort_by.as_deref() {
            Some("rating") => {
                if query.order.as_deref() == Some("asc") {
                    select = select.order_by_asc(player::Column::AegisRating);
                } else {
                    select = select.order_by_desc(player::Column::AegisRating);
                }
            }
            Some("username") => {
                if query.order.as_deref() == Some("desc") {
                    select = select.order_by_desc(player::Column::Username);
                } else {
                    select = select.order_by_asc(player::Column::Username);
                }
            }
            Some("created_at") => {
                if query.order.as_deref() == Some("asc") {
                    select = select.order_by_asc(player::Column::CreatedAt);
                } else {
                    select = select.order_by_desc(player::Column::CreatedAt);
                }
            }
            _ => {
                // Default: sort by rating descending
                select = select.order_by_desc(player::Column::AegisRating);
            }
        }

        let players = select.limit(limit).offset(offset).all(&self.db).await?;

        // Convert to response format
        let player_responses: Vec<PlayerProfileResponse> = players
            .into_iter()
            .map(|player| PlayerProfileResponse {
                id: player.id,
                username: player.username,
                email: player.email,
                verified: player.verified,
                in_game_name: player.in_game_name,
                real_name: player.real_name,
                bio: player.bio,
                profile_picture: player.profile_picture,
                age: player.age,
                country: player.country,
                location: player.location,
                languages: player.languages,
                primary_game: player.primary_game,
                in_game_role: player.in_game_role,
                aegis_rating: player.aegis_rating,
                tournaments_played: player.tournaments_played,
                battles_played: player.battles_played,
                earnings: player.earnings,
                team_id: player.team_id,
                team_status: player.team_status,
                availability: player.availability,
                discord_tag: player.discord_tag,
                twitch: player.twitch,
                youtube: player.youtube,
                twitter: player.twitter,
                profile_visibility: player.profile_visibility,
                card_theme: player.card_theme,
                coins: player.coins,
                check_in_streak: player.check_in_streak,
                total_check_ins: player.total_check_ins,
                last_check_in: player.last_check_in,
                created_at: player.created_at,
                updated_at: player.updated_at,
            })
            .collect();

        Ok(player_responses)
    }
    // Add these methods to PlayerService impl block in player_service.rs

    pub async fn get_by_email(&self, email: String) -> Result<Option<player::Model>, AppError> {
        Ok(Player::find()
            .filter(player::Column::Email.eq(email))
            .one(&self.db)
            .await?)
    }

    pub async fn update_password(
        &self,
        user_id: Uuid,
        hashed_password: String,
    ) -> Result<bool, AppError> {
        if let Some(player) = Player::find_by_id(user_id).one(&self.db).await? {
            let mut player_update: player::ActiveModel = player.into();
            player_update.password = Set(hashed_password);
            player_update.updated_at = Set(chrono::Utc::now());
            Player::update(player_update).exec(&self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn verify_email(&self, user_id: Uuid) -> Result<bool, AppError> {
        if let Some(player) = Player::find_by_id(user_id).one(&self.db).await? {
            let mut player_update: player::ActiveModel = player.into();
            player_update.verified = Set(true);
            player_update.updated_at = Set(chrono::Utc::now());
            Player::update(player_update).exec(&self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
