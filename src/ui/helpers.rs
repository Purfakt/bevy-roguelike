use bevy::prelude::*;

use super::UiAssets;

const FONT_SIZE: f32 = 18.;

#[derive(Component)]
pub struct ClickableButton;

pub fn get_button(commands: &mut Commands, width: Val, height: Val, margin: UiRect, image: &Handle<Image>) -> Entity {
    commands
        .spawn((
            ClickableButton,
            ButtonBundle {
                style: Style {
                    width,
                    height,
                    margin,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                image: UiImage::new(image.clone()),
                ..Default::default()
            },
        ))
        .id()
}

pub fn get_text_bundle(text: &str, assets: &UiAssets) -> impl Bundle {
    TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                color: Color::WHITE,
                font: assets.font.clone(),
                font_size: FONT_SIZE,
            },
        ),
        ..Default::default()
    }
}
