// Basics
// - Syntax
// - Scalars: bool, char, numbers (unsigned/signed, int/float), str
// - Variables
// - Name case conventions
// - Statically typed

fn main() {
    let x: i32 = 10;
    let y: i32 = 20;
    let is_even: bool = x % 2 == 0;

    // tuple
    let position: (i32, i32) = (x, y);

    // array
    let uv: [f32; 2] = [0.3, 12.5];

    // type inference
    let z = x + y;
    let is_complete = true;

    // destructuring
    let (px, py) = position;
}

// Immutability
// - Everything is immutable by default
// - Mutation allowed but need to be explicit

fn mutablity() {
    let x = 10;

    // error
    // x = 5;

    let mut x = 10;
    x = 5;

    let mut message = String::from("Hello");
    message.push_str(" world");
}

// Functions
// - Type signatures required, return type optional
// - Can implicitly return values
// - Can be defined anonymously
// - Can reference variables in outer scope (closures)
// - No auto currying
// - Generics

fn greet(name: String) {
    println!("Hello, {}", name);
}

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

// generic
fn wrap<T>(id: u32, value: T) -> (u32, T) {
    (id, value)
}

fn working_with_functions() {
    greet(String::from("Derek"));

    let n = 10;
    let even_result = is_even(n);

    let square = |x: u32| x * x;
    let n_squared = square(n);

    let numbers: Vec<u32> = vec![0, 1, 2];
    let numbers_squared = numbers.into_iter().map(square);
}

// fn as value
fn all_even(xs: Vec<u32>) -> bool {
    xs.into_iter().all(is_even)
}

// closures
fn multiply_all_by(xs: Vec<u32>, amount: u32) -> Vec<u32> {
    xs.into_iter().map(|x| x * amount).collect()
}

// Advanced closure
// https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html
fn make_adder(amount: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| amount + x)
}

// Control Flow
// - if/else expressions
// - pattern matching

fn control_flow() {
    let cents = 25;

    let coin = if cents == 1 {
        "penny"
    } else if cents == 5 {
        "nickel"
    } else {
        ""
    };

    let coin = match cents {
        1 => "penny",
        5 => "nickel",
        _ => "",
    };
}

// Structs and Enums

struct Point {
    x: u32,
    y: u32,
}

enum Team {
    Home,
    Away,
}

struct PlayerTrack {
    position: Point,
    team: Team,
    jersey_number: String,
}

fn working_with_structs_and_enums() {
    let position = Point { x: 0, y: 0 };
    let team = Team::Home;

    let player_track = PlayerTrack {
        position,
        team,
        jersey_number: String::from("10"),
    };

    let jersey_number = player_track.jersey_number;

    let PlayerTrack { team, .. } = player_track;
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn step(position: Point, to: Direction) -> Point {
    let Point { x, y } = position;

    match to {
        Direction::Up => Point { x, y: y + 1 },
        Direction::Down => Point { x, y: y - 1 },
        Direction::Left => Point { x: x - 1, y },
        Direction::Right => Point { x: x + 1, y },
    }
}

enum Msg {
    Start,
    Move(Direction),
    Run(u32, Direction), // Alternativlly, could use anonymous struct: Run { speed: u32, direction: Direction },
}

struct Model {
    is_playing: bool,
    position: Point,
}

fn update(model: Model, msg: Msg) -> Model {
    match msg {
        Msg::Start => Model {
            is_playing: true,
            ..model
        },
        Msg::Move(direction) => Model {
            position: step(model.position, direction),
            ..model
        },
        Msg::Run(speed, direction) => todo!("implement run msg"),
    }
}

// Generics

struct QueryResults<T> {
    page: u32,
    next: bool,
    results: Vec<T>,
}

fn is_last_page<T>(results: QueryResults<T>) -> bool {
    results.next
}

enum Maybe<T> {
    Just(T),
    Nothing,
}

// Option
// - More Pattern Matching

// enum Option<T> {
//     Some(T),
//     None,
// }

fn step_if_vertical(position: Point, to: Direction) -> Option<Point> {
    let Point { y, .. } = position;

    // Matches Are Exhaustive
    match to {
        Direction::Up => Some(Point {
            y: y + 1,
            ..position
        }),
        Direction::Down => Some(Point {
            y: y - 1,
            ..position
        }),
        _ => None,
    }
}

fn working_with_options() {
    let option = Option::Some(String::from("10")); // more conventional: Some(10);

    // match
    let other_option = match option {
        Some(s) => Some(s.len()),
        None => None,
    };

    // if let

    let option = Some(String::from("10"));

    let other_option = if let Some(s) = option {
        Some(s.len())
    } else {
        None
    };

    // map

    let option = Some(String::from("10"));

    let other_option = option.map(|s| s.len());

    // and_then

    let option = Some(String::from("10"));

    let other_option = option.and_then(|s| s.find("1"));

    // filter_map
    let strings = vec!["10", "20", "30"];
    let matches: Vec<_> = strings.iter().filter_map(|s| s.find("1")).collect();
}

// Collections

use std::collections::{HashMap, HashSet};

fn working_with_collections() {
    // mutable
    let mut vector = Vec::new();

    vector.push(1);
    vector.push(2);

    //
    let squared = vector.iter().map(|i| i * i).collect::<Vec<_>>();

    // immutable
    let vector = vec![1, 2];

    // vector.push(1);

    // hashset

    let mut set = HashSet::new();

    set.insert(1);

    let have_seen_one = set.contains(&1);

    // hashmap

    let mut hashmap = HashMap::new();

    hashmap.insert("name", "derek");

    if let Some(name) = hashmap.get("name") {
        println!("Hello, {}", name);
    }
}
