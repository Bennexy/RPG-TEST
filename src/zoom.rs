use crate::{game_plugins::player::Player, consts::MAX_ZOOM};
use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct ScaleableWorldViewPlugin;

impl Plugin for ScaleableWorldViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
        .add_systems(Update, change_world_scale);
    }
}

#[derive(Component)]
pub struct WorldView {
    zoom_factor: f32,
}

impl Default for WorldView {
    fn default() -> Self {
        Self { zoom_factor: 64.0 }
    }
}

impl WorldView {
    pub fn get_zoom_state(&self) -> bevy::render::camera::ScalingMode {
        bevy::render::camera::ScalingMode::AutoMin {
            min_width: 256.0 * self.zoom_factor,
            min_height: 144.0 * self.zoom_factor,
        }
    }

    pub fn zoom_in(&mut self) {
        self.zoom_factor *= 0.5;
        if self.zoom_factor <= 1. {
            self.zoom_factor = 1.;
            return;
        }
    }

    pub fn zoom_out(&mut self) {
        self.zoom_factor *= 2.0;
        if self.zoom_factor >= MAX_ZOOM {
            self.zoom_factor = MAX_ZOOM;
            return;
        }
    }
}

fn setup(mut commands: Commands) {
    let mut camera: Camera2dBundle = Camera2dBundle::default();
    let world_view = WorldView::default();

    camera.projection.scaling_mode = world_view.get_zoom_state();

    commands.spawn((camera, world_view));
}

pub fn change_world_scale(
    mut world_view: Query<
        (&mut OrthographicProjection, &mut WorldView),
        (Without<Player>, With<WorldView>),
    >,
    input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {

    if input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlLeft) {
        // info!("in control loop");
        let (mut projection, mut world_view) = world_view.single_mut();

        if input.just_pressed(KeyCode::Minus) {
            world_view.zoom_out();
            projection.scaling_mode = world_view.get_zoom_state();
        } else if input.just_pressed(KeyCode::Plus) {
            world_view.zoom_in();
            projection.scaling_mode = world_view.get_zoom_state();
        } else {
            for ev in scroll_evr.read() {
                debug!("Scroll: vertical: {}, horizontal: {}", ev.y, ev.x);

                if ev.y > 0.0 {
                    world_view.zoom_in();
                    projection.scaling_mode = world_view.get_zoom_state();
                } else if ev.y < 0.0 {
                    world_view.zoom_out();
                    projection.scaling_mode = world_view.get_zoom_state();
                } else {
                    error!("unkown zoom direction");
                }

                debug!("{}", world_view.zoom_factor)
            }
        }
    }
}
