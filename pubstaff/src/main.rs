use std::io::stdin;
use std::io::stdout;
use std::io::Write;

static STEP: u32 = 1;

struct Position {
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

struct Sound {
    vol: u8,
}

struct Speaker {
    _place: Position,
    melody: Sound,
}

struct Person {
    place: Position,
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
