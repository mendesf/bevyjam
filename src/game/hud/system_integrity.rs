use bevy::prelude::*;

use crate::game::animation::{AnimationIndices, AnimationTimer};

#[derive(Component)]
pub struct SystemIntegrity;

#[derive(Component)]
pub struct SystemIntegrityBackground;

#[derive(Component)]
pub struct SystemIntegrityFan;

#[derive(Component, Reflect)]
pub struct SystemIntegrityValue(pub u8);

#[derive(Component, Reflect)]
pub struct SystemIntegrityValueDigit;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub(super) enum SystemIntegrityState {
    #[default]
    Absolute,
    Reliable,
    Vulnerable,
    Hacked,
}

const SYSTEM_INTEGRITY_INITIAL_VALUE: u8 = 100;

pub(super) fn spawn_system_integrity(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        SystemIntegrity {},
        SpatialBundle { ..default() },
    )).with_children(|parent| {
        spawn_system_integrity_value(texture_atlases, asset_server, parent);
        spawn_system_integrity_background(texture_atlases, asset_server, parent);
        spawn_system_integrity_fan(texture_atlases, asset_server, parent);
    });
}

fn spawn_system_integrity_value(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    let texture_atlas = &TextureAtlas::from_grid(
        asset_server.load("textures/UI/SI/UI_SI_Numbers.png"),
        Vec2::new(6.0, 11.0),
        10,
        3,
        None,
        None,
    );

    parent.spawn((
        SystemIntegrityValue(SYSTEM_INTEGRITY_INITIAL_VALUE),
        SpatialBundle {
            transform: Transform::from_xyz(3.0, 1.5, 0.0),
            ..default()
        },
    )).with_children(|parent| {
        for _ in 0..3 {
            parent.spawn((
                SystemIntegrityValueDigit {},
                SpriteSheetBundle {
                    texture_atlas: texture_atlases.add(texture_atlas.clone()),
                    sprite: TextureAtlasSprite::new(0),
                    ..default()
                }
            ));
        }
    });
}

fn spawn_system_integrity_background(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("textures/UI/SI/UI_SI_BG.png"),
        Vec2::new(58.0, 56.0),
        3,
        1,
        None,
        None,
    );

    parent.spawn((
        SystemIntegrityBackground {},
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture_atlas),
            sprite: TextureAtlasSprite::new(0),
            ..default()
        }
    ));
}

fn spawn_system_integrity_fan(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("textures/UI/SI/UI_SI_Fan.png"),
        Vec2::new(50.0, 52.0),
        13,
        1,
        None,
        None,
    );

    parent.spawn((
        SystemIntegrityFan {},
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture_atlas),
            sprite: TextureAtlasSprite::new(0),
            ..default()
        },
        AnimationIndices { first: 0, last: 2 },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    ));
}

pub(super) fn update_system_integrity_state(
    state: Res<State<SystemIntegrityState>>,
    mut next_state: ResMut<NextState<SystemIntegrityState>>,
    mut query: Query<&SystemIntegrityValue>,
) {
    if let Ok(system_integrity) = query.get_single_mut() {
        match system_integrity.0 {
            51..=100 => {
                if state.0 != SystemIntegrityState::Absolute {
                    next_state.set(SystemIntegrityState::Absolute);
                }
            }
            26..=50 => {
                if state.0 != SystemIntegrityState::Reliable {
                    next_state.set(SystemIntegrityState::Reliable);
                }
            }
            1..=25 => {
                if state.0 != SystemIntegrityState::Vulnerable {
                    next_state.set(SystemIntegrityState::Vulnerable);
                }
            }
            0 => {
                if state.0 != SystemIntegrityState::Hacked {
                    next_state.set(SystemIntegrityState::Hacked);
                }
            }
            _ => ()
        }
    }
}

pub(super) fn update_system_integrity_digits(
    state: Res<State<SystemIntegrityState>>,
    value_query: Query<&SystemIntegrityValue>,
    mut digit_query: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Visibility), With<SystemIntegrityValueDigit>>,
) {
    let value = value_query.get_single().unwrap();
    let size = Vec2::new(6.0, 11.0);
    let columns = 10;
    let spacing: f32 = 1.0;

    let binding = value.0.to_string();
    let mut digits = binding.chars();
    let count = digits.clone().count();
    let width = count as f32 * size.x + (count as f32 - 1.0) * spacing;
    let line = match state.0 {
        SystemIntegrityState::Absolute => 0,
        SystemIntegrityState::Reliable => 1,
        SystemIntegrityState::Vulnerable | SystemIntegrityState::Hacked => 2
    };

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

pub(super) fn update_system_integrity_background(
    state: Res<State<SystemIntegrityState>>,
    mut query: Query<&mut TextureAtlasSprite, With<SystemIntegrityBackground>>,
) {
    if state.is_changed() {
        if let Ok(mut sprite) = query.get_single_mut() {
            sprite.index = match state.0 {
                SystemIntegrityState::Absolute => 0,
                SystemIntegrityState::Reliable => 1,
                SystemIntegrityState::Vulnerable | SystemIntegrityState::Hacked => 2
            }
        }
    }
}

pub(super) fn update_system_integrity_fan(
    state: Res<State<SystemIntegrityState>>,
    mut query: Query<&mut AnimationIndices, With<SystemIntegrityFan>>,
) {
    if state.is_changed() {
        if let Ok(mut animation_indices) = query.get_single_mut() {
            match state.0 {
                SystemIntegrityState::Absolute => {
                    animation_indices.first = 0;
                    animation_indices.last = 2;
                }
                SystemIntegrityState::Reliable => {
                    animation_indices.first = 3;
                    animation_indices.last = 5;
                }
                SystemIntegrityState::Vulnerable => {
                    animation_indices.first = 6;
                    animation_indices.last = 12;
                }
                SystemIntegrityState::Hacked => {
                    animation_indices.first = 12;
                    animation_indices.last = 12;
                }
            }
        }
    }
}
