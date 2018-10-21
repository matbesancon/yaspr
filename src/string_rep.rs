use super::*;
/// Builds representation of various objects
/// but more specifically of the Game struct
use std::string::ToString;

type Point = (Position, char);

pub const APPLE_REP: char = '@';
pub const BUSH_REP: char = '+';
pub const ROCK_REP: char = '#';
pub const GRASS_REP: char = '.';
pub const SNAKE_BODY_REP: char = '■';
pub const SNAKE_HEAD_DOWN_REP: char = 'v';
pub const SNAKE_HEAD_UP_REP: char = '∧';
pub const SNAKE_HEAD_RIGHT_REP: char = '>';
pub const SNAKE_HEAD_LEFT_REP: char = '<';

trait StringRepresent {
    fn represent(&self) -> Vec<Point>;
}

fn element_char(elt: &Element) -> char {
    match elt.kind {
        ElementKind::Apple => APPLE_REP,
        ElementKind::Bush => BUSH_REP,
        ElementKind::Rock => ROCK_REP,
        _ => GRASS_REP,
    }
}

impl StringRepresent for Map {
    fn represent(&self) -> Vec<Point> {
        self.elements
            .iter()
            .map(|el| (el.pos, element_char(el)))
            .collect()
    }
}

impl StringRepresent for Snake {
    fn represent(&self) -> Vec<Point> {
        let head = match self.direction {
            Direction::Down => SNAKE_HEAD_DOWN_REP,
            Direction::Up => SNAKE_HEAD_UP_REP,
            Direction::Left => SNAKE_HEAD_LEFT_REP,
            Direction::Right => SNAKE_HEAD_RIGHT_REP,
        };
        let mut v = vec![(*self.positions.get(0).unwrap(), head)];
        let n = self.positions.len();
        for idx in 1..n {
            v.push(((*self.positions.get(idx).unwrap()), SNAKE_BODY_REP));
        }
        v
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        let mut s = (0..self.map.height)
            .map(|_| (0..self.map.width).map(|_| ' ').collect::<Vec<_>>())
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

pub fn game_char(g: &Game, p: Position) -> char {
    let mut c = ' ';
    for elt in g.map.elements.iter() {
        if elt.pos == p {
            c = element_char(elt);
        }
    }
    // snake presence over-rides elements
    if p == *g.snake.positions.get(0).unwrap() {
        c = match g.snake.direction {
            Direction::Down => SNAKE_HEAD_DOWN_REP,
            Direction::Up => SNAKE_HEAD_UP_REP,
            Direction::Left => SNAKE_HEAD_LEFT_REP,
            Direction::Right => SNAKE_HEAD_RIGHT_REP,
        };
    } else {
        for p2 in g.snake.positions.iter() {
            if *p2 == p {
                c = SNAKE_BODY_REP;
            }
        }
    }
    c
}
