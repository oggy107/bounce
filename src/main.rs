use std::fmt::{Display, Error, Formatter};

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
    ball: Ball,
    frame: Frame,
}

impl Game {
    fn new() -> Game {
        Game {
            ball: Ball {
                x: 44,
                y: 21,
                vert_dir: VertDir::Down,
                horiz_dir: HorizDir::Right,
            },
            frame: Frame {
                width: 63,
                height: 31,
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
        } else if self.y <= 0 {
            self.vert_dir = VertDir::Down;
        } else if frame.height <= self.y {
            self.vert_dir = VertDir::Up;
        } else {
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
        for _ in 0..64 {
            write!(fmt, "-")?;
        }

        for y in 0..32 {
            for x in 0..64 {
                if self.ball.x == x && self.ball.y == y {
                    write!(fmt, "O")?;
                }

                if x == 0 || x == 63 {
                    write!(fmt, "|")?;
                } else if x != 0 && y != 31 {
                    write!(fmt, " ")?;
                } else {
                    write!(fmt, "-")?;
                }
            }

            write!(fmt, "\n")?;
        }

        write!(fmt, "\n")?;
        Ok(())
    }
}

fn main() {
    let mut game = Game::new();
    let sleep_dur = std::time::Duration::from_millis(33);

    loop {
        println!("{}", game);
        game.step();
        std::thread::sleep(sleep_dur);
        println!("{} {}", game.ball.x, game.ball.y);
    }
}
