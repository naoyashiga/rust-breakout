use crate::{PositionVec2, VelocityVec2, SizeVec2};
use lazy_static::lazy_static;

pub struct Constants {
    // Window dimensions
    pub window_width: f32,
    pub window_height: f32,

    // Ball constants
    pub ball_start_pos: PositionVec2,
    pub ball_start_vel: VelocityVec2,
    pub ball_size: SizeVec2,

    // Paddle constants
    pub paddle_start_pos: PositionVec2,
    pub paddle_size: SizeVec2,
    pub paddle_speed: f32,

    // Brick constants
    pub brick_rows: usize,
    pub brick_columns: usize,
    pub brick_size: SizeVec2,
    pub brick_padding: f32,
    pub brick_offset_top: f32,
}

impl Constants {
    pub fn new() -> Self {
        Constants {
            window_width: 800.0,
            window_height: 600.0,

            ball_start_pos: PositionVec2::new(400.0, 300.0),
            ball_start_vel: VelocityVec2::new(5.0, -5.0),
            ball_size: SizeVec2::new(10.0, 10.0),

            paddle_start_pos: PositionVec2::new(350.0, 550.0),
            paddle_size: SizeVec2::new(100.0, 20.0),
            paddle_speed: 10.0,

            brick_rows: 5,
            brick_columns: 10,
            brick_size: SizeVec2::new(70.0, 20.0),
            brick_padding: 10.0,
            brick_offset_top: 50.0,
        }
    }
}

lazy_static! {
    pub static ref CONSTANTS: Constants = Constants::new();
}