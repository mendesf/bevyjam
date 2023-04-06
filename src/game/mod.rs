use bevy::prelude::*;

use crate::game::animation::AnimationPlugin;
use crate::game::hud::HudPlugin;

mod hud;
mod animation;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AnimationPlugin)
            .add_plugin(HudPlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
