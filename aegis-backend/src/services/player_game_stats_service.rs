use crate::models::enums::GameType;
use crate::models::postgres::{player_game_stats, PlayerGameStats};
use crate::utils::errors::AppError;
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct PlayerGameStatsService {
    db: DatabaseConnection,
}

impl PlayerGameStatsService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_player_stats(
        &self,
        player_id: Uuid,
    ) -> Result<Vec<player_game_stats::Model>, AppError> {
        Ok(PlayerGameStats::find()
            .filter(player_game_stats::Column::PlayerId.eq(player_id))
            .all(&self.db)
            .await?)
    }

    pub async fn get_player_game_stats(
        &self,
        player_id: Uuid,
        game_type: GameType,
    ) -> Result<Option<player_game_stats::Model>, AppError> {
        Ok(PlayerGameStats::find()
            .filter(player_game_stats::Column::PlayerId.eq(player_id))
            .filter(player_game_stats::Column::GameType.eq(game_type))
            .one(&self.db)
            .await?)
    }

    pub async fn update_stats(
        &self,
        player_id: Uuid,
        game_type: GameType,
        wins: Option<i32>,
        kills: Option<i32>,
        battles_played: Option<i32>,
    ) -> Result<player_game_stats::Model, AppError> {
        let existing = self
            .get_player_game_stats(player_id, game_type.clone())
            .await?;

        match existing {
            Some(stats) => {
                let mut update: player_game_stats::ActiveModel = stats.into();
                if let Some(w) = wins {
                    update.wins = Set(w);
                }
                if let Some(k) = kills {
                    update.kills = Set(k);
                }
                if let Some(b) = battles_played {
                    update.battles_played = Set(b);
                }
                update.last_updated = Set(chrono::Utc::now());

                Ok(PlayerGameStats::update(update).exec(&self.db).await?)
            }
            None => {
                let new_stats = player_game_stats::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    player_id: Set(player_id),
                    game_type: Set(game_type),
                    wins: Set(wins.unwrap_or(0)),
                    kills: Set(kills.unwrap_or(0)),
                    battles_played: Set(battles_played.unwrap_or(0)),
                    last_updated: Set(chrono::Utc::now()),
                    ..Default::default()
                };

                Ok(new_stats.insert(&self.db).await?)
            }
        }
    }
}
