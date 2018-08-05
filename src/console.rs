use basic::Position;
use basic::Size;
use cell::Cell;
use std::cmp::max;
use std::cmp::min;
use std::ffi::CString;
use std::io::stdout;
use std::io::Write;
use std::mem::zeroed;
use std::ptr;
use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::HWND;
use winapi::um::consoleapi::{GetConsoleMode, GetNumberOfConsoleInputEvents, SetConsoleMode};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
use winapi::um::wincon::{
    CHAR_INFO_Char, FillConsoleOutputAttribute, FillConsoleOutputCharacterA, GetConsoleCursorInfo,
    GetConsoleScreenBufferInfo, GetConsoleWindow, SetConsoleCP, SetConsoleCursorInfo,
    SetConsoleCursorPosition, SetConsoleScreenBufferSize, SetConsoleTitleA, SetConsoleWindowInfo,
    WriteConsoleOutputA, CHAR_INFO, CONSOLE_CURSOR_INFO, CONSOLE_SCREEN_BUFFER_INFO, COORD,
    ENABLE_MOUSE_INPUT, ENABLE_WINDOW_INPUT, SMALL_RECT,
};
use winapi::um::winnt::HANDLE;
use winapi::um::winuser::{SetWindowPos, SWP_NOSIZE, SWP_NOZORDER};

trait Empty {
    fn empty() -> Self;
}

impl Empty for COORD {
    fn empty() -> COORD {
        COORD { X: 0, Y: 0 }
    }
}

impl Empty for SMALL_RECT {
    fn empty() -> SMALL_RECT {
        SMALL_RECT {
            Top: 0,
            Right: 0,
            Bottom: 0,
            Left: 0,
        }
    }
}

impl Empty for CONSOLE_SCREEN_BUFFER_INFO {
    fn empty() -> CONSOLE_SCREEN_BUFFER_INFO {
        CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD::empty(),
            dwCursorPosition: COORD::empty(),
            wAttributes: 0,
            srWindow: SMALL_RECT::empty(),
            dwMaximumWindowSize: COORD::empty(),
        }
    }
}

pub struct Console {
    handle: HWND,
    output: HANDLE,
    input: HANDLE,
    mode: DWORD,
    restore_mode: DWORD,
    screen_buffer_info: CONSOLE_SCREEN_BUFFER_INFO,
    initial_size: Size,
}

impl Drop for Console {
    fn drop(&mut self) {
        self.resize(self.initial_size);

        unsafe {
            SetConsoleMode(self.input, self.restore_mode);
            SetConsoleScreenBufferSize(self.output, self.screen_buffer_info.dwSize);
        }

        self.clear();
        self.set_cursor(Position::new(0, 0));
        self.cursor_visible(true);
    }
}

impl Console {
    pub fn new() -> Self {
        let handle = unsafe { GetConsoleWindow() };
        unsafe {
            assert_ne!(
                ptr::null(),
                handle,
                "Could not get console window handle: ERROR {}",
                GetLastError()
            );
        }

        let output = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        unsafe {
            assert_ne!(
                INVALID_HANDLE_VALUE,
                output,
                "Could not get standard output handle: ERROR {}",
                GetLastError()
            );
        }

        let input = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
        unsafe {
            assert_ne!(
                INVALID_HANDLE_VALUE,
                input,
                "Could not get standard window input handle: ERROR {}",
                GetLastError()
            );
        }

        let mut restore_mode = 0;
        unsafe {
            assert_ne!(
                0,
                GetConsoleMode(input, &mut restore_mode),
                "Could not get console window mode: ERROR {}",
                GetLastError()
            );
        }

        let mut screen_buffer_info = CONSOLE_SCREEN_BUFFER_INFO::empty();
        unsafe {
            assert_ne!(
                0,
                GetConsoleScreenBufferInfo(output, &mut screen_buffer_info),
                "Could not get console screen buffer info: ERROR {}",
                GetLastError()
            );
        }

        let initial_size = Size::new(
            screen_buffer_info.srWindow.Right - screen_buffer_info.srWindow.Left + 1,
            screen_buffer_info.srWindow.Bottom - screen_buffer_info.srWindow.Top + 1,
        );

        // Skip enter
        let mut read = 0;
        unsafe {
            GetNumberOfConsoleInputEvents(input, &mut read);
        }

        unsafe {
            assert_ne!(
                0,
                SetConsoleCP(65001),
                "Could not set Code-Page: : ERROR {}",
                GetLastError()
            );
        }

        let mut console = Self {
            handle,
            output,
            input,
            mode: ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT,
            restore_mode,
            screen_buffer_info,
            initial_size,
        };
        console.set_cursor(Position::new(0, 0));
        console.cursor_visible(false);
        console.clear();

        console
    }

