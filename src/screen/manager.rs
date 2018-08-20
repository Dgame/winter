use basic::{Coord, Size, Viewport};
use screen::Screen;
use cli::Console;

pub struct ScreenManager {
    screens: Vec<Screen>,
    screen_id: usize,
}

impl ScreenManager {
    pub fn new(size: Size) -> Self {
        let viewport = Viewport::with(Coord::zero(), size);
        let screen = Screen::new(viewport);
        let mut screens = Vec::with_capacity(4);
        screens.push(screen);

        Self {
            screens,
            screen_id: 0,
        }
    }

    pub fn screen(&self) -> &Screen {
        &self.screens[self.screen_id]
    }

    pub fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screens[self.screen_id]
    }

    pub fn switch_screen(&mut self, screen_id: usize) {
        self.screen_id = screen_id;
    }

    pub fn render(&mut self, console: &mut Console) {
        for screen in self.screens.iter_mut() {
            screen.render(console);
        }
    }
}
