use bevy::prelude::*;

#[derive(Component)]
pub struct SystemProtection;

#[derive(Component)]
pub struct SystemProtectionBackground;

#[derive(Component, Reflect)]
pub struct SystemProtectionValue(pub u8);

#[derive(Component)]
pub struct SystemProtectionValueDigit;

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
    let texture_atlas = &TextureAtlas::from_grid(
        asset_server.load("textures/UI/SO/UI_SO_Numbers.png"),
        Vec2::new(12.0, 18.0),
        10,
        1,
        None,
        None,
    );

    parent.spawn((
        SystemProtectionValue(SYSTEM_PROTECTION_INITIAL_VALUE),
        SpatialBundle {
            transform: Transform::from_xyz(3.5, 2.5, 1.0),
            ..default()
        },
    )).with_children(|parent| {
        for _ in 0..2 {
            parent.spawn((
                SystemProtectionValueDigit {},
                SpriteSheetBundle {
                    texture_atlas: texture_atlases.add(texture_atlas.clone()),
                    sprite: TextureAtlasSprite::new(0),
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

pub(super) fn update_system_protection_digits(
    value_query: Query<&SystemProtectionValue>,
    mut digit_query: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Visibility), With<SystemProtectionValueDigit>>,
) {
    let value = value_query.get_single().unwrap();
    let size = Vec2::new(12.0, 23.0);
    let columns = 10;
    let spacing: f32 = 1.0;

    let binding = value.0.to_string();
    let mut digits = binding.chars();
    let count = digits.clone().count();
    let width = count as f32 * size.x + (count as f32 - 1.0) * spacing;
    let line = 0;

    digit_query.iter_mut().enumerate().for_each(|(i, item)| {
        let (mut transform, mut sprite, mut visibility) = item;

        if let Some(digit_value) = digits.next() {
            transform.translation.x = -width / 2.0 + i as f32 * (size.x + spacing);
            let column: usize = digit_value.to_string().parse().unwrap();
            sprite.index = line * columns + column;
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    });
}