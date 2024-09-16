use ggez::audio::{SoundSource, Source};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{self, KeyCode};
use ggez::{Context, GameResult};

mod constants;
use constants::CONSTANTS;

#[derive(Debug, Clone, Copy)]
struct PositionVec2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
struct SizeVec2 {
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Copy)]
struct VelocityVec2 {
    x: f32,
    y: f32,
}

impl PositionVec2 {
    fn new(x: f32, y: f32) -> Self {
        PositionVec2 { x, y }
    }
}

impl SizeVec2 {
    fn new(width: f32, height: f32) -> Self {
        SizeVec2 { width, height }
    }
}

impl VelocityVec2 {
    fn new(x: f32, y: f32) -> Self {
        VelocityVec2 { x, y }
    }
}

struct Ball {
    pos: PositionVec2,
    vel: VelocityVec2,
    size: SizeVec2,
}

struct Paddle {
    pos: PositionVec2,
    size: SizeVec2,
    speed: f32,
}

struct Brick {
    pos: PositionVec2,
    size: SizeVec2,
    color: Color,
}

struct GameState {
    ball: Ball,
    paddle: Paddle,
    bricks: Vec<Brick>,
    game_over: bool,
    paddle_hit_sound: Source,
    brick_hit_sound: Source,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut state = GameState {
            ball: Ball {
                pos: CONSTANTS.ball_start_pos,
                vel: CONSTANTS.ball_start_vel,
                size: CONSTANTS.ball_size,
            },
            paddle: Paddle {
                pos: CONSTANTS.paddle_start_pos,
                size: CONSTANTS.paddle_size,
                speed: CONSTANTS.paddle_speed,
            },
            bricks: Vec::new(),
            game_over: false,
            paddle_hit_sound: Source::new(ctx, CONSTANTS.paddle_hit_sound_path)?,
            brick_hit_sound: Source::new(ctx, CONSTANTS.brick_hit_sound_path)?,
        };

        state.generate_bricks();

