use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bitmap_font::*;
use memory_cache::*;
use system_integrity::*;
use system_protection::*;

mod system_integrity;
mod memory_cache;
mod system_protection;
mod bitmap_font;

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
            .add_system(update_system_integrity_animation.after(update_system_integrity_state))
            .add_system(update_system_integrity_color.after(update_system_integrity_state))
            .add_system(update_bitmap_number_digits::<SystemIntegrityValue, SystemIntegrityDigit>)
            .add_system(update_bitmap_number_digits::<MemoryCacheValue, MemoryCacheDigit>)
            .add_system(update_bitmap_number_digits::<SystemProtectionValue, SystemProtectionDigit>);
    }
}

fn spawn_hud(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let parent = commands.spawn((
        Hud {},
        SpatialBundle {
            transform: Transform::from_xyz(
                -window.width() / 2.0 + 58.0 + HUD_MARGIN,
                -window.height() / 2.0 + 162.0 + HUD_MARGIN,
                0.0,
            ).with_scale(Vec3::splat(2.0)),
            ..default()
        }
    )).id();

    commands.entity(parent).with_children(|parent| {
        spawn_system_integrity(&mut texture_atlases, &asset_server, parent);
        spawn_memory_cache(&mut texture_atlases, &asset_server, parent);
        spawn_system_protection(&mut texture_atlases, &asset_server, parent);
    });
}
