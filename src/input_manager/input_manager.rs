use winit::event::WindowEvent;
use winit::event::VirtualKeyCode;
use winit::event::ElementState;
use std::collections::HashMap;

pub struct InputManager{
    keys: HashMap::<VirtualKeyCode,ElementState>,
    mouse_pos: cgmath::Vector2::<f64>,
    mouse_button: winit::event::MouseButton
}

impl InputManager{
    pub fn new() -> Self{
        Self{
            keys: HashMap::<VirtualKeyCode,ElementState>::new(),
            mouse_pos: cgmath::Vector2::<f64> { x: 0.0, y: 0.0 },
            mouse_button: winit::event::MouseButton::Left
        }
    }

    pub fn update(&mut self, input_event: &WindowEvent){
        match input_event{
            WindowEvent::KeyboardInput {
                input: winit::event::KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => { 
                let val = self.keys.entry(*keycode).or_insert(*state);
                *val = *state;
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

    pub fn get_key_value(&self, key: VirtualKeyCode) -> ElementState{
        self.keys[&key]
    }

    pub fn try_get_key_value(&self, key: VirtualKeyCode) -> Result<ElementState, ()>{
        match self.keys.get_key_value(&key){
            Some(v) => Ok(*v.1),
            None => Err(())
        }
    }

    pub fn get_mouse_position(&self) -> cgmath::Vector2::<f64>{
        self.mouse_pos
    }

    pub fn get_mouse_button(&self) -> winit::event::MouseButton{
        self.mouse_button
    }
}