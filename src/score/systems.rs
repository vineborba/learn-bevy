use bevy::prelude::*;

use crate::root::GameOverMessage;

use super::resources::{HighScores, Score};

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.into_inner().value);
    }
}

pub fn update_high_scores(
    mut game_over_message_reader: MessageReader<GameOverMessage>,
    mut high_scores: ResMut<HighScores>,
) {
    for message in game_over_message_reader.read() {
        high_scores.scores.push(("Player".into(), message.score))
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Schores: {:?}", high_scores);
    }
}
