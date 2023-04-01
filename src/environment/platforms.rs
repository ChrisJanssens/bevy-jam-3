use crate::prelude::*;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_platforms.in_schedule(OnEnter(AppState::InGame)));
    }
}

fn spawn_platforms(mut cmd: Commands, win: Query<&Window>) {
    let window = win.single();
    //bottom
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(window.width(), 50.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -(window.height() / 2.0) + 25.0,
                0.0,
            )),
            ..Default::default()
        },
        Collider::cuboid(window.width() / 2.0, 25.0),
    ));
    //left
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(50.0, window.height())),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                -(window.width() / 2.0) + 25.0,
                0.0,
                0.0,
            )),
            ..Default::default()
        },
        Collider::cuboid(25.0, window.height() / 2.0),
    ));
    //right
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(50.0, window.height())),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0 - 25.0,
                0.0,
                0.0,
            )),
            ..Default::default()
        },
        Collider::cuboid(25.0, window.height() / 2.0),
    ));
}
