extern crate snake_game;

use snake_game::string_rep::game_char;
use snake_game::{Direction, Game, Position};

#[test]
fn game_created_properties() {
    let (h, w) = (100, 100);
    let g = Game::new(h, w);
    for y in 0..h {
        for x in 0..w {
            let c = game_char(
                &g,
                Position {
                    x: x as i32,
                    y: y as i32,
                },
            );
            if x == w / 2 && y == h / 2 {
                assert_eq!(c, '<');
            } else {
                assert_eq!(c, ' ');
            }
        }
    }
}

#[test]
fn snake_moving() {
    let (h, w) = (100, 100);
    let mut g = Game::new(h, w);
    let pinit = Position {
        x: (w / 2) as i32,
        y: (h / 2) as i32,
    };
    assert_eq!(Some(&pinit), g.snake.get(0));
    let pnext = Position {
        x: (w / 2 - 1) as i32,
        y: (h / 2) as i32,
    };
    let ok = g.next(0.5);
    assert!(ok);
    assert_eq!(g.snake.get(0), Some(&pnext));
    let pup = Position {
        x: (w / 2 - 1) as i32,
        y: (h / 2 - 1) as i32,
    };
    g.change_dir(Direction::Up);
    let ok2 = g.next(0.5);
    assert!(ok2);
    assert_eq!(Some(&pup), g.snake.get(0))
}

#[test]
fn game_over() {
    let (h, w) = (10, 10);
    let mut g = Game::new(h, w);
    for _ in 0..5 {
        assert!(g.next(0.5));
    }
    assert!(!g.next(0.5));
}
