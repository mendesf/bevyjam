use bevy::prelude::*;

use crate::game::animation::AnimationPlugin;
use crate::game::hud::HudPlugin;
use crate::game::card::CardPlugin;

mod hud;
mod animation;
mod card;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AnimationPlugin)
            .add_plugin(HudPlugin)
            .add_plugin(CardPlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
