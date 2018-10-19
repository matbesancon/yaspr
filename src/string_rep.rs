/// Builds representation of various objects
/// but more specifically of the Game struct

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
