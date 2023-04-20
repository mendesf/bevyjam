use bevy::prelude::*;

use crate::game::animation::AnimationPlugin;
use crate::game::hud::HudPlugin;
use crate::game::card::CardPlugin;
use crate::game::combat::CombatPlugin;

mod hud;
mod animation;
mod card;
mod combat;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AnimationPlugin)
            // .add_plugin(HudPlugin)
            // .add_plugin(CardPlugin)
            .add_plugin(CombatPlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
