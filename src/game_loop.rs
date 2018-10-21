use super::*;

const GREEN: types::Color = [0.0, 1.0, 0.0, 1.0];
const LIGHT_GREEN: types::Color = [0.0, 1.0, 0.0, 0.5];
const RED: types::Color = [1.0, 0.05, 0.05, 1.0];
const GREY: types::Color = [0.9, 0.9, 0.9, 0.9];

/// Main App with graphic engine and game state
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    game: Game,     // Game state
}

impl App {
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
}
