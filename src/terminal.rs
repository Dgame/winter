use basic::Position;
use basic::Size;
use console::Console;
use input::InputEvent;
use view::View;
use winapi::um::wincon::KEY_EVENT;

struct ViewChange {
    new_size: Size,
    pos: Position,
}

pub struct Terminal {
    console: Console,
    views: Vec<View>,
    active: usize,
}

impl Terminal {
    pub fn default() -> Self {
        Self::new(None, None)
    }

    pub fn new(title: Option<&str>, size: Option<Size>) -> Self {
        let mut console = Console::new();

        if let Some(title) = title {
            console.set_title(title);
        } else {
            console.set_title("Winter Console");
        }

        if let Some(size) = size {
            console.resize(size);
        }

        let size = console.get_size();
        let view = View::new(Position::zero(), size);

        Self {
            console,
            views: vec![view],
            active: 0,
        }
    }

    pub fn move_cursor_lhs(&mut self) {
        let mut pos = self.get_active_view().get_write_pos();
        if pos.x > 0 {
            pos.x -= 1;
            self.get_active_view().set_write_pos(pos);
            self.console.set_cursor_pos(pos);
        }
    }

    pub fn move_cursor_rhs(&mut self) {
        let mut pos = self.get_active_view().get_write_pos();
        if pos.x < self.get_active_view().get_line_offset() {
            pos.x += 1;
            self.get_active_view().set_write_pos(pos);
            self.console.set_cursor_pos(pos);
        }
    }

    fn get_active_view(&mut self) -> &mut View {
        self.views.get_mut(self.active).expect("No active View?!")
    }

    fn adjust_view(view: &mut View, new_size: Size) {
        view.resize(new_size);
        view.clear_back_buffer(); // Sonst wird nix angezeigt
    }

    fn split_with<F: Fn(&mut View) -> ViewChange>(&mut self, closure: F) -> ViewChange {
        self.console.clear();

        let view = self.get_active_view();
        closure(view)
    }

    pub fn vsplit(&mut self) -> usize {
        let change = self.split_with(|view| {
            let new_size = view.get_size().with_half_width();
            Self::adjust_view(view, new_size);

            let mut pos = view.get_position();
            pos.x += new_size.width;

            ViewChange { new_size, pos }
        });

        self.new_view(change.pos, change.new_size);

        self.views.len() - 1
    }

    fn new_view(&mut self, pos: Position, size: Size) {
        let view = View::new(pos, size);

        self.views.push(view);
    }

    pub fn get_input_events(&mut self) -> Vec<InputEvent> {
        let mut inputs = Vec::new();
        for input in self.console.get_input() {
            match input.EventType {
                KEY_EVENT => inputs.push(InputEvent::from(&input)),
                _ => {}
            }
        }

        inputs
    }

    pub fn newline(&mut self) {
        let pos = self.get_active_view().newline();
        self.console.set_cursor_pos(pos);
    }

    pub fn write<S: Into<String>>(&mut self, input: S) {
        let pos = self.get_active_view().write(input);
        self.console.set_cursor_pos(pos);
    }

    pub fn remove_lhs(&mut self) {
        let pos = self.get_active_view().remove_lhs();
        self.console.set_cursor_pos(pos);
    }

    pub fn remove_rhs(&mut self) {
        let pos = self.get_active_view().remove_rhs();
        self.console.set_cursor_pos(pos);
    }

    pub fn display(&mut self) {
        for view in self.views.iter_mut() {
            view.display(&mut self.console);
        }
    }
}
