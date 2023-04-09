use bevy::prelude::*;

use super::bitmap_font::{BitmapNumberConfig, BitmapNumberValue};

#[derive(Component)]
pub struct SystemProtection;

#[derive(Component)]
pub struct SystemProtectionBackground;

#[derive(Component, Reflect)]
pub struct SystemProtectionValue(pub u8);

impl BitmapNumberValue for SystemProtectionValue {
    fn get_value(&self) -> u8 {
        self.0
    }
}

#[derive(Component)]
pub struct SystemProtectionDigit;

const SYSTEM_PROTECTION_INITIAL_VALUE: u8 = 0;

pub(super) fn spawn_system_protection(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        SystemProtection {},
        SpatialBundle {
            transform: Transform::from_xyz(30.5, 22.5, 1.0),
            ..default()
        },
    )).with_children(|parent| {
        spawn_system_protection_value(texture_atlases, asset_server, parent);
        spawn_system_protection_background(asset_server, parent);
    });
}

fn spawn_system_protection_value(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    let tile_size = Vec2::new(12.0, 18.0);
    let columns = 10;

    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("textures/UI/SO/UI_SO_Numbers.png"),
        tile_size,
        columns,
        1,
        None,
        None,
    );
    let texture_atlas_handle = &texture_atlases.add(texture_atlas);

    parent.spawn((
        SystemProtectionValue(SYSTEM_PROTECTION_INITIAL_VALUE),
        BitmapNumberConfig {
            tile_size,
            columns,
            row: 0,
            font_spacing: 1.0,
        },
        SpatialBundle {
            transform: Transform::from_xyz(3.5, 2.5, 1.0),
            ..default()
        },
    )).with_children(|parent| {
        for _ in 0..2 {
            parent.spawn((
                SystemProtectionDigit {},
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    ..default()
                }
            ));
        }
    });
}

fn spawn_system_protection_background(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        SystemProtectionBackground {},
        SpriteBundle {
            texture: asset_server.load("textures/UI/SO/UI_SO_BG.png"),
            ..default()
        },
    ));
}
