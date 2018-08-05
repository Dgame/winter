use view::View;
use console::Console;
use std::rc::Rc;
use basic::Size;
use basic::Position;

struct ViewChange {
    new_size: Size,
    pos: Position
}

pub struct Terminal {
    console: Rc<Console>,
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
        let console = Rc::new(console);
        let view = View::new(console.clone(), Position::zero(), size);

        Self {
            console,
            views: vec![view],
            active: 0,
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

            ViewChange {
                new_size,
                pos
            }
        });

        self.new_view(change.pos, change.new_size);

        self.views.len() - 1
    }

    fn new_view(&mut self, pos: Position, size: Size) {
        let view = View::new(self.console.clone(), pos, size);

        self.views.push(view);
    }

    pub fn display(&mut self) {
        for view in self.views.iter_mut() {
            view.display();
        }
    }
}