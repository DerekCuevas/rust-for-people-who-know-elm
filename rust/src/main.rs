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

    let mut x: f64 = 10.0;
    x = 5_f64;

    let x = 10;
    let x = 12;

    // x = "not allowed!";

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

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

// generic
fn wrap<T>(id: u32, value: T) -> (u32, T) {
    (id, value)
}

fn working_with_functions() {
    let mut name = String::from("Derek");

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

// #[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
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

    // { model | field = foo }
    match to {
        Direction::Up => Point {
            y: y + 1,
            ..position
        },
        Direction::Down => Point { x, y: y - 1 },
        Direction::Left => Point { x: x - 1, y },
        Direction::Right => Point { x: x + 1, y },
    }
}

enum Msg {
    Start,
    Move(Direction),
    Run(u32, Direction),
    // Alternativlly, could use anonymous struct:
    // Run { speed: u32, direction: Direction },
}

struct Model {
    is_playing: bool,
    position: Point,
}

fn update(model: Model, msg: Msg) -> Model {
    // // let move_msg = Msg::Move(Direction::Down);

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
    results: [T; 3],
}

fn is_last_page<T>(results: QueryResults<T>) -> bool {
    let mock_results: [u32; 3] = [1, 2, 3];

    let qr = QueryResults {
        page: 0,
        next: false,
        results: mock_results,
    };

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

// fn get_size(foo: String) -> usize {}

fn working_with_options() {
    let option = Some(String::from("10")); // more conventional: Some(10);
                                           // let size: usize = 10;

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

    let squared: i64 = vector.iter().map(|i| i * i).sum();

    // immutable
    let mut vector = vec![1, 2];

    vector.push(1);

    // hashset

    let mut set = HashSet::new();

    set.insert(1);

    let have_seen_one = set.contains(&1);

    // set.iter().map(|x| x);

    // hashmap

    let mut hashmap = HashMap::new();

    hashmap.insert("name", "derek");

    if let Some(name) = hashmap.get("name") {
        println!("Hello, {}", name);
    }
}

// Memory Safety
// No garbage collection
// Ownership: Move, Borrow, Copy, Clone
// Lifetimes: The scope within which a borrowed reference is valid

fn greet(name: String) {
    println!("Hello, {}", name);
    // s is dropped
    // drop(s);
}

fn ownership() {
    // Rules:
    // Each value in Rust has a variable that's called its owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    let name = "Derek".to_string();
    let new_owner_of_name = name;

    // name is moved
    // println!("{}", name);

    let clone_of_new_owner_of_name = new_owner_of_name.clone();

    greet(new_owner_of_name);

    // new_owner_of_name is moved
    // println!("{}", new_owner_of_name);

    println!("{}", clone_of_new_owner_of_name);

    // clone_of_new_owner_of_name is dropped
}

fn is_name_valid(name: &String) -> bool {
    name.len() < 20
}

fn truncate(s: &mut String, new_len: usize) {
    s.truncate(new_len);
}

// You may have one or the other of these two kinds of borrows, but not both at the same time:
// - one or more references (&T) to a resource
// - exactly one mutable reference (&mut T)
fn multiple_readers_or_single_writers() {
    let mut name = "Derek".to_string();

    let name_ref_a = &name;
    let name_ref_b = &name;

    let is_value = is_name_valid(name_ref_a);
    let is_still_value = is_name_valid(name_ref_b);

    let name_mut_ref = &mut name;

    // &mut is now in scope, cannot also have immutable reference
    // let name_ref_c = &name;

    truncate(name_mut_ref, 2);

    // name_mut_ref is now out of scope
    let name_ref_d = &name;
}

// More structs and enums
// Copy, Clone
// Methods
// Error handling (? operator)

fn is_within_bounds(p: Point, max_x: usize, max_y: usize) -> bool {
    p.x.abs() as usize <= max_x && p.y.abs() as usize <= max_y
}

fn city_block_distance(a: &Point, b: &Point) -> usize {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as usize
}

impl Point {
    pub fn new() -> Self {
        Point { x: 0, y: 0 }
    }

    // &self vs self
    pub fn is_origin(self) -> bool {
        self.x == 0 && self.y == 0
    }

    pub fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

impl Direction {
    pub fn is_positive(&self) -> bool {
        match self {
            Self::Up | Self::Right => true,
            _ => false,
        }
    }
}

fn working_more_with_structs_and_enums() {
    let start = Point { x: 0, y: 0 };
    let end = Point { x: 3, y: -4 };

    // is_within_bounds(end, 10, 10);

    // end is moved
    // but can derive Copy, Clone on Point since all fields are Copy, Clone
    let distance = city_block_distance(&start, &end);

    let mut origin = Point::new();

    // move (since not &self)
    // let is_origin = origin.is_origin();

    let direction = Direction::Down;
    let is_pos = direction.is_positive();

    // two issues
    // start.step(direction);

    origin.step(direction);
}

// Traits
// https://blog.rust-lang.org/2015/05/11/traits.html

// Iterators
// https://doc.rust-lang.org/std/iter/trait.Iterator.html
// is_even iterator adapter
