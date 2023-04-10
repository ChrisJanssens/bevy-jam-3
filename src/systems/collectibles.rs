use bevy_rapier2d::rapier::prelude::CollisionEventFlags;

use crate::prelude::*;

pub struct CollectiblesPlugin;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Potions {
    Red,
    Green,
    Blue,
}

#[derive(Component)]
struct Collectible {
    collectible_type: Potions,
}

impl Plugin for CollectiblesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetTracker::<Potions, Image>::default())
            .add_startup_system(load_collectibles)
            .add_system(add_collectibles.in_schedule(OnEnter(AppState::InGame)))
            .add_system(pickup_collectibles.in_base_set(CoreSet::PostUpdate));
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
        cmd.spawn((
            SpriteBundle {
                texture: blue_handle,
                transform: Transform::from_translation(Vec3::new(-100.0, -250.0, 0.0)),
                ..Default::default()
            },
            Collider::capsule_y(10.0, 8.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Collectible {
                collectible_type: Potions::Blue,
            },
        ));
    }
    if let Some(green_handle) = tracker.get_handle(Potions::Green) {
        cmd.spawn((
            SpriteBundle {
                texture: green_handle,
                transform: Transform::from_translation(Vec3::new(100.0, -250.0, 0.0)),
                ..Default::default()
            },
            Collider::capsule_y(10.0, 8.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Collectible {
                collectible_type: Potions::Green,
            },
        ));
    }
    if let Some(red_handle) = tracker.get_handle(Potions::Red) {
        cmd.spawn((
            SpriteBundle {
                texture: red_handle,
                transform: Transform::from_translation(Vec3::new(320.0, -250.0, 0.0)),
                ..Default::default()
            },
            Collider::capsule_y(10.0, 8.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Collectible {
                collectible_type: Potions::Red,
            },
        ));
    }
}

fn pickup_collectibles(
    mut cmd: Commands,
    collectibles: Query<(Entity, &Collectible)>,
    mut transform_events: EventWriter<TransformPlayer>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        for (collect_ent, collectible) in collectibles.iter() {
            match collision_event {
                CollisionEvent::Started(en1, en2, t) => {
                    if (en1 == &collect_ent || en2 == &collect_ent)
                        && t == &CollisionEventFlags::SENSOR
                    {
                        transform_events.send(TransformPlayer {
                            catalyst: collectible.collectible_type,
                        });
                        cmd.entity(collect_ent).despawn();
                    }
                }
                CollisionEvent::Stopped(_, _, _) => (),
            }
        }
    }
}
