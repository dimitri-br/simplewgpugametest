use winit::event::WindowEvent;

pub struct InputManager{
    keycode: u32,
    key: winit::event::VirtualKeyCode,
    mouse_pos: cgmath::Vector2::<f64>,
    mouse_button: winit::event::MouseButton
}

impl InputManager{
    pub fn new() -> Self{
        Self{
            keycode: 0,
            key: winit::event::VirtualKeyCode::End,
            mouse_pos: cgmath::Vector2::<f64> { x: 0.0, y: 0.0 },
            mouse_button: winit::event::MouseButton::Left
        }
    }

    pub fn update(&mut self, input_event: &WindowEvent){
        match input_event{
            WindowEvent::KeyboardInput {input, ..} => { 
                self.keycode = input.scancode;
                self.key = input.virtual_keycode.unwrap();
            },
            WindowEvent::MouseInput {button, ..} => {
                self.mouse_button = *button;
            },
            WindowEvent::CursorMoved{position, ..} => {
                self.mouse_pos.x = position.x as f64;
                self.mouse_pos.y = position.y as f64;
            }
            _ => {}
        }
    }

    pub fn get_key(&self) -> winit::event::VirtualKeyCode{
        self.key
    }

    pub fn get_keycode(&self) -> u32{
        self.keycode
    }

    pub fn get_mouse_position(&self) -> cgmath::Vector2::<f64>{
        self.mouse_pos
    }

    pub fn get_mouse_button(&self) -> winit::event::MouseButton{
        self.mouse_button
    }
}