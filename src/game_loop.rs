use super::*;

const GREEN: types::Color = [0.0, 1.0, 0.0, 1.0];
const BLACK: types::Color = [0.0, 0.0, 0.0, 1.0];
const LIGHT_GREEN: types::Color = [0.0, 1.0, 0.0, 0.5];
const RED: types::Color = [1.0, 0.05, 0.05, 1.0];
const GREY: types::Color = [0.9, 0.9, 0.9, 0.9];

/// Main App with graphic engine and game state
pub struct App {
    gl: GlGraphics,         // OpenGL drawing backend.
    square_dim: u32, // width and height of each unit pixel
    game: Game,             // Game state
    window: GlutinWindow,   // window manager
}

impl App {
    pub fn new(w: usize, h: usize) -> App {
        let ogl = OpenGL::V3_2;
        let window: GlutinWindow = WindowSettings::new(
            "YASPR",
            Size{width : (w * 20) as u32,height : (h * 20) as u32}
        )
        .opengl(ogl)
        .exit_on_esc(true)
        .build()
        .unwrap();
        let g = Game::new(w, h);
        App {
            gl: GlGraphics::new(ogl),
            game: g,
            window: window,
            square_dim: 20,
        }
    }

    /// Game event loop rendering, changing the game state and dispatching on key strokes
    pub fn event_loop(&mut self) -> u32 {
        let mut events = Events::new(EventSettings::new());
        let mut dt = 0.0;
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }
            if let Some(u) = e.update_args() {
                dt += u.dt;
                if dt > 0.5 {
                    let still_playing = self.update(&u);
                    if !still_playing {
                        return self.game.score;
                    }
                    dt = 0.0;
                }
            }
            if let Some(p) = e.press_args() {
                let terminate = !self.key_dispatch(p);
                if terminate {
                    return self.game.score;
                }
            }
        }
        return self.game.score;
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let sd = self.square_dim;
        let square = rectangle::square(0.0, 0.0, sd as f64);
        let snake_pos = self.game.snake.positions.iter();
        let map_pos = self.game.map.elements.iter();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(LIGHT_GREEN, gl);
            for p in snake_pos {
                let transform = c.transform.trans(
                    p.x as f64 * sd as f64, p.y as f64 * sd as f64
                );
                let col = color(ElementKind::SnakePart);
                rectangle(col, square, transform, gl);
            }
            for elt in map_pos {
                let transform = c.transform.trans(
                    elt.pos.x as f64 * sd as f64, elt.pos.y as f64 * sd as f64
                );
                let col = color(elt.kind);
                rectangle(col, square, transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) -> bool {
        self.game.next(args.dt)
    }

    /// handles keyboard action, returns whether the game continues
    pub fn key_dispatch(&mut self, input: Button) -> bool {
        match input {
            Button::Keyboard(Key::Down) => self.game.change_dir(Direction::Down),
            Button::Keyboard(Key::Up) => self.game.change_dir(Direction::Up),
            Button::Keyboard(Key::Left) => self.game.change_dir(Direction::Left),
            Button::Keyboard(Key::Right) => self.game.change_dir(Direction::Right),
            Button::Keyboard(Key::Escape) => return false,
            _ => (),
        };
        return true;
    }
}

fn color(k: ElementKind) -> types::Color {
    match k {
        ElementKind::Apple => RED,
        ElementKind::SnakePart => BLACK,
        ElementKind::Bush => GREEN,
        ElementKind::Rock => GREY,
        ElementKind::Grass => LIGHT_GREEN,
    }
}
