
#[cfg(windows)]
pub mod windows;

pub trait Handle {
    fn is_key_pressed(&self, vk: VK) -> bool;

    fn get_window_rect(&self) -> Rect;

    // assuming (0,0) => upperleft
    fn get_mouse_position_in_window(&self) -> Cursor;
}

#[derive(Debug)]
pub enum VK {
    Key1, // set source
    Key2, // set target
    Key3, // show results
    Key4, // clear
    Key5, // switch calculation mode
}

#[derive(Debug)]
pub struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    pub fn new(width: i32, height: i32) -> Self {
        Rect {
            width: width,
            height: height,
        }
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}

#[derive(Debug)]
pub struct Cursor {
    x: i32,
    y: i32,
}

impl Cursor {
    pub fn new(x: i32, y: i32) -> Self {
        Cursor { x: x, y: y }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}