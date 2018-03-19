use ::*;
use graphics::{Rect, DrawMode};
use super::world::*;
use super::play::Play;

use std::path::{Path, PathBuf};

/// The state of the game
pub struct Editor {
    pos: Point2,
    fast: bool,
    level: Level,
    cur_mat: Material,
    current_mat_text: PosText,
    save: PathBuf,
}

const PALETTE: [Material; 4] = [
    Material::Grass,
    Material::Dirt,
    Material::Floor,
    Material::Wall,
];

impl Editor {
    pub fn new<P: AsRef<Path>>(assets: &Assets, save: P, ctx: &mut Context) -> GameResult<Self> {
        // Initialise the text objects
        let current_mat_text = assets.text(ctx, Point2::new(2., 18.0), "Materials:")?;
        let level = Level::load(save.as_ref()).unwrap_or_else(|_| Level::new());

        Ok(Editor {
            fast: false,
            pos: Point2::new(200., 200.),
            cur_mat: Material::Wall,
            current_mat_text,
            level,
            save: save.as_ref().to_path_buf(),
        })
    }
    /// Update the text objects
    fn update_ui(&mut self, s: &State, ctx: &mut Context) {
        let current_mat_str = format!("Materials:");

        self.current_mat_text.update_text(&s.assets, ctx, &current_mat_str).unwrap();
    }
}
const START_X: f32 = 103.;

impl GameState for Editor {
    fn update(&mut self, s: &mut State) {
        let speed = match self.fast {
            false => 175.,
            true => 315.,
        };
        let v = speed * Vector2::new(s.input.hor(), s.input.ver());
        self.pos += v * DELTA;
    }
    fn logic(&mut self, s: &mut State, ctx: &mut Context) {
        if s.mouse_down.left && s.mouse.y > 64. {
            let (mx, my) = Grid::snap(s.mouse - s.offset);
            self.level.grid.insert(mx, my, self.cur_mat);
        }

        // Update the UI
        self.update_ui(&s, ctx);
        s.focus_on(self.pos);
    }

    fn draw(&mut self, s: &State, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, graphics::WHITE)?;
        self.level.grid.draw(ctx, &s.assets)?;
        if let Some(start) = self.level.start_point {
            graphics::set_color(ctx, GREEN)?;
            graphics::circle(ctx, DrawMode::Fill, start, 16., 1.)?;
            graphics::set_color(ctx, BLUE)?;
            graphics::circle(ctx, DrawMode::Fill, start, 9., 1.)?;
        }
        for enemy in &self.level.enemies {
            enemy.draw(ctx, &s.assets)?;
        }

        Ok(())
    }
    fn draw_hud(&mut self, s: &State, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, Color{r: 0.5, g: 0.5, b: 0.5, a: 1.})?;
        graphics::rectangle(ctx, DrawMode::Fill, Rect{x:0.,y:0.,h: 64., w: s.width as f32})?;
        graphics::set_color(ctx, graphics::WHITE)?;

        for (i, mat) in PALETTE.iter().enumerate() {
            let x = START_X + i as f32 * 36.;
            if *mat == self.cur_mat {
                graphics::set_color(ctx, Color{r: 1., g: 1., b: 0., a: 1.})?;
                graphics::rectangle(ctx, DrawMode::Fill, Rect{x: x - 1., y: 15., w: 34., h: 34.})?;
                graphics::set_color(ctx, graphics::WHITE)?;
            }
            mat.draw(ctx, &s.assets, x, 16.)?;
        }

        self.current_mat_text.draw_text(ctx)
    }
    fn key_up(&mut self, s: &mut State, keycode: Keycode) {
        use Keycode::*;
        match keycode {
            Z => self.level.save(&self.save).unwrap(),
            X => self.level = Level::load(&self.save).unwrap(),
            P => s.switch(Box::new(Play::new(self.level.clone()))),
            LShift => self.fast = false,
            _ => return,
        }
    }
    fn mouse_up(&mut self, s: &mut State, btn: MouseButton) {
        use MouseButton::*;
        match btn {
            Left if s.mouse.y <= 64. => {
                if s.mouse.x > START_X && s.mouse.x < START_X + PALETTE.len() as f32 * 36. {
                    let i = ((s.mouse.x - START_X) / 36.) as usize;

                    self.cur_mat = PALETTE[i];
                }
            }
            Right => self.level.start_point = Some(s.mouse - s.offset),
            Middle => self.level.enemies.push(Enemy::new(Object::new(s.mouse - s.offset))),
            _ => ()
        }
    }
    fn key_down(&mut self, _s: &mut State, keycode: Keycode) {
        use Keycode::*;
        match keycode {
            LShift => self.fast = true,
            _ => return,
        }
    }
}
