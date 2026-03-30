use bevy::prelude::*;
mod main_menu;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Game,
}

#[derive(Component, Clone, Copy)]
enum MenuButtonAction {
    Play,
    Quit,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    #[default]
    Disabled,
}

pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

// --- START BUTTON (Blue) ---
const START_NORMAL: Color = Color::srgb(0.1, 0.45, 0.9);
const START_HOVER: Color = Color::srgb(0.2, 0.6, 1.0);
const START_PRESSED: Color = Color::srgb(0.5, 0.8, 1.0);

// ---NORMAL BUTTON (Grey) ---
const NORMAL_NORMAL: Color = Color::srgb(0.2, 0.2, 0.2);
const NORMAL_HOVER: Color = Color::srgb(0.3, 0.3, 0.3);
const NORMAL_PRESSED: Color = Color::srgb(0.4, 0.4, 0.4);

pub fn menu_plugin(app: &mut App) {
    app.init_state::<AppState>()
        .init_state::<MenuState>()
        .add_systems(OnEnter(AppState::Menu), menu_setup)
        .add_systems(OnEnter(MenuState::Main), main_menu::main_menu_setup)
        .add_systems(Update, (button_system, menu_action).run_if(in_state(AppState::Menu)));
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonAction),
        Changed<Interaction>
    >
) {
    for (interaction, mut bg_color, button_action) in &mut interaction_query {
        match (button_action, *interaction) {
            // --- ปุ่ม Start (สีฟ้า) ---
            (MenuButtonAction::Play, Interaction::Pressed) => {
                *bg_color = START_PRESSED.into();
            }
            (MenuButtonAction::Play, Interaction::Hovered) => {
                *bg_color = START_HOVER.into();
            }
            (MenuButtonAction::Play, Interaction::None) => {
                *bg_color = START_NORMAL.into();
            }

            // --- ปุ่มอื่นๆ (สีเทา) ---
            (_, Interaction::Pressed) => {
                *bg_color = NORMAL_PRESSED.into();
            }
            (_, Interaction::Hovered) => {
                *bg_color = NORMAL_HOVER.into();
            }
            (_, Interaction::None) => {
                *bg_color = NORMAL_NORMAL.into();
            }
        }
    }
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>)
    >,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    menu_state.set(MenuState::Disabled);
                }
            }
        }
    }
}