        Ok(state)
    }

    pub fn reset(&mut self) {
        self.ball.pos = CONSTANTS.ball_start_pos;
        self.ball.vel = CONSTANTS.ball_start_vel;
        self.paddle.pos = CONSTANTS.paddle_start_pos;
        self.bricks.clear();
        self.generate_bricks();
        self.game_over = false;
    }

    fn generate_bricks(&mut self) {
        for row in 0..CONSTANTS.brick_rows {
            for col in 0..CONSTANTS.brick_columns {
                let brick = Brick {
                    pos: PositionVec2::new(
                        col as f32 * (CONSTANTS.brick_size.width + CONSTANTS.brick_padding)
                            + CONSTANTS.brick_padding,
                        row as f32 * (CONSTANTS.brick_size.height + CONSTANTS.brick_padding)
                            + CONSTANTS.brick_offset_top,
                    ),
                    size: CONSTANTS.brick_size,
                    color: CONSTANTS.brick_color,
                };
                self.bricks.push(brick);
            }
        }
    }

    fn check_ball_wall_collision(&mut self) {
        if self.ball.pos.x - self.ball.size.width / 2.0 <= 0.0
            || self.ball.pos.x + self.ball.size.width / 2.0 >= CONSTANTS.window_width
        {
            self.ball.vel.x = -self.ball.vel.x;
        }
        if self.ball.pos.y - self.ball.size.height / 2.0 <= 0.0 {
            self.ball.vel.y = -self.ball.vel.y;
        }
    }

    fn check_ball_paddle_collision(&mut self, ctx: &mut Context) -> GameResult<()> {
        let ball_bottom = self.ball.pos.y + self.ball.size.height / 2.0;
        let paddle_top = self.paddle.pos.y;
        let ball_x = self.ball.pos.x;
        let paddle_left = self.paddle.pos.x;
        let paddle_right = self.paddle.pos.x + self.paddle.size.width;

        if ball_bottom >= paddle_top && ball_x >= paddle_left && ball_x <= paddle_right {
            self.ball.vel.y = -self.ball.vel.y;
            // Optional: Change ball angle based on where it hit the paddle
            let paddle_center = paddle_left + self.paddle.size.width / 2.0;
            let ball_offset = (ball_x - paddle_center) / (self.paddle.size.width / 2.0);
            self.ball.vel.x += ball_offset * 2.0; // Amplify the angle change

            // Play paddle hit sound
            self.paddle_hit_sound.play(ctx)?;
        }
        Ok(())
    }

    fn check_ball_brick_collision(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut collision_occurred = false;
        self.bricks.retain(|brick| {
            let collision = self.ball.pos.x + self.ball.size.width / 2.0 > brick.pos.x
                && self.ball.pos.x - self.ball.size.width / 2.0 < brick.pos.x + brick.size.width
                && self.ball.pos.y + self.ball.size.height / 2.0 > brick.pos.y
                && self.ball.pos.y - self.ball.size.height / 2.0 < brick.pos.y + brick.size.height;

            if collision {
                // Reverse ball direction
                if self.ball.pos.x < brick.pos.x || self.ball.pos.x > brick.pos.x + brick.size.width
                {
                    self.ball.vel.x = -self.ball.vel.x;
                } else {
                    self.ball.vel.y = -self.ball.vel.y;
                }
                collision_occurred = true;
            }

            !collision // Keep the brick if there's no collision
        });

        if collision_occurred {
            // Play brick hit sound
            self.brick_hit_sound.play(ctx)?;
        }
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.game_over {
            // Move the ball
            self.ball.pos.x += self.ball.vel.x;
            self.ball.pos.y += self.ball.vel.y;

            // Move the paddle
            if keyboard::is_key_pressed(ctx, KeyCode::Left) {
                self.paddle.pos.x -= self.paddle.speed;
            }
            if keyboard::is_key_pressed(ctx, KeyCode::Right) {
                self.paddle.pos.x += self.paddle.speed;
            }

            // Prevent the paddle from moving off-screen
            self.paddle.pos.x = self
                .paddle
                .pos
                .x
                .max(0.0)
                .min(CONSTANTS.window_width - self.paddle.size.width);

            // Check for collisions
            self.check_ball_wall_collision();
            self.check_ball_paddle_collision(ctx)?;
            self.check_ball_brick_collision(ctx)?;

            // Check for game over condition
            if self.ball.pos.y > CONSTANTS.window_height {
                self.game_over = true;
            }
        } else {
            // Handle retry when game is over
            if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                self.reset();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        if !self.game_over {
            // Draw the ball
            let ball_rect = graphics::Rect::new(
                self.ball.pos.x - self.ball.size.width / 2.0,
                self.ball.pos.y - self.ball.size.height / 2.0,
                self.ball.size.width,
                self.ball.size.height,
            );
            let ball = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                ball_rect,
                Color::WHITE,
            )?;
            graphics::draw(ctx, &ball, graphics::DrawParam::default())?;

            // Draw the paddle
            let paddle_rect = graphics::Rect::new(
                self.paddle.pos.x,
                self.paddle.pos.y,
                self.paddle.size.width,
                self.paddle.size.height,
            );
            let paddle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                paddle_rect,
                Color::WHITE,
            )?;
            graphics::draw(ctx, &paddle, graphics::DrawParam::default())?;

            // Draw the bricks
            for brick in &self.bricks {
                let brick_rect = graphics::Rect::new(
                    brick.pos.x,
                    brick.pos.y,
                    brick.size.width,
                    brick.size.height,
                );
                let brick_mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    brick_rect,
                    brick.color,
                )?;
                graphics::draw(ctx, &brick_mesh, graphics::DrawParam::default())?;
            }
        } else {
            // Draw game over screen
            let game_over_text = graphics::Text::new("Game Over! Press SPACE to retry");
            let screen_center = graphics::screen_coordinates(ctx);
            let text_dimensions = game_over_text.dimensions(ctx);
            let position = [
                screen_center.w / 2.0 - text_dimensions.w as f32 / 2.0,
                screen_center.h / 2.0 - text_dimensions.h as f32 / 2.0,
            ];
            graphics::draw(ctx, &game_over_text, (position, 0.0, Color::WHITE))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("breakout", "Your Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Breakout!"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(CONSTANTS.window_width, CONSTANTS.window_height),
        )
        .build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
