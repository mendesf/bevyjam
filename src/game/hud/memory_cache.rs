use bevy::prelude::*;

use super::bitmap_font::{BitmapNumberConfig, BitmapNumberValue};

#[derive(Component)]
pub struct MemoryCache;

#[derive(Component)]
pub struct MemoryCacheBackground;

#[derive(Component, Reflect)]
pub struct MemoryCacheValue(pub u8);

impl BitmapNumberValue for MemoryCacheValue {
    fn get_value(&self) -> u8 {
        self.0
    }
}

#[derive(Component)]
pub struct MemoryCacheDigit;

const MEMORY_CACHE_INITIAL_VALUE: u8 = 9;

pub(super) fn spawn_memory_cache(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        MemoryCache {},
        SpatialBundle {
            transform: Transform::from_xyz(39.0, -39.0, 0.0),
            ..default()
        },
    )).with_children(|parent| {
        spawn_memory_cache_value(texture_atlases, asset_server, parent);
        spawn_memory_cache_background(asset_server, parent);
    });
}

fn spawn_memory_cache_value(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    let tile_size = Vec2::new(12.0, 23.0);
    let columns = 10;

    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("textures/UI/MB/UI_MB_Numbers.png"),
        tile_size,
        columns,
        1,
        None,
        None,
    );
    let texture_atlas_handle = &texture_atlases.add(texture_atlas);

    parent.spawn((
        MemoryCacheValue(MEMORY_CACHE_INITIAL_VALUE),
        BitmapNumberConfig {
            tile_size,
            columns,
            row: 0,
            font_spacing: 1.0,
        },
        SpatialBundle {
            transform: Transform::from_xyz(6.0, 2.0, 0.0),
            ..default()
        },
    )).with_children(|parent| {
        for _ in 0..2 {
            parent.spawn((
                MemoryCacheDigit {},
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    ..default()
                }
            ));
        }
    });
}


fn spawn_memory_cache_background(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        MemoryCacheBackground {},
        SpriteBundle {
            texture: asset_server.load("textures/UI/MB/UI_MB_BG.png"),
            ..default()
        },
    ));
}
