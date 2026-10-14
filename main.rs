use std::{
    error::Error,
    io::{stdout, Write},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

const WIDTH: u16 = 30;
const HEIGHT: u16 = 16;
const TICK_MS: u64 = 145;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(self) -> (i16, i16) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

struct Game {
    snake: Vec<(u16, u16)>,
    direction: Direction,
    next_direction: Direction,
    food: (u16, u16),
    score: u32,
    paused: bool,
    game_over: bool,
    rng: u64,
}

impl Game {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        let mut game = Self {
            snake: vec![
                (WIDTH / 2, HEIGHT / 2),
                (WIDTH / 2 - 1, HEIGHT / 2),
                (WIDTH / 2 - 2, HEIGHT / 2),
            ],
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: (2, 2),
            score: 0,
            paused: false,
            game_over: false,
            rng: seed.max(1),
        };
        game.food = game.random_food().unwrap_or((2, 2));
        game
    }

    fn queue_direction(&mut self, direction: Direction) {
        if direction != self.direction.opposite() {
            self.next_direction = direction;
        }
    }

    fn step(&mut self) {
        if self.paused || self.game_over {
            return;
        }

        self.direction = self.next_direction;
        let (dx, dy) = self.direction.delta();
        let (head_x, head_y) = self.snake[0];
        let next_x = head_x as i16 + dx;
        let next_y = head_y as i16 + dy;

        if next_x < 0 || next_y < 0 || next_x >= WIDTH as i16 || next_y >= HEIGHT as i16 {
            self.game_over = true;
            return;
        }

        let next_head = (next_x as u16, next_y as u16);
        let will_eat = next_head == self.food;
        // The tail moves away on a normal step, so entering its current cell is allowed.
        let occupied_len = if will_eat {
            self.snake.len()
        } else {
            self.snake.len().saturating_sub(1)
        };
        if self
            .snake
            .iter()
            .take(occupied_len)
            .any(|segment| *segment == next_head)
        {
            self.game_over = true;
            return;
        }

        self.snake.insert(0, next_head);
        if will_eat {
            self.score += 1;
            if let Some(food) = self.random_food() {
                self.food = food;
            } else {
                self.game_over = true;
            }
        } else {
            self.snake.pop();
        }
    }

    fn tick_duration(&self) -> Duration {
        Duration::from_millis(TICK_MS.saturating_sub(self.score as u64 * 3).max(55))
    }

    fn random_food(&mut self) -> Option<(u16, u16)> {
        let mut empty = Vec::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if !self.snake.iter().any(|segment| *segment == (x, y)) {
                    empty.push((x, y));
                }
            }
        }
        if empty.is_empty() {
            return None;
        }

        // Small xorshift generator keeps the game dependency-light and deterministic per run.
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        Some(empty[(self.rng as usize) % empty.len()])
    }
}

struct TerminalGuard;

impl TerminalGuard {
    fn new() -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;
        if let Err(error) = execute!(stdout(), EnterAlternateScreen, Hide) {
            let _ = disable_raw_mode();
            return Err(Box::new(error));
        }
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(stdout(), Show, LeaveAlternateScreen, ResetColor);
        let _ = disable_raw_mode();
    }
}

fn render(out: &mut impl Write, game: &Game) -> Result<(), Box<dyn Error>> {
    let (terminal_width, terminal_height) = terminal::size()?;
    queue!(out, MoveTo(0, 0), Clear(ClearType::All))?;

    if terminal_width < WIDTH * 2 + 2 || terminal_height < HEIGHT + 6 {
        queue!(
            out,
            SetForegroundColor(Color::Yellow),
            Print("Terminal is too small. Please enlarge it and try again."),
            ResetColor,
        )?;
        out.flush()?;
        return Ok(());
    }

    let border = "─".repeat((WIDTH * 2) as usize);
    queue!(
        out,
        SetForegroundColor(Color::Cyan),
        Print(" RUST SNAKE  "),
        SetForegroundColor(Color::White),
        Print(format!(
            "Score: {}   Best speed: {}ms\r\n",
            game.score,
            game.tick_duration().as_millis()
        )),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("┌{border}┐\r\n")),
    )?;

    for y in 0..HEIGHT {
        queue!(out, SetForegroundColor(Color::DarkGrey), Print("│"))?;
        for x in 0..WIDTH {
            let point = (x, y);
            if game.snake.first() == Some(&point) {
                queue!(out, SetForegroundColor(Color::Green), Print("██"))?;
            } else if game.snake.iter().skip(1).any(|segment| *segment == point) {
                queue!(out, SetForegroundColor(Color::DarkGreen), Print("▓▓"))?;
            } else if game.food == point {
                queue!(out, SetForegroundColor(Color::Red), Print("◆ "))?;
            } else {
                queue!(out, Print("  "))?;
            }
        }
        queue!(out, SetForegroundColor(Color::DarkGrey), Print("│\r\n"))?;
    }

    queue!(
        out,
        Print(format!("└{border}┘\r\n")),
        SetForegroundColor(Color::Grey)
    )?;
    if game.game_over {
        queue!(
            out,
            SetForegroundColor(Color::Red),
            Print("GAME OVER — press R to restart or Q to quit\r\n")
        )?;
    } else if game.paused {
        queue!(
            out,
            SetForegroundColor(Color::Yellow),
            Print("PAUSED — press P or Space to continue\r\n")
        )?;
    } else {
        queue!(
            out,
            Print("Move: WASD / arrows   Pause: P / Space   Restart: R   Quit: Q / Esc\r\n")
        )?;
    }
    queue!(out, ResetColor)?;
    out.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _terminal = TerminalGuard::new()?;
    let mut out = stdout();
    let mut game = Game::new();
    let mut last_tick = Instant::now();

    loop {
        render(&mut out, &game)?;

        if event::poll(Duration::from_millis(15))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        game = Game::new();
                        last_tick = Instant::now();
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') | KeyCode::Char(' ') => {
                        if !game.game_over {
                            game.paused = !game.paused;
                            last_tick = Instant::now();
                        }
                    }
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                        game.queue_direction(Direction::Up)
                    }
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                        game.queue_direction(Direction::Down)
                    }
                    KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                        game.queue_direction(Direction::Left)
                    }
                    KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                        game.queue_direction(Direction::Right)
                    }
                    _ => {}
                }
            }
        }

        if !game.paused && !game.game_over && last_tick.elapsed() >= game.tick_duration() {
            game.step();
            last_tick = Instant::now();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prevents_reversing_into_itself() {
        let mut game = Game::new();
        game.queue_direction(Direction::Left);
        assert_eq!(game.next_direction, Direction::Right);
        game.queue_direction(Direction::Up);
        assert_eq!(game.next_direction, Direction::Up);
    }

    #[test]
    fn grows_when_eating_food() {
        let mut game = Game::new();
        game.snake = vec![(4, 4), (3, 4), (2, 4)];
        game.direction = Direction::Right;
        game.next_direction = Direction::Right;
        game.food = (5, 4);
        game.step();
        assert_eq!(game.score, 1);
        assert_eq!(game.snake.len(), 4);
    }

    #[test]
    fn ends_when_hitting_wall() {
        let mut game = Game::new();
        game.snake = vec![(WIDTH - 1, HEIGHT / 2), (WIDTH - 2, HEIGHT / 2)];
        game.direction = Direction::Right;
        game.next_direction = Direction::Right;
        game.step();
        assert!(game.game_over);
    }
}
