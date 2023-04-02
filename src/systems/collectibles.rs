use crate::prelude::*;

pub struct CollectiblesPlugin;

#[derive(Eq, PartialEq)]
enum Potions {
    Red,
    Green,
    Blue,
}

impl Plugin for CollectiblesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetTracker::<Potions, Image>::default())
            .add_startup_system(load_collectibles)
            .add_system(add_collectibles.in_schedule(OnEnter(AppState::InGame)));
    }
}

fn load_collectibles(
    assetserver: Res<AssetServer>,
    mut tracker: ResMut<AssetTracker<Potions, Image>>,
) {
    let blue_handle: Handle<Image> = assetserver.load("potion_blue.png");
    let red_handle: Handle<Image> = assetserver.load("potion_red.png");
    let green_handle: Handle<Image> = assetserver.load("potion_green.png");

    tracker.assets.push(AssetReference {
        asset: Potions::Red,
        handle: red_handle,
    });
    tracker.assets.push(AssetReference {
        asset: Potions::Green,
        handle: green_handle,
    });
    tracker.assets.push(AssetReference {
        asset: Potions::Blue,
        handle: blue_handle,
    });
}

fn add_collectibles(mut cmd: Commands, tracker: Res<AssetTracker<Potions, Image>>) {
    if let Some(blue_handle) = tracker.get_handle(Potions::Blue) {
        cmd.spawn(SpriteBundle {
            texture: blue_handle,
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 0.0)),
            ..Default::default()
        });
    }
}
