use super::animate::*;
use crate::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
            .add_system(move_player.in_set(OnUpdate(AppState::InGame)))
            .add_system(animate_player.in_set(OnUpdate(AppState::InGame)));
    }
}

fn spawn_player(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("gary_walking_spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(27.0, 45.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    cmd.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(54.0, 90.0)),
                ..Default::default()
            },
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        },
        Collider::cuboid(15.0, 45.0),
        RigidBody::Dynamic,
        KinematicCharacterController {
            up: Vec2::Y,
            offset: CharacterLength::Absolute(0.01),
            snap_to_ground: Some(CharacterLength::Relative(0.5)),
            ..default()
        },
        Velocity::zero(),
        GravityScale(8.0),
        Player,
        LockedAxes::ROTATION_LOCKED,
        PlayerAnimation {
            length: 4,
            timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
        },
    ));
}

fn move_player(
    mut controllers: Query<(&mut Velocity, &mut KinematicCharacterController), With<Player>>,
    outputs: Query<&KinematicCharacterControllerOutput>,
    mut player_query: Query<&mut TextureAtlasSprite, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (mut velocity, mut player_controller) = controllers.single_mut();
    let mut movement = Vec2::new(0.0, 0.0);
    let mut player = player_query.single_mut();

    let ground_touched = outputs.iter().map(|p| p.grounded).any(|t| t);
    if keyboard_input.pressed(KeyCode::A) {
        movement += Vec2::new(-1.0, 0.0);
        player.flip_x = true;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement += Vec2::new(1.0, 0.0);
        player.flip_x = false;
    }
    if keyboard_input.just_pressed(KeyCode::W) && ground_touched {
        velocity.linvel = Vec2::new(0.0, 400.0);
    }
    player_controller.translation = Some(movement);
}