    pub fn resize(&self, size: Size) {
        let one_by_one = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 1,
            Bottom: 1,
        };
        let new_size = COORD {
            X: size.width,
            Y: size.height,
        };
        let info = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: size.width - 1,
            Bottom: size.height - 1,
        };

        unsafe {
            SetConsoleWindowInfo(self.output, 1, &one_by_one);
            SetConsoleScreenBufferSize(self.output, new_size);
            SetConsoleWindowInfo(self.output, 1, &info);
        }
    }

    pub fn clear(&self) {
        let coord_screen = COORD { X: 0, Y: 0 };
        let mut chars_written: DWORD = 0;
        let cell_count =
            self.screen_buffer_info.dwSize.X as usize * self.screen_buffer_info.dwSize.Y as usize;

        unsafe {
            FillConsoleOutputCharacterA(
                self.output,
                ' ' as i8,
                cell_count as u32,
                coord_screen,
                &mut chars_written,
            );
            FillConsoleOutputAttribute(
                self.output,
                self.screen_buffer_info.wAttributes,
                cell_count as u32,
                coord_screen,
                &mut chars_written,
            );
        }
    }

    pub fn set_cursor(&mut self, pos: Position) {
        unsafe {
            GetConsoleScreenBufferInfo(self.output, &mut self.screen_buffer_info);
        }

        let change = COORD {
            X: min(
                self.screen_buffer_info.srWindow.Right - self.screen_buffer_info.srWindow.Left + 1,
                max(0, pos.x),
            ),
            Y: max(0, pos.y),
        };

        stdout().flush().expect("Could not flush");

        unsafe {
            SetConsoleCursorPosition(self.output, change);
        }
    }

    pub fn write_cell(&self, pos: Position, cell: Cell) {
        let char_buf_size = COORD { X: 1, Y: 1 };
        let character_pos = COORD { X: 0, Y: 0 };
        let mut write_area = SMALL_RECT {
            Left: pos.x,
            Top: pos.y,
            Right: pos.x,
            Bottom: pos.y,
        };
        let mut info: CHAR_INFO_Char = unsafe { zeroed() };
        unsafe {
            *info.UnicodeChar_mut() = cell.ch as u16;
        }

        let character = CHAR_INFO {
            Char: info,
            Attributes: cell.get_color_attributes(),
        };

        unsafe {
            WriteConsoleOutputA(
                self.output,
                &character,
                char_buf_size,
                character_pos,
                &mut write_area,
            );
        }
    }

    pub fn set_title(&self, title: &str) {
        let cstr = CString::new(title).unwrap();
        unsafe {
            SetConsoleTitleA(cstr.as_ptr());
        }
    }

    pub fn cursor_visible(&self, visible: bool) {
        let mut cci = CONSOLE_CURSOR_INFO {
            dwSize: 0,
            bVisible: 0,
        };
        unsafe {
            GetConsoleCursorInfo(self.output, &mut cci);
        }
        cci.bVisible = visible as i32;
        unsafe {
            SetConsoleCursorInfo(self.output, &cci);
        }
    }

    pub fn reposition(&self, pos: Position) {
        let hwnd_insert_after: HWND = ptr::null_mut();
        unsafe {
            SetWindowPos(
                self.handle,
                hwnd_insert_after,
                pos.x as i32,
                pos.y as i32,
                0,
                0,
                SWP_NOZORDER | SWP_NOSIZE,
            );
        }
    }

    pub fn get_size(&mut self) -> Size {
        unsafe {
            GetConsoleScreenBufferInfo(self.output, &mut self.screen_buffer_info);
        }

        Size::new(
            self.screen_buffer_info.srWindow.Right - self.screen_buffer_info.srWindow.Left + 1,
            self.screen_buffer_info.srWindow.Bottom - self.screen_buffer_info.srWindow.Top + 1,
        )
    }
}
