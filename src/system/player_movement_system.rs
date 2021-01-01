use crate::{SystemBase, EntityManager, PlayerMovementComponent, Transform, Renderer, Rc, RefCell, InputManager};

pub struct PlayerMovementSystem{
    x: f32,
}

impl SystemBase for PlayerMovementSystem{
    fn execute(&mut self, renderer: &Renderer, entity_manager: &mut EntityManager, input_manager: &InputManager){
        for entity_ref in entity_manager.get_entities_with_types_mut(&[PlayerMovementComponent::get_component_id(), Transform::get_component_id()]){
            

            let temp_entity = Rc::new(RefCell::new(entity_ref));
            let temp = temp_entity.borrow();
            let mut speed = 0.0;
            let mut move_vec = cgmath::Vector2::<f32> { x: 0.0, y: 0.0 };
            let movement_component = temp.get_component::<PlayerMovementComponent>(PlayerMovementComponent::get_component_id()).unwrap();
            speed = movement_component.speed;

            move_vec.x = match input_manager.try_get_key_value(winit::event::VirtualKeyCode::Left){
                Ok(v) => {
                    match v{
                        winit::event::ElementState::Pressed => -1.0,
                        winit::event::ElementState::Released => 0.0,
                    }
                },
                Err(_) => 0.0,
            };
            move_vec.x = match input_manager.try_get_key_value(winit::event::VirtualKeyCode::Right){
                Ok(v) => {
                    match v{
                        winit::event::ElementState::Pressed => 1.0,
                        winit::event::ElementState::Released => move_vec.x,
                    }
                },
                Err(_) => 0.0,
            };

            move_vec.y = match input_manager.try_get_key_value(winit::event::VirtualKeyCode::Up){
                Ok(v) => {
                    match v{
                    winit::event::ElementState::Pressed => 1.0,
                    winit::event::ElementState::Released => 0.0
                    }
                },
                Err(_) => 0.0,
            };
            move_vec.y = match input_manager.try_get_key_value(winit::event::VirtualKeyCode::Down){
                Ok(v) => {
                    match v{
                        winit::event::ElementState::Pressed => -1.0,
                        winit::event::ElementState::Released => move_vec.y
                    }
                },
                Err(_) => 0.0,
            };


            drop(temp);
            
            let mut temp = temp_entity.borrow_mut();
            let transform = match temp.get_component_mut::<Transform>(Transform::get_component_id()){
                Ok(transform) => transform,
                Err(_) => panic!("Error - component not found!"),
            };


            
            transform.position += cgmath::Vector3::<f32> { x: move_vec.x * speed, y: move_vec.y * speed, z: 0.0};
            transform.rotation = cgmath::Quaternion::from(cgmath::Euler {
                x: cgmath::Deg(0.0),
                y: cgmath::Deg(0.0),
                z: cgmath::Deg(self.x),
            });
            self.x += 0.6;

            transform.update_uniform_buffers(&renderer);
        }
    }
}

impl PlayerMovementSystem{
    pub fn new() -> Self{
        Self{
            x: 0.0,
        }
    }
}