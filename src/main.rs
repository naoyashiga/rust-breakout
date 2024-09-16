use ggex::input::keyboard::{self, KeyCode};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::keyboard;
use ggez::{Context, GameResult};

struct Ball {
    pos: [f32; 2],
    vel: [f32; 2],
    size: f32,
}

struct Paddle {
    pos: [f32; 2],
    size: [f32; 2],
    speed: f32,
}

struct Brick {
    pos: [f32; 2],
    size: [f32; 2],
    color: Color,
}

struct GameState {
    ball: Ball,
    paddle: Paddle,
    bricks: Vec<Brick>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            ball: Ball {
                pos: [400.0, 300.0],
                vel: [5.0, -5.0],
                size: 10.0,
            },
            paddle: Paddle {
                pos: [350.0, 550.0],
                size: [100.0, 10.0],
                speed: 5.0,
            },
            bricks: Vec::new(),
        }
    }

    fn check_ball_wall_collision(&mut self) {
        if self.ball.pos[0] - self.ball.size / 2.0 <= 0.0
            || self.ball.pos[0] + self.ball.size / 2.0 >= 800.0
        {
            self.ball.vel[0] = -self.ball.vel[0];
        }

        if self.ball.pos[1] - self.ball.size / 2.0 <= 0.0 {
            self.ball.vel[1] = -self.ball.vel[1];
        }
    }

    fn check_ball_paddle_collision(&mut self) {
        let ball_bottom = self.ball.pos[1] + self.ball.size / 2.0;
        let paddle_top = self.paddle.pos[1];
        let ball_x = self.ball.pos[0];
        let paddle_left = self.paddle.pos[0];
        let paddle_right = self.paddle.pos[0] + self.paddle.size[0];

        if ball_bottom >= paddle_top && ball_x >= paddle_left && ball_x <= paddle_right {
            self.ball.vel[1] = -self.ball.vel[1];

            let paddle_center = self.paddle.pos[0] + self.paddle.size[0] / 2.0;
            let ball_offset = (ball_x - paddle_center) / (self.paddle.size[0] - 2.0);
            self.ball.vel[0] += ball_offset * 2.0;
        }
    }

    fn check_ball_brick_collision(&mut self) {
        self.bricks.retain(|brick| {
            let collision = self.ball.pos[0] + self.ball.size / 2.0 >= brick.pos[0]
                && self.ball.pos[0] - self.ball.size / 2.0 <= brick.pos[0] + brick.size[0]
                && self.ball.pos[1] + self.ball.size / 2.0 >= brick.pos[1]
                && self.ball.pos[1] - self.ball.size / 2.0 <= brick.pos[1] + brick.size[1];
            
            if collision {
                if self.ball.pos[0] < brick.pos[0] || self.ball.pos[0] > brick.pos[0] + brick.size[0] {
                    self.ball.vel[0] = -self.ball.vel[0];
                } else {
                    self.ball.vel[1] = -self.ball.vel[1];
                }
            }

            !collision
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.ball.pos[0] += self.ball.vel[0];
        self.ball.pos[1] += self.ball.vel[1];

        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.paddle.pos[0] -= self.paddle.speed;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.paddle.pos[0] += self.paddle.speed;
        }

        self.paddle.pos[0] = self.paddle.pos[0].max(0.0).min(800.0 - self.paddle.size[0]);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let ball_rect = graphics::Rect::new(
            self.ball.pos[0] - self.ball.size / 2.0,
            self.ball.pos[1] - self.ball.size / 2.0,
            self.ball.size,
            self.ball.size,
        );

        let ball = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::Color::WHITE,
        )?;
        graphics::draw(ctx, &ball, graphics::DrawParam::default())?;

        let paddle_rect = graphics::Rect::new(
            self.paddle.pos[0],
            self.paddle.pos[1],
            self.paddle.size[0],
            self.paddle.size[1],
        );

        let paddle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            paddle_rect,
            graphics::Color::WHITE,
        )?;
        graphics::draw(ctx, &paddle, graphics::DrawParam::default())?;

        graphics::present(ctx)?;
        Ok(())
    }
}
