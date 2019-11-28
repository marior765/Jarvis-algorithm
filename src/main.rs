use std::error::Error as StdError;
use std::io::stdin;
use std::result::Result as StdResult;

type Result<T> = StdResult<T, Box<dyn StdError>>;

struct Point {
    x: i8,
    y: i8
}

fn determinate_signum(a: &Point, b: &Point, c: &Point) -> bool 
{
    (((b.x - a.x) * (c.x - b.y) - (c.x - b.x) * (b.y - a.y)) >= 0)
}

fn jarvis(source: Vec<Point>, result: Vec<Point>)
{
    if (source.is_empty()) {
        return;
    }

    let src: Vec<Point> = source.clone();
}

fn read_from_console() -> Result<()> {
    println!("Hello, there!  What is your name?");

    let buffer = &mut String::new();
    stdin().read_line(buffer)?; // <- API requires buffer param as of Rust 1.0; returns `Result` of bytes read
    let res = match buffer.trim_end() {
        "" => "Ah, well I can respect your wish to remain anonymous.".to_owned(),
        name => format!("Hello, {}.  Nice to meet you!", name),
    };
    println!("{}", res);

    Ok(())
}

fn main() {
    read_from_console();
    println!("Hello, world!");
}
