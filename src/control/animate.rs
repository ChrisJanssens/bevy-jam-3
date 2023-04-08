use crate::prelude::*;

#[derive(Component)]
pub struct PlayerAnimation {
    pub length: usize,
    pub timer: Timer,
}

pub fn animate_player_idle() {}

pub fn animate_player_walking(
    mut player_anim: Query<(&mut TextureAtlasSprite, &mut PlayerAnimation), With<Player>>,
    time: Res<Time>,
) {
    for (mut atlas, mut anim) in player_anim.iter_mut() {
        if anim.timer.tick(time.delta()).finished() {
            let mut i = atlas.index + 1;
            if i == anim.length {
                i = 0;
            }
            atlas.index = i;
        }
    }
}
