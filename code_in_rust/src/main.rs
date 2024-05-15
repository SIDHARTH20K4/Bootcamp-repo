use std::fmt::{Display, Formatter, Error};
use std::time::{Duration};

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: i32,
    y: i32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: i32,
    height: i32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {
        Game {
            frame: Frame {
                width: 63,
                height: 31,
            },
            ball: Ball {
                x: 44,
                y: 21,
                vert_dir: VertDir::Down,
                horiz_dir: HorizDir::Right,
            },
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x <= 0 {
            self.horiz_dir = HorizDir::Right;
        } else if frame.width <= self.x {
            self.horiz_dir = HorizDir::Left;
        } else if frame.height <= self.y {
            self.vert_dir = VertDir::Up;
        } else {
            // Handle other cases if needed
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }

        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "Game Board:\n");
        for _ in 0..64 {
            write!(fmt, "-");
        }
        write!(fmt, "\n");
        for y in 0..32 {
            for x in 0..64 {
                if self.ball.x == x && self.ball.y == y {
                    write!(fmt, "0");
                } else if x == 0 {
                    write!(fmt, "!");
                } else {
                    write!(fmt, " ");
                }
            }
            write!(fmt, "\n");
        }
        Ok(())
    }
}

fn main() {
    let mut new_game: Game = Game::new();
    let sleep_time: Duration = Duration::from_millis(33);
    loop {
        println!("{}", new_game);
        new_game.step();
        std::thread::sleep(sleep_time);
        println!("Ball coordinates: ({}, {})", new_game.ball.x, new_game.ball.y);
    }
}
