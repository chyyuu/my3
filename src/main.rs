use ncurses::*;
use rand::Rng;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

const WIDTH: u16 = 120;
const HEIGHT: u16 = 30;
const SPEED: u64 = 150; // 游戏速度(毫秒)

struct Game {
    snake: VecDeque<(u16, u16)>,
    direction: Direction,
    food: (u16, u16),
    game_over: bool,
    score: u32,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let food = (
            rng.gen_range(1..WIDTH - 1),
            rng.gen_range(1..HEIGHT - 1),
        );
        let mut snake = VecDeque::new();
        snake.push_back((WIDTH / 2, HEIGHT / 2));
        snake.push_back((WIDTH / 2 - 1, HEIGHT / 2));
        snake.push_back((WIDTH / 2 - 2, HEIGHT / 2));
        
        Self {
            snake,
            direction: Direction::Right,
            food,
            game_over: false,
            score: 0,
        }
    }

    fn draw(&self) {
        erase();
        // 绘制边框和分数
        mvprintw(0, 0, &("┌".to_owned() + &"─".repeat(WIDTH as usize) + "┐"));
        mvprintw(1, 0, &("│ 贪吃蛇游戏 - 分数: {:03} ".to_string() + &" ".repeat(WIDTH as usize - 17) + "│"));
        mvprintw(2, 0, &("├".to_owned() + &"─".repeat(WIDTH as usize) + "┤"));
        
        for y in 0..HEIGHT {
            mvprintw((y + 3) as i32, 0, "│");
            for x in 0..WIDTH {
                if self.snake.front() == Some(&(x, y)) {
                    mvaddch((y + 3) as i32, (x + 1) as i32, '●' as u32); // 蛇头
                } else if self.snake.contains(&(x, y)) {
                    mvaddch((y + 3) as i32, (x + 1) as i32, '○' as u32); // 蛇身
                } else if (x, y) == self.food {
                    mvaddch((y + 3) as i32, (x + 1) as i32, '★' as u32); // 食物
                }
            }
            mvprintw((y + 3) as i32, (WIDTH + 1) as i32, "│");
        }
        mvprintw((HEIGHT + 3) as i32, 0, &("└".to_owned() + &"─".repeat(WIDTH as usize) + "┘"));
        mvprintw((HEIGHT + 4) as i32, 0, "控制: ↑ ↓ ← →  退出: ESC");
        refresh();
    }

    fn update(&mut self) {
        let head = *self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1.saturating_sub(1)),
            Direction::Down => (head.0, head.1.saturating_add(1)),
            Direction::Left => (head.0.saturating_sub(1), head.1),
            Direction::Right => (head.0.saturating_add(1), head.1),
        };

        // 碰撞检测
        if new_head.0 == 0 || new_head.0 >= WIDTH - 1 
            || new_head.1 == 0 || new_head.1 >= HEIGHT - 1 
            || self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.push_front(new_head);
        if new_head == self.food {
            self.score += 10;
            let mut rng = rand::thread_rng();
            self.food = (
                rng.gen_range(1..WIDTH - 1),
                rng.gen_range(1..HEIGHT - 1),
            );
            // 确保食物不出现在蛇身上
            while self.snake.contains(&self.food) {
                self.food = (
                    rng.gen_range(1..WIDTH - 1),
                    rng.gen_range(1..HEIGHT - 1),
                );
            }
        } else {
            self.snake.pop_back();
        }
    }

    fn handle_input(&mut self) {
        nodelay(stdscr(), true);
        let input = getch();
        match input {
            KEY_UP if self.direction != Direction::Down => {
                self.direction = Direction::Up
            }
            KEY_DOWN if self.direction != Direction::Up => {
                self.direction = Direction::Down
            }
            KEY_LEFT if self.direction != Direction::Right => {
                self.direction = Direction::Left
            }
            KEY_RIGHT if self.direction != Direction::Left => {
                self.direction = Direction::Right
            }
            27 => self.game_over = true,
            _ => {}
        }
    }
}

fn main() {
    // 初始化ncurses
    initscr();
    cbreak();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    erase();

    // 游戏主循环
    let mut game = Game::new();
    let mut last_update = Instant::now();

    while !game.game_over {
        game.handle_input();
        
        if last_update.elapsed() >= Duration::from_millis(SPEED) {
            game.update();
            game.draw();
            last_update = Instant::now();
        }
        napms(SPEED as i32);
    }

    // 清理ncurses
    endwin();
    println!("游戏结束! 最终分数: {}", game.score);
}


