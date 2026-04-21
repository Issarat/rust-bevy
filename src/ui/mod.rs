use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui).add_systems(Update, (
            update_health_bar,
            handle_temp_box,
        ));
    }
}

// ── Components ───────────────────────────────────────────────────

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthBarFill;

#[derive(Component)]
pub struct MoneyText;

#[derive(Component)]
pub struct TempBox {
    pub timer: Timer,
    pub blink_timer: Timer,
    pub visible: bool,
}

#[derive(Component)]
pub struct PlayerUI {
    pub current_health: u8,
    pub max_health: u8,
    pub money: u32,
}

impl Default for PlayerUI {
    fn default() -> Self {
        Self {
            current_health: 100,
            max_health: 100,
            money: 0,
        }
    }
}

// ── Systems ──────────────────────────────────────────────────────

fn setup_ui(mut commands: Commands) {
    // UI Camera
    commands.spawn(Camera2d);

    // Root UI node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            PlayerUI::default(),
        ))
        .with_children(|parent| {
            // Top bar (health + money)
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                ))
                .with_children(|parent| {
                    // Health bar container
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(20.0),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                            BorderColor::all(Color::srgb(0.8, 0.8, 0.8)),
                        ))
                        .with_children(|parent| {
                            // Health bar fill
                            parent.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                                HealthBarFill,
                            ));
                        });

                    // Temp box (20x20, next to health bar)
                    parent.spawn((
                        Node {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            margin: UiRect::left(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                        TempBox {
                            timer: Timer::from_seconds(30.0, TimerMode::Once),
                            blink_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                            visible: true,
                        },
                    ));

                    // Money text
                    parent.spawn((
                        Text::new("$0"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.8, 0.0)),
                        MoneyText,
                    ));
                });

            // Bottom bar (empty for now - can add inventory, skills, etc.)
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
            ));
        });
}

fn update_health_bar(
    ui_query: Query<&PlayerUI, Changed<PlayerUI>>,
    mut fill_query: Query<&mut Node, (With<HealthBarFill>, Without<PlayerUI>)>,
    mut money_query: Query<&mut Text, (With<MoneyText>, Without<PlayerUI>)>
) {
    for ui in &ui_query {
        // Update health bar width
        if let Ok(mut fill) = fill_query.single_mut() {
            let health_percent = ((ui.current_health as f32) / (ui.max_health as f32)) * 100.0;
            fill.width = Val::Percent(health_percent);
        }

        // Update money text
        if let Ok(mut text) = money_query.single_mut() {
            *text = Text::new(format!("${}", ui.money));
        }
    }
}

fn handle_temp_box(
    time: Res<Time>,
    mut temp_boxes: Query<(Entity, &mut TempBox, &mut BackgroundColor)>,
    mut commands: Commands
) {
    for (entity, mut temp_box, mut bg_color) in &mut temp_boxes {
        temp_box.timer.tick(time.delta());
        temp_box.blink_timer.tick(time.delta());

        // Only blink after 10 seconds of lifetime
        let should_blink = temp_box.timer.elapsed_secs() >= 5.0;

        if should_blink && temp_box.blink_timer.just_finished() {
            temp_box.visible = !temp_box.visible;
            bg_color.0 = if temp_box.visible {
                Color::srgb(1.0, 0.0, 0.0)
            } else {
                Color::srgb(0.3, 0.0, 0.0)
            };
        }

        // Remove after 30 seconds
        if temp_box.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

// ── Helper Functions ─────────────────────────────────────────────

pub fn damage_player(ui_query: &mut Query<&mut PlayerUI>, amount: u8) {
    for mut ui in ui_query {
        ui.current_health = ui.current_health.saturating_sub(amount);
    }
}

pub fn heal_player(ui_query: &mut Query<&mut PlayerUI>, amount: u8) {
    for mut ui in ui_query {
        ui.current_health = (ui.current_health + amount).min(ui.max_health);
    }
}

pub fn add_money(ui_query: &mut Query<&mut PlayerUI>, amount: u32) {
    for mut ui in ui_query {
        ui.money += amount;
    }
}
