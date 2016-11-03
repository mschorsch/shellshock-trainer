
extern crate winapi;
extern crate user32;

use self::winapi::{POINT, HWND, RECT};

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::iter::once;

use super::{Handle, Rect, Cursor, VK};

const SHELLSHOCK_TITLE: &'static str = "ShellShock Live";

#[derive(Debug)]
pub struct WinHandle {
    hwnd: HWND,
}

impl WinHandle {
    fn new(hwnd: HWND) -> Self {
        WinHandle { hwnd: hwnd }
    }
}

impl Handle for WinHandle {
    #[allow(overflowing_literals)]
    fn is_key_pressed(&self, vk: VK) -> bool {

        // https://msdn.microsoft.com/de-de/library/windows/desktop/dd375731(v=vs.85).aspx
        let key_code = match vk {
            VK::Key1 => 0x31,
            VK::Key2 => 0x32,
            VK::Key3 => 0x33,
            VK::Key4 => 0x34,
        };

        let state = unsafe { user32::GetAsyncKeyState(key_code) };
        (state & 0x8000) != 0
    }

    fn get_window_rect(&self) -> Rect {
        let mut win_rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };

        unsafe { user32::GetClientRect(self.hwnd, &mut win_rect) };

        let width = win_rect.right - win_rect.left;
        let height = win_rect.bottom - win_rect.top;

        Rect::new(width, height)
    }

    fn get_mouse_position_in_window(&self) -> Cursor {
        let mut pt = POINT { x: 0, y: 0 };
        unsafe {
            user32::GetCursorPos(&mut pt);
            user32::ScreenToClient(self.hwnd, &mut pt);
        }
        Cursor::new(pt.x, pt.y)
    }
}

pub fn find_shellshock_handle() -> WinHandle {
    use std::thread;
    use std::time;

    loop {
        thread::sleep(time::Duration::from_millis(100));

        if let Some(handle) = get_handle_by_title(SHELLSHOCK_TITLE) {
            return handle;
        }
    }
}

fn get_handle_by_title(title: &str) -> Option<WinHandle> {
    let wide: Vec<u16> = OsStr::new(title).encode_wide().chain(once(0)).collect();
    let hwnd = unsafe { user32::FindWindowW(ptr::null(), wide.as_ptr()) };
    if hwnd == ptr::null_mut() {
        return Option::None;
    }
    Option::Some(WinHandle::new(hwnd))
}