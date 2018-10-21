use super::*;

const GREEN: types::Color = [0.0, 1.0, 0.0, 1.0];
const LIGHT_GREEN: types::Color = [0.0, 1.0, 0.0, 0.5];
const RED: types::Color = [1.0, 0.05, 0.05, 1.0];
const GREY: types::Color = [0.9, 0.9, 0.9, 0.9];

/// Main App with graphic engine and game state
pub struct App {
    gl: GlGraphics,       // OpenGL drawing backend.
    game: Game,           // Game state
    window: GlutinWindow, // window manager
}

impl App {
    /// Game event loop rendering, changing the game state and dispatching on key strokes
    pub fn event_loop(&mut self, events: &mut Events) {
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }
            if let Some(u) = e.update_args() {
                self.update(&u);
            }
            if let Some(p) = e.press_args() {
                let terminate = !self.key_dispatch(p);
                if terminate {
                    return;
                }
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            let transform = c.transform.trans(x, y).rot_rad(33.0).trans(-25.0, -25.0);
            rectangle(RED, square, transform, gl);
        })
    }
    pub fn update(&mut self, args: &UpdateArgs) {
        self.game.next(args.dt);
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
