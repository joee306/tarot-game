use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Clone, States, Default, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    Loading,
    Main,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerSpites {
    #[asset(texture_atlas(tile_size_x = 24., tile_size_y = 24., columns = 7, rows = 1,))]
    #[asset(path = "sprites/gabe-idle-run.png")]
    pub gabe_walking: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 10, rows = 1,))]
    #[asset(path = "sprites/valla_walking.png")]
    pub valla_walking: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 128., tile_size_y = 128., columns = 12, rows = 1,))]
    #[asset(path = "sprites/tom_walking.png")]
    pub player_walking_side: Handle<TextureAtlas>,

    #[asset(path = "sprites/tom_walking.png")]
    pub player_walking_side_img: Handle<Image>,

    //#[asset(texture_atlas(tile_size_x = 128., tile_size_y = 128., columns = 12, rows = 1,))]
    #[asset(path = "sprites/tom_walking_up.png")]
    pub player_walking_up: Handle<Image>,

    //#[asset(texture_atlas(tile_size_x = 128., tile_size_y = 128., columns = 12, rows = 1,))]
    #[asset(path = "sprites/tom_walking_down.png")]
    pub player_walking_down: Handle<Image>,
}

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub frame_count: usize,
}
