use crate::prelude::*;
use bevy::asset::Asset;

#[derive(Resource)]
pub struct AssetTracker<T, H: Asset> {
    pub assets: Vec<AssetReference<T, H>>,
}

impl<T, H: Asset> Default for AssetTracker<T, H> {
    fn default() -> Self {
        AssetTracker {
            assets: Vec::<AssetReference<T, H>>::new(),
        }
    }
}

impl<T: Eq, H: Asset> AssetTracker<T, H> {
    pub fn get_handle(&self, target: T) -> Option<Handle<H>> {
        for r in self.assets.iter() {
            if r.asset == target {
                return Some(r.handle.clone_weak());
            }
        }
        None
    }
}

pub struct AssetReference<T, H: Asset> {
    pub asset: T,
    pub handle: Handle<H>,
}
