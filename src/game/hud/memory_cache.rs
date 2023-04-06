use bevy::prelude::*;

#[derive(Component)]
pub struct MemoryCache;

pub(super) fn spawn_memory_cache(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        MemoryCache {},
        SpriteBundle {
            texture: asset_server.load("textures/UI/MB/UI_MB_BG.png"),
            transform: Transform::from_xyz(39.0, -39.0, 0.0),
            ..default()
        },
    ));
}