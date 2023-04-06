use bevy::prelude::*;

#[derive(Component)]
pub struct SystemProtection;

pub(super) fn spawn_system_protection(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        SystemProtection {},
        SpriteBundle {
            texture: asset_server.load("textures/UI/SO/UI_SO_BG.png"),
            transform: Transform::from_xyz(30.5, 22.5, 1.0),
            ..default()
        },
    ));
}