use std::io::stdin;
use std::io::stdout;
use std::io::Write;

static STEP: u32 = 1;

pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn go_left(&mut self) {
        self.x -= STEP;
    }

    fn go_right(&mut self) {
        self.x += STEP;
    }

    fn go_up(&mut self) {
        self.y += STEP;
    }

    fn go_down(&mut self) {
        self.y -= STEP;
    }
}

pub struct Sound {
    vol: u8,
}

struct Speaker {
    _place: Position,
    melody: Sound,
}

struct Person {
    place: Position,
}

pub fn calculate_loudness(s: Position, o: Position) -> Sound{
    let dx: i32 = (o.x as i64 - s.x as i64) as i32;
    let dy: i32 = (o.y as i64 - s.y as i64) as i32;
    let d = ((dx.abs().pow(2) + dy.abs().pow(2)) as f32).sqrt();
    let v = (100.0 * 1.0/(d.powf(2.0) + 1.0)) as u8;
    let s: Sound = Sound{ vol: v};
    s
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn simple_test(){
        assert_eq!(calculate_loudness(Position{x:0,y:0}, Position{x:1,y:0}).vol, 50);
    }
}

fn main() {
    println!("Hello, PUB Chat!");

    let mut you = Person {
        place: Position { x: 50, y: 50 },
    };

    let speaker1 = Speaker {
        _place: Position { x: 0, y: 0 },
        melody: Sound { vol: 100 },
    };

    loop {
        let mut line = String::new();
        print!("What to do? ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        match line.as_str() {
            "up\n" => you.place.go_up(),
            "down\n" => you.place.go_down(),
            "left\n" => you.place.go_left(),
            "right\n" => you.place.go_right(),
            "quit\n" => break,
            _ => println!("I can't do it."),
        }

        println!("Your position: {} {}", you.place.x, you.place.y);
        println!("Speaker volume: {}", speaker1.melody.vol);
    }
}
