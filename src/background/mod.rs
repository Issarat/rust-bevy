use rand::Rng;
use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum BackgroundSet {
    #[default]
    Mountain,
    City,
}

//=============================Parallax Mode=====================================
// #[derive(Clone, Reflect, Debug)]
// pub struct MyScrollerItem {
//     pub size: Vec2,
// }

// impl GeneratedItem for MyScrollerItem {
//     fn size(&self) -> Vec2 {
//         self.size
//     }
// }

// #[derive(Component, Clone, Default)]
// pub struct MyGenerator {}

// impl ScrollerGenerator for MySpriteGenerator {
//     type I = MyScrollerItem;

//     fn gen_item(&mut self) -> Self::I {
//         Self::I {}
//     }
// }

pub fn get_bg_setup(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    //let mut rng = rand::thread_rng();
    //let bg_rand = (rng.gen_item < BackgroundSet) > ();
    let idle_bg = asset_server.load("backgrounds/Mountain/sky.png");
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                position_type: PositionType::Absolute,
                ..default()
            },
            ImageNode::new(idle_bg),
        ))
        .id()
}
