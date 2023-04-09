use bevy::{
    input::{
        ButtonState,
        mouse::MouseButtonInput,
    },
    prelude::*,
    sprite::collide_aabb::collide,
    window::PrimaryWindow,
};
use rand::prelude::*;

pub const CARD_SIZE: Vec2 = Vec2::new(96.0, 144.0);
pub const CARD_MARGIN: f32 = 16.0;
pub const CARD_PADDING: f32 = 1.0;
pub const BATTLEFIELD_NUMBER_OF_CARDS: usize = 7;
pub const PLAYER_HAND_NUMBER_OF_CARDS: usize = 5;


fn area_with(number_of_cards: usize) -> f32 {
    let number_of_cards = number_of_cards as f32;
    number_of_cards * CARD_SIZE.x + (number_of_cards - 1.0) * CARD_PADDING
}

fn card_translation(number_of_cards: usize, position: usize) -> Vec3 {
    let index = position as f32;
    let width = area_with(number_of_cards);
    let x = -width / 2.0 + CARD_SIZE.x / 2.0 + index as f32 * (CARD_SIZE.x + CARD_PADDING);
    info!("card_translation x: {:?}", x);
    Vec3::new(x, 0.0, 1.0)
}


fn build_sprite_bundle(width: f32, translation_y: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(width, CARD_SIZE.y)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., translation_y, 0.0)),
        ..default()
    }
}

fn mouse_translation(position: Vec2, window: &Window, z: f32) -> Vec3 {
    Vec3::new(
        position.x - window.width() / 2.0,
        position.y - window.height() / 2.0,
        z)
}

#[derive(Resource)]
struct MousePosition(Vec2);

impl Default for MousePosition {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Component)]
pub struct CardArea {
    number_of_cards: usize,
}

#[derive(Component)]
pub struct Battlefield;

#[derive(Component)]
pub struct PlayerHand;

#[derive(Component)]
pub struct Card;

#[derive(Component)]
pub struct CardBeingDragged {
    original_translation: Vec3,
}

#[derive(Component, Reflect)]
pub struct CardPlaced;

#[derive(Component, Reflect)]
pub struct CardsOrder(Vec<Entity>);

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>()
            .register_type::<CardsOrder>()
            .add_startup_system(spawn_line_middle_screen)
            .add_startup_system(spawn_battlefield)
            .add_startup_system(spawn_player_hand)
            .add_system(update_mouse_position)
            .add_system(mouse_pressed.after(update_mouse_position))
            .add_system(move_card.after(mouse_pressed))
            .add_system(mouse_released.after(move_card))
            .add_system(reorder_cards.after(mouse_released));
    }
}

fn spawn_line_middle_screen(mut commands: Commands) {
    let width = area_with(BATTLEFIELD_NUMBER_OF_CARDS);

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(width, 1.0)),
                ..default()
            },
            ..default()
        });
}

fn spawn_battlefield(mut commands: Commands) {
    let width = area_with(BATTLEFIELD_NUMBER_OF_CARDS);
    let translation_y = -CARD_SIZE.y / 2.0 - CARD_MARGIN;

    commands.spawn((
        CardArea { number_of_cards: BATTLEFIELD_NUMBER_OF_CARDS },
        CardsOrder(vec![]),
        Battlefield {},
        build_sprite_bundle(width, translation_y)
    ));
}

fn spawn_player_hand(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let width = area_with(PLAYER_HAND_NUMBER_OF_CARDS);
    let translation_y = -window.height() / 2.0 + (CARD_SIZE.y / 2.0) + CARD_MARGIN;

    commands.spawn((
        CardArea { number_of_cards: PLAYER_HAND_NUMBER_OF_CARDS },
        PlayerHand {},
        build_sprite_bundle(width, translation_y),
    )).with_children(|parent| {
        let mut rng = thread_rng();

        for i in 0..PLAYER_HAND_NUMBER_OF_CARDS {
            let translation = card_translation(PLAYER_HAND_NUMBER_OF_CARDS, i);
            parent.spawn((
                Card {},
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)),
                        custom_size: Some(CARD_SIZE),
                        ..default()
                    },
                    transform: Transform::from_translation(translation),
                    ..default()
                }
            ));
        }
    });
}

fn update_mouse_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_position: ResMut<MousePosition>,
) {
    for event in cursor_moved_events.iter() {
        mouse_position.0 = event.position;
    }
}

fn move_card(
    mouse_position: Res<MousePosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut card_query: Query<(&mut Transform, &GlobalTransform), With<CardBeingDragged>>,
) {
    if mouse_position.is_changed() {
        if let Ok((mut transform, global_transform)) = card_query.get_single_mut() {
            let window = window_query.get_single().unwrap();
            let translation = global_transform.translation();
            let delta = mouse_translation(mouse_position.0, window, translation.z + 1.0) - translation;
            info!("delta {:?}", delta);
            transform.translation += delta;
        }
    }
}

