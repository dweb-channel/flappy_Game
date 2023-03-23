use bracket_lib::prelude::*;

use crate::{player::Player, SCREEN_HEIGHT};
/// gap_y - (size/2)
/// gap_y
/// gap_y + (size/2)
pub struct Obstacle {
    pub x: i32,     // 整个世界空间的横坐标
    pub gap_y: i32, // 中间的空隙坐标
    pub size: i32,  // 空隙大小
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),   // 不包含40
            size: i32::max(2, 20 - score), // score玩家的积分，积分越多洞越窄
        }
    }
    // 渲染障碍物
    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x; // 屏幕上的空间
        let half_size = self.size / 2;

        // 上边的
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        // 下边的
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
        }
    }
    /// 判断是否撞到
    pub fn hit_obstacle(&self, player: &Player) -> bool {
      let half_size = self.size / 2;
      // 玩家的x坐标和障碍物的坐标是否一样
      let does_x_match = player.x == self.x;
      // 是否在上面的障碍物的范围内
      let player_above_gap = player.y < self.gap_y - half_size;
      // 是否在下面障碍物的坐标范围内
      let player_below_gap = player.y > self.gap_y + half_size;
      does_x_match && (player_above_gap || player_below_gap)
    }
}
