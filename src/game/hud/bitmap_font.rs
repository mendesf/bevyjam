use bevy::prelude::*;

pub trait BitmapNumberValue: Component {
    fn get_value(&self) -> u8;
}

#[derive(Component)]
pub struct BitmapNumberConfig {
    pub tile_size: Vec2,
    pub columns: usize,
    pub row: usize,
    pub font_spacing: f32,
}

impl BitmapNumberConfig {
    fn width(&self, count: usize) -> f32 {
        let count = count as f32;
        count * self.tile_size.x + (count - 1.0) * self.font_spacing
    }

    fn sprite_index(&self, digit: char) -> usize {
        let column = digit.to_string().parse::<usize>().unwrap();
        self.row * self.columns + column
    }

    fn translation_x(&self, count: usize, index: usize) -> f32 {
        -self.width(count) / 2.0 + index as f32 * (self.tile_size.x + self.font_spacing)
    }
}

pub(super) fn update_bitmap_number_digits<Value: BitmapNumberValue, Digit: Component>(
    value_query: Query<(&Value, &BitmapNumberConfig), With<Value>>,
    mut digit_query: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Visibility), With<Digit>>,
) {
    let (value, config) = value_query.get_single().unwrap();
    let binding = value.get_value().to_string();
    let mut digits = binding.chars();
    let count = digits.clone().count();

    digit_query.iter_mut().enumerate().for_each(|(i, item)| {
        let (mut transform, mut sprite, mut visibility) = item;

        if let Some(digit) = digits.next() {
            transform.translation.x = config.translation_x(count, i);
            sprite.index = config.sprite_index(digit);
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    });
}