pub fn reorder_cards(
    battlefield_query: Query<(&CardArea, &CardsOrder), (With<Battlefield>, Changed<CardsOrder>)>,
    mut card_query: Query<&mut Transform, With<CardPlaced>>,
) {
    if let Ok((card_area, cards_order)) = battlefield_query.get_single() {
        for (position, card_entity) in cards_order.0.iter().enumerate() {
            if let Ok(mut card_transform) = card_query.get_mut(*card_entity) {
                card_transform.translation = card_translation(card_area.number_of_cards, position);
            }
        }
    }
}

fn mouse_pressed(
    mut commands: Commands,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    // mouse_button_input: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    card_query: Query<(Entity, &Transform, &GlobalTransform), (With<Card>, Without<CardBeingDragged>)>,
) {
    let window = window_query.get_single().unwrap();

    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            // if mouse_button_input.pressed(MouseButton::Left) {
            let mouse_size = Vec2::new(1.0, 1.0);

            for (entity, transform, global_transform) in card_query.iter() {
                let card_pos = global_transform.translation();
                let mouse_pos = mouse_translation(mouse_position.0, window, card_pos.z);
                let collision = collide(
                    card_pos,
                    CARD_SIZE,
                    mouse_pos,
                    mouse_size,
                );

                if let Some(collision) = collision {
                    info!("card collision {:?}", collision);
                    commands.entity(entity).insert(CardBeingDragged {
                        original_translation: transform.translation,
                    });
                }
            }
        }
    }
}


fn mouse_released(
    mut commands: Commands,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut area_query: Query<(Entity, &Sprite, &Transform, &GlobalTransform, &CardArea, &mut CardsOrder), With<Battlefield>>,
    area_children_query: Query<&Children, With<Battlefield>>,
    mut card_query: Query<(Entity, &mut Transform, &GlobalTransform, &CardBeingDragged), (Without<Battlefield>, Without<CardPlaced>)>,
    mut card_query_2: Query<(Entity, &mut Transform, &GlobalTransform, &CardBeingDragged), (Without<Battlefield>, With<CardPlaced>)>,
) {
    for (card_entity, mut card_transform, card_global_transform, card_being_dragged) in card_query.iter_mut() {
        for event in mouse_button_input_events.iter() {
            if let Ok((play_area_entity, sprite, transform, global_transform, card_area, mut cards_order)) = area_query.get_single_mut() {
                let area_size = sprite.custom_size.unwrap() * transform.scale.truncate();

                if event.button == MouseButton::Left && event.state == ButtonState::Released {
                    commands.entity(card_entity).remove::<CardBeingDragged>();

                    let collision = collide(
                        global_transform.translation(),
                        area_size,
                        card_global_transform.translation(),
                        CARD_SIZE,
                    );

                    if let Some(collision) = collision {
                        info!("area collision {:?}", collision);
                        commands.entity(play_area_entity).add_child(card_entity);
                        cards_order.0.push(card_entity);
                        let position = area_children_query.get_single()
                            .map(|children| children.len())
                            .unwrap_or_else(|_| 0);
                        commands.entity(card_entity).insert(CardPlaced {});

                        card_transform.translation = card_translation(card_area.number_of_cards, position);
                    } else {
                        card_transform.translation = card_being_dragged.original_translation;
                    }
                }
            }
        }
    }

    for (card_entity, mut card_transform, card_global_transform, card_being_dragged) in card_query_2.iter_mut() {
        for event in mouse_button_input_events.iter() {
            if let Ok((_, sprite, transform, global_transform, card_area, mut cards_order)) = area_query.get_single_mut() {
                let area_size = sprite.custom_size.unwrap() * transform.scale.truncate();

                if event.button == MouseButton::Left && event.state == ButtonState::Released {
                    commands.entity(card_entity).remove::<CardBeingDragged>();

                    let collision = collide(
                        global_transform.translation(),
                        area_size,
                        card_global_transform.translation(),
                        CARD_SIZE,
                    );

                    if let Some(collision) = collision {
                        info!("area collision {:?}", collision);
                        info!("card_global_transform.translation().x {:?}", card_global_transform.translation().x);
                        info!("global_transform.translation().x {:?}", global_transform.translation().x);
                        let card_translation_x = card_global_transform.translation().x;
                        let multiplier = if card_translation_x > card_being_dragged.original_translation.x { -1.0 } else if card_translation_x < card_being_dragged.original_translation.x { 1.0 } else { 0.0 };
                        let negative_with = area_with(card_area.number_of_cards) / 2.0;
                        let relative_hover_x = card_translation_x + negative_with + multiplier * CARD_SIZE.x / 2.0;
                        info!("relative_hover_x {:?}", relative_hover_x);

                        let len = area_children_query.get_single()
                            .map(|children| children.len())
                            .unwrap_or_else(|_| 0);

                        let to_position = ((relative_hover_x / CARD_SIZE.x).floor() as usize).min(len - 1);
                        info!("position {:?}", to_position);

                        if let Some(from_position) = cards_order.0.iter().position(|entity| *entity == card_entity) {
                            cards_order.0.remove(from_position);
                        }

                        cards_order.0.insert(to_position, card_entity);

                        card_transform.translation = card_translation(card_area.number_of_cards, to_position);
                    } else {
                        card_transform.translation = card_being_dragged.original_translation;
                    }
                }
            }
        }
    }
}