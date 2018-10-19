extern crate rand;

use std::collections::VecDeque;
use std::cmp::PartialEq;
use rand::{thread_rng, ThreadRng, Rng};

const PROB_ROCK : u8 = 3; // in a range [0,9] <=> p = 0.4
const MAX_OBSTACTLE_TIME: f64 = 60.0;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ElementKind {
    Rock,
    Bush,
    Apple,
    Grass,
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Element {
    pub pos : Position,
    pub time_left : f64,
    pub kind : ElementKind,
}

#[derive(Debug)]
pub struct Snake {
    positions : VecDeque<Position>,
    direction : Direction,
}

impl Snake {

    pub fn new(start_x : u16, start_y : u16) -> Snake {
        let mut v = VecDeque::new();
        v.push_front(Position{x : start_x as i32, y : start_y as i32});
        Snake{positions : v, direction : Direction::Left}
    }

    fn next_pos(&self) -> Position {
        let init_pos = self.positions.get(0).unwrap();
        match self.direction {
            Direction::Down  => Position{x : init_pos.x, y : init_pos.y - 1},
            Direction::Up    => Position{x : init_pos.x, y : init_pos.y + 1},
            Direction::Left  => Position{x : init_pos.x - 1, y : init_pos.y},
            Direction::Right => Position{x : init_pos.x + 1, y : init_pos.y},
        }
    }

    fn move_apple(&mut self, p : Position) {
        self.positions.push_front(p);
    }

    fn move_neutral(&mut self, p : Position) {
        self.positions.push_front(p);
        self.positions.pop_back(); 
    }

    fn is_at(&self, p : Position) -> bool {
        self.positions.iter().any(|p2| p == *p2)
    }
}


/// Contains the elements on the map
#[derive(Debug)]
pub struct Map {
    pub width    : usize,
    pub height   : usize,
    pub elements : Vec<Element>,
}

impl Map {

    pub fn new(w : usize, h : usize) -> Map {
        let v = Vec::new();
        Map{width : w, height : h, elements: v}
    }


    /// fetches the element of the map at given Position
    /// None corresponds to an error => out of bound
    /// Some(e) to the element e
    fn elem_at_pos(&self, p : Position) -> Option<ElementKind> {
        if p.x < 0 || p.x >= self.width as i32 {
            return None;
        }
        if p.y < 0 || p.y >= self.height as i32 {
            return None;
        }
        let item = {
            let elems = &self.elements;
            elems.into_iter().filter(|e|
                (*e).pos == p
            ).next()
        };
        match item {
            None => Some(ElementKind::Grass),
            Some(el) => Some(el.kind)
        }
    }

    fn update_elements(&mut self, dt : f64) {
        {
            let elems = &mut self.elements;
            for mut e in elems.into_iter() {
                e.time_left -= dt;
            }
        }
        let filtered: Vec<Element> = {
            (&self).elements
                .iter()
                .filter(|e|
                    e.time_left >= 0.0 && e.kind != ElementKind::Apple).map(|e| *e
                )
                .collect()
        };
        self.elements = filtered;
    }

    fn delete_at(&mut self, p : Position) {
        let mut n = self.elements.len();
        let mut idx = 0usize;
        while idx < n {
            if self.elements[idx].pos == p {
                self.elements.remove(idx);
                n -= 1;
            } else {
                idx += 1;
            }
        }
    }
}

#[derive(Debug)]
pub struct Game {
    speed : f64,
    map : Map,
    snake : Snake,
    score : u32,
    rng : ThreadRng,
}

impl Game {

    pub fn new(w : usize, h : usize) -> Game {
        let m = Map::new(w, h);
        let s = Snake::new((w / 2) as u16, (h / 2) as u16);
        Game{
            speed : 1.0,
            map : m,
            snake : s,
            score : 0,
            rng : thread_rng(),
        }
    }

    pub fn change_dir(&mut self, d : Direction) {
        self.snake.direction = d;
    }

    fn encountered_element(&self) -> (Position, Option<ElementKind>) {
        let p = {
            let s = &self.snake;
            s.next_pos()
        };
        (p, self.map.elem_at_pos(p))
    }

    /// updates the game state
    /// returns whether game continues or not
    pub fn next(&mut self, dt : f64) -> bool {
        let enc = self.encountered_element();
        match enc {
            (_, None) => return false, // game over
            (p, Some(e)) => match e {
                ElementKind::Rock  => return false, // game over
                ElementKind::Apple => {
                    self.score += 20;
                    self.snake.move_apple(p);
                    if self.score % 3 == 0 { // game acceleration one apple out of 3
                        self.speed *= 1.05;
                    }
                    if (self.score + self.rng.gen_range(0,10)) % 5 == 0 { // 1/5 prob of new obstacle
                        self.spaw_obstacle();
                    }
                    self.map.delete_at(p);
                    self.spawn_apple();
                },
                ElementKind::Grass => {
                    self.snake.move_neutral(p);
                },
                ElementKind::Bush => {
                    self.score -= 5;
                    self.map.delete_at(p);
                    self.snake.move_neutral(p);
                },
            }
        }
        self.map.update_elements(dt);
        return true;
    }

    fn spawn_item(&mut self) -> Position {
        let r = &mut self.rng;
        loop {
            let y = r.gen_range(0,self.map.height) as i32;
            let x = r.gen_range(0,self.map.width)  as i32;
            let p = Position{x : x, y : y};
            if !self.snake.is_at(p) {
                return p;
            }
        }
    }

    fn spawn_apple(&mut self) {
        let p = self.spawn_item();
        let apple = Element{pos: p, kind : ElementKind::Apple, time_left : 1000.0};
        self.map.elements.push(apple);
    }

    fn spaw_obstacle(&mut self) {
        let is_rock = self.rng.gen_range(0,10) <= PROB_ROCK;
        let ek = if is_rock {ElementKind::Rock} else {ElementKind::Bush};
        let p = self.spawn_item();
        let tl = self.rng.gen_range(MAX_OBSTACTLE_TIME * 0.5, MAX_OBSTACTLE_TIME); 
        self.map.elements.push(Element{pos : p, kind : ek, time_left : tl});
    }
}

/// Builds representation of various objects
/// but more specifically of the Game struct
pub mod str_rep {
    use std::string::ToString;
    use super::*;

    type Point = (Position, char);

    trait Represent {
        fn represent(&self) -> Vec<Point>;
    }

    impl Represent for Map {
        fn represent(&self) -> Vec<Point> {
            self.elements.iter().map(|el|
                match el.kind {
                    ElementKind::Apple => (el.pos, '@'),
                    ElementKind::Bush  => (el.pos, '+'),
                    ElementKind::Rock  => (el.pos, '#'),
                    _ => (el.pos, '.'),
                }
            ).collect()
        }
    }

    impl Represent for Snake {
        fn represent(&self) -> Vec<Point> {
            let head = match self.direction {
                Direction::Down => 'v',
                Direction::Up   => '∧',
                Direction::Left => '>',
                Direction::Right => '<',
            };
            let mut v = vec![(*self.positions.get(0).unwrap(), head)];
            let n = self.positions.len();
            for idx in 1..n {
                v.push(((*self.positions.get(idx).unwrap()), '■'));
            }
            v
        }        
    }
        
    impl ToString for Game {
        fn to_string(&self) -> String {
            let mut s = (0..self.map.height)
                .map(|_|
                    (0..self.map.width).map(|_| ' '
                    ).collect::<Vec<_>>()
                )
                .collect::<Vec<_>>();
            let map = self.map.represent();
            let snake = self.snake.represent();
            for (p, c) in map.iter() {
                s[p.y as usize][p.x as usize] = *c;
            }
            for (p, c) in snake.iter() {
                s[p.y as usize][p.x as usize] = *c;
            }
            s.join(&'\n').iter().collect()
        }
    }
}