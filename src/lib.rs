extern "system" {
    pub fn SendInput(cInputs: u32, pInputs: *mut Input, cbSize: i32) -> u32;
    pub fn GetSystemMetrics(nIndex: i32) -> i32;
}

pub struct MouseInput {
    pub dx: i32,
    pub dy: i32,
    pub mouse_data: u32,
    pub flags: u32,
    pub time: u32,
    pub extra_info: u64,
}

pub struct Input {
    pub input_type: u32,
    pub input_data: MouseInput,
}

pub fn send_input(dx: i32, dy: i32, mouse_data: u32, flags: u32) {
    let mut input = Input{
        input_type: 0, 
        input_data: MouseInput {
            dx: dx, 
            dy: dy, 
            mouse_data: mouse_data, 
            flags: flags, 
            time: 0, 
            extra_info: 0
        }
    };
    unsafe {SendInput(1, &mut input, std::mem::size_of::<Input>() as i32);}
}

pub fn move_mouse(mut x: i32, mut y: i32, abs: bool) {
    let mut flags = 0x0001;
    if abs == true {
        x = x*65335/unsafe{GetSystemMetrics(78)};
        y = y*65335/unsafe{GetSystemMetrics(79)};
        flags = flags | 0x8000;
    }
    send_input(x, y, 0, flags);
}

pub struct MouseButton {
    pub e_down: u32,
    pub e_up: u32,
}

impl MouseButton {
    pub fn down(&self) {send_input(0, 0, 0, self.e_down)}
    pub fn up(&self) {send_input(0, 0, 0, self.e_up)}
}

pub const LEFT: MouseButton = MouseButton{e_down: 0x0002, e_up: 0x0004};
pub const RIGHT: MouseButton = MouseButton{e_down: 0x0008, e_up: 0x0010};
pub const MIDDLE: MouseButton = MouseButton{e_down: 0x0020, e_up: 0x0040};

pub fn wheel(movement: u32) {send_input(0, 0, movement, 0x0800)}