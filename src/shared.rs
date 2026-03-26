use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub fn get_bounds(window: &Window, sprite_size: f32) -> (f32, f32, f32, f32) {
    let half = sprite_size / 2.0;
    (
        -window.width() / 2.0 + half,
        window.width() / 2.0 - half,
        -window.height() / 2.0 + half,
        window.height() / 2.0 - half,
    )
}

pub fn random_position(window: &Window) -> (f32, f32) {
    ((random::<f32>() - 0.5) * window.width(), (random::<f32>() - 0.5) * window.height())
}

pub fn spawn_sprites<F, B>(
    commands: &mut Commands,
    window: &Window,
    count: usize,
    position: Option<(f32, f32)>,
    mut bundle_fn: F
)
    where B: Bundle, F: FnMut(f32, f32) -> B
{
    for _ in 0..count {
        let (x, y) = position.unwrap_or_else(|| random_position(window));
        commands.spawn(bundle_fn(x, y));
    }
}

pub fn get_window<'a>(window_query: &'a Query<&Window, With<PrimaryWindow>>) -> Option<&'a Window> {
    window_query.single().ok()
}
