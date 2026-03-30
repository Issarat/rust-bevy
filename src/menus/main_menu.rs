use bevy::{ color::palettes::css::CRIMSON, prelude::* };

use super::{ TEXT_COLOR, MenuState, MenuButtonAction };

#[derive(Component)]
struct OnMainMenuScreen;

pub fn main_menu_setup(mut commands: Commands) {
    let button_node = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    commands.spawn((
        DespawnOnExit(MenuState::Main),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(CRIMSON.into()),
        OnMainMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    Text::new("Bevy Game Menu UI"),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(px(50)),
                        ..default()
                    },
                ),
                (
                    Button,
                    button_node.clone(),
                    MenuButtonAction::Play,
                    children![(
                        Text::new("New Game"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    )],
                ),
                (
                    Button,
                    button_node,
                    MenuButtonAction::Quit,
                    children![(Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR))],
                )
            ],
        )],
    ));
}
