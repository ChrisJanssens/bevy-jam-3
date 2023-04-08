use std::collections::VecDeque;

use crate::prelude::*;

#[derive(Component)]
pub struct PlayerAnimation {
    pub idle_range: (usize, usize),
    pub walk_range: (usize, usize),
    pub jump_range: (usize, usize),
    pub timer: Timer,
    pub blink_timer: Option<Timer>,
    pub blink_sequence: VecDeque<u64>,
}

pub fn animate_player_idle(
    mut player_anim: Query<(&mut TextureAtlasSprite, &mut PlayerAnimation), With<Player>>,
    time: Res<Time>,
) {
    let (mut atlas, mut anim) = player_anim.single_mut();
    match &mut anim.blink_timer {
        Some(timer) => {
            // eyes closed
            if timer.tick(time.delta()).finished() {
                atlas.index = anim.idle_range.1;
                anim.blink_timer = None;
            }
        }
        None => {
            // eyes open
            if anim.timer.tick(time.delta()).finished() {
                if let Some(cur_time) = anim.blink_sequence.pop_front() {
                    atlas.index = anim.idle_range.0;
                    anim.blink_timer =
                        Some(Timer::new(Duration::from_millis(cur_time), TimerMode::Once));
                    anim.blink_sequence.push_back(cur_time);
                }
            }
        }
    }
}

pub fn animate_player_walking(
    mut player_anim: Query<(&mut TextureAtlasSprite, &mut PlayerAnimation), With<Player>>,
    time: Res<Time>,
) {
    for (mut atlas, mut anim) in player_anim.iter_mut() {
        if anim.timer.tick(time.delta()).finished() {
            let mut i = atlas.index + 1;
            if i > anim.walk_range.1 {
                i = anim.walk_range.0;
            }
            atlas.index = i;
        }
    }
}

pub fn animate_player_jumping(
    mut player_anim: Query<
        (&mut TextureAtlasSprite, &mut PlayerAnimation, &mut Velocity),
        With<Player>,
    >,
    time: Res<Time>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    for (mut atlas, mut anim, mut vel) in player_anim.iter_mut() {
        if anim.timer.tick(time.delta()).finished() {
            let i = atlas.index + 1;
            if i == anim.jump_range.1 {
                next_player_state.set(PlayerState::JumpingInAir);
            } else if i == anim.jump_range.1 - 1 {
                vel.linvel = Vec2::new(0.0, 400.0);
            }
            atlas.index = i;
        }
    }
}
