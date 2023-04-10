use std::collections::VecDeque;

use super::animate::*;
use crate::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    movement_scalar: f32,
    pub jump_strength: f32,
}

pub struct TransformPlayer {
    pub catalyst: Potions,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    Jumping,
    JumpingInAir,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum PlayerSprite {
    LittleGary,
    NormalGary,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetTracker::<PlayerSprite, TextureAtlas>::default())
            .add_state::<PlayerState>()
            .add_event::<TransformPlayer>()
            .add_startup_system(load_sheets)
            .add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
            .add_system(move_player.in_set(OnUpdate(AppState::InGame)))
            .add_system(handle_transform)
            .add_system(animate_player_idle.in_set(OnUpdate(PlayerState::Idle)))
            .add_system(animate_player_walking.in_set(OnUpdate(PlayerState::Walking)))
            .add_system(animate_player_jumping.in_set(OnUpdate(PlayerState::Jumping)));
    }
}

fn load_sheets(
    assetserver: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut tracker: ResMut<AssetTracker<PlayerSprite, TextureAtlas>>,
) {
    let texture_handle = assetserver.load("gary_normal_spritesheet.png");
    let texture_handle_small = assetserver.load("gary_little_spritesheet.png");
    let normal_gary =
        TextureAtlas::from_grid(texture_handle, Vec2::new(27.0, 45.0), 4, 3, None, None);
    let little_gary =
        TextureAtlas::from_grid(texture_handle_small, Vec2::new(9.0, 15.0), 4, 3, None, None);

    tracker.assets.push(AssetReference {
        asset: PlayerSprite::NormalGary,
        handle: texture_atlases.add(normal_gary),
    });
    tracker.assets.push(AssetReference {
        asset: PlayerSprite::LittleGary,
        handle: texture_atlases.add(little_gary),
    });
}

fn spawn_player(mut cmd: Commands, tracker: Res<AssetTracker<PlayerSprite, TextureAtlas>>) {
    if let Some(player_sprite) = tracker.get_handle(PlayerSprite::NormalGary) {
        cmd.spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 7,
                    custom_size: Some(Vec2::new(54.0, 90.0)),
                    ..Default::default()
                },
                texture_atlas: player_sprite,
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
            Player {
                movement_scalar: 1.0,
                jump_strength: 400.0,
            },
            LockedAxes::ROTATION_LOCKED,
            PlayerAnimation {
                idle_range: (8, 9),
                walk_range: (0, 3),
                jump_range: (4, 7),
                timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
                blink_timer: None,
                blink_sequence: VecDeque::from([4000, 800]),
            },
        ));
    }
}

fn move_player(
    mut controllers: Query<(
        &mut KinematicCharacterController,
        &mut TextureAtlasSprite,
        &PlayerAnimation,
        &Player,
    )>,
    outputs: Query<&KinematicCharacterControllerOutput>,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    cur_state: Res<State<PlayerState>>,
) {
    let (mut player_controller, mut player_sprite, play_anim, player) = controllers.single_mut();
    let mut movement = Vec2::new(0.0, 0.0);

    let ground_touched = outputs.iter().map(|p| p.grounded).any(|t| t);

    if keyboard_input.pressed(KeyCode::A) {
        movement += Vec2::new(-1.0 * player.movement_scalar, 0.0);
        player_sprite.flip_x = true;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement += Vec2::new(1.0 * player.movement_scalar, 0.0);
        player_sprite.flip_x = false;
    }

    if ground_touched {
        if keyboard_input.just_pressed(KeyCode::A) || keyboard_input.just_pressed(KeyCode::D) {
            player_sprite.index = play_anim.walk_range.0;
            next_player_state.set(PlayerState::Walking);
        } else if (keyboard_input.just_released(KeyCode::A) && !keyboard_input.pressed(KeyCode::D))
            || (!keyboard_input.pressed(KeyCode::A) && keyboard_input.just_released(KeyCode::D))
        {
            player_sprite.index = play_anim.idle_range.0;
            next_player_state.set(PlayerState::Idle);
        } else if cur_state.0 == PlayerState::JumpingInAir {
            if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::D) {
                player_sprite.index = play_anim.walk_range.0;
                next_player_state.set(PlayerState::Walking); // hit the ground walking
            } else {
                player_sprite.index = play_anim.idle_range.0;
                next_player_state.set(PlayerState::Idle);
            }
        }
        if keyboard_input.just_pressed(KeyCode::W) {
            player_sprite.index = play_anim.jump_range.0;
            next_player_state.set(PlayerState::Jumping);
        }
    }
    player_controller.translation = Some(movement);
}

fn handle_transform(
    mut player: Query<(
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &mut Collider,
        &mut GravityScale,
        &mut Player,
        &mut PlayerAnimation,
    )>,
    mut transform_events: EventReader<TransformPlayer>,
    tracker: Res<AssetTracker<PlayerSprite, TextureAtlas>>,
) {
    if let Ok((mut sprite, mut atlas, mut collider, mut gravity, mut player, mut playeranim)) =
        player.get_single_mut()
    {
        for event in transform_events.iter() {
            match event.catalyst {
                Potions::Blue => {
                    if let Some(player_sprite) = tracker.get_handle(PlayerSprite::LittleGary) {
                        sprite.custom_size = Some(Vec2::new(18.0, 30.0));
                        sprite.color = Color::default();
                        *atlas = player_sprite;
                        *collider = Collider::cuboid(8.0, 13.0);
                        *gravity = GravityScale(9.0);
                        playeranim.timer =
                            Timer::new(Duration::from_millis(80), TimerMode::Repeating);
                        player.movement_scalar = 4.0;
                        player.jump_strength = 200.0;
                    }
                }
                Potions::Green => {
                    if let Some(player_sprite) = tracker.get_handle(PlayerSprite::NormalGary) {
                        sprite.custom_size = Some(Vec2::new(54.0, 90.0));
                        sprite.color = Color::GREEN;
                        *atlas = player_sprite;
                        *collider = Collider::cuboid(15.0, 45.0);
                        *gravity = GravityScale(8.0);
                        playeranim.timer =
                            Timer::new(Duration::from_millis(200), TimerMode::Repeating);
                        player.movement_scalar = 2.0;
                        player.jump_strength = 1000.0;
                    }
                }
                Potions::Red => {
                    if let Some(player_sprite) = tracker.get_handle(PlayerSprite::NormalGary) {
                        sprite.custom_size = Some(Vec2::new(54.0, 90.0));
                        sprite.color = Color::RED;
                        *atlas = player_sprite;
                        *collider = Collider::cuboid(15.0, 45.0);
                        *gravity = GravityScale(8.0);
                        playeranim.timer =
                            Timer::new(Duration::from_millis(200), TimerMode::Repeating);
                        player.movement_scalar = 1.0;
                        player.jump_strength = 0.0;
                    }
                }
            }
        }
    }
}
