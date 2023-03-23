mod obstacle;
mod player;

use bracket_lib::prelude::*;
use image::*;
use obstacle::Obstacle;
use player::Player;

enum GameMode {
    Munu,
    Playing,
    End,
}

/// 游戏屏幕宽度
const SCREEN_WIDTH: i32 = 80;
/// 游戏屏幕高度
const SCREEN_HEIGHT: i32 = 50;
/// 每隔75毫秒做一些事情
const FRAME_DURATION: f32 = 75.0;

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    score: i32, // 分数
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Munu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        // 清空屏幕
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon！");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        self.set_background(ctx, "assets/background.png");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        // frame_time_ms 记录了每次调用tick所经过的时间
        self.frame_time += ctx.frame_time_ms;
        // 向前移动并且重力增加
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        // 空格触发，往上飞
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        // 渲染
        self.player.render(ctx);
        ctx.print(0, 0, "Press Space to Flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        // 渲染障碍物
        self.obstacle.render(ctx, self.player.x);
        // 判断是否越过障碍物
        if self.player.x > self.obstacle.x {
            self.score += 1;
            // 渲染新的障碍物
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        // 如果y 大于游戏高度，就是坠地或者撞到障碍物，则游戏结束
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(5, "You are dead！");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }

    pub fn set_background(&mut self, ctx: &mut BTerm, url: &str) {
        let img = image::open(url).unwrap();
        let (img_width, img_height) = img.dimensions();
        // Draw image to console
        for x in 0..img_width {
            for y in 0..img_height {
                let pixel = img.get_pixel(x, y);
                ctx.set_bg(x as i32, y as i32, (pixel[0], pixel[1], pixel[2]));
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Munu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    // context.ba
    main_loop(context, State::new())
}
