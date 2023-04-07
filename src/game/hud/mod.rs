use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use memory_cache::*;
use system_integrity::*;
use system_protection::*;

mod system_integrity;
mod memory_cache;
mod system_protection;

#[derive(Component)]
pub struct Hud;

pub struct HudPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct HudSpawnSystemSet;

const HUD_MARGIN: f32 = 16.0;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SystemIntegrityState>()
            .register_type::<SystemIntegrityValue>()
            .register_type::<MemoryCacheValue>()
            .register_type::<SystemProtectionValue>()
            .add_startup_system(spawn_hud)
            .add_system(update_system_integrity_state)
            .add_system(update_system_integrity_background.after(update_system_integrity_state))
            .add_system(update_system_integrity_fan.after(update_system_integrity_state))
            .add_system(update_system_integrity_digits)
            .add_system(update_memory_cache_digits)
            .add_system(update_system_protection_digits);
    }
}

fn spawn_hud(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Hud {},
        SpatialBundle {
            transform: Transform::from_xyz(
                -window.width() / 2.0 + 58.0 + HUD_MARGIN,
                -window.height() / 2.0 + 162.0 + HUD_MARGIN,
                0.0,
            ).with_scale(Vec3::splat(2.0)),
            ..default()
        }
    )).with_children(|parent| {
        spawn_system_integrity(&mut texture_atlases, &asset_server, parent);
        spawn_memory_cache(&mut texture_atlases, &asset_server, parent);
        spawn_system_protection(&mut texture_atlases, &asset_server, parent);
    });
}
