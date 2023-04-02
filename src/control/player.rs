use crate::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
            .add_system(move_player.in_set(OnUpdate(AppState::InGame)));
    }
}

fn spawn_player(mut cmd: Commands) {
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        },
        Collider::cuboid(20.0, 20.0),
        RigidBody::Dynamic,
        KinematicCharacterController {
            up: Vec2::Y,
            offset: CharacterLength::Absolute(0.01),
            snap_to_ground: Some(CharacterLength::Relative(0.5)),
            ..default()
        },
        Velocity::zero(),
        GravityScale::default(),
        Player,
        LockedAxes::ROTATION_LOCKED,
    ));
}

fn move_player(
    mut controllers: Query<(&mut Velocity, &mut KinematicCharacterController), With<Player>>,
    outputs: Query<&KinematicCharacterControllerOutput>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (mut velocity, mut player) = controllers.single_mut();
    let mut movement = Vec2::new(0.0, 0.0);

    let ground_touched = outputs.iter().map(|p| p.grounded).any(|t| t);
    movement += if keyboard_input.pressed(KeyCode::A) {
        Vec2::new(-1.0, 0.0)
    } else {
        Vec2::new(0.0, 0.0)
    };
    movement += if keyboard_input.pressed(KeyCode::D) {
        Vec2::new(1.0, 0.0)
    } else {
        Vec2::new(0.0, 0.0)
    };
    if keyboard_input.just_pressed(KeyCode::W) && ground_touched {
        velocity.linvel = Vec2::new(0.0, 100.0);
    }
    player.translation = Some(movement);
}
