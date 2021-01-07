use crate::{SystemBase, EntityManager, PlayerMovementComponent, Transform, Renderer, Rc, RefCell, InputManager, Camera};

pub struct PlayerMovementSystem{
}

impl SystemBase for PlayerMovementSystem{
    fn execute(&mut self, renderer: &Renderer, entity_manager: &mut EntityManager, input_manager: &InputManager, delta_time: f32, camera: &mut Camera){
        for entity_ref in entity_manager.get_entities_with_types_mut(&[PlayerMovementComponent::get_component_id(), Transform::get_component_id()]){
            

            let temp_entity = Rc::new(RefCell::new(entity_ref));
            let mut temp = temp_entity.borrow_mut();
            let trans_pos = match temp.get_component::<Transform>(Transform::get_component_id()){
                Ok(transform) => transform.position,
                Err(_) => panic!("Error - component not found!"),
            };
            let mut speed = 0.0;
            let mut move_vec = cgmath::Vector2::<f32> { x: 0.0, y: 0.0 };
            let mut movement_component = temp.get_component_mut::<PlayerMovementComponent>(PlayerMovementComponent::get_component_id()).unwrap();
            speed = movement_component.speed;

            movement_component.position = trans_pos;
            move_vec.x = match input_manager.try_get_key_value(winit::event::VirtualKeyCode::Left){
                Ok(v) => {
                    match v{
                        winit::event::ElementState::Pressed => 1.0,
                        winit::event::ElementState::Released => 0.0,
                    }
                },
                Err(_) => 0.0,
            };
            move_vec.x = match input_manager.try_get_key_value(winit::event::VirtualKeyCode::Right){
                Ok(v) => {
                    match v{
                        winit::event::ElementState::Pressed => -1.0,
                        winit::event::ElementState::Released => move_vec.x,
                    }
                },
                Err(_) => move_vec.x,
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
                Err(_) => move_vec.y,
            };


            drop(temp);
            
            let mut temp = temp_entity.borrow_mut();
            let transform = match temp.get_component_mut::<Transform>(Transform::get_component_id()){
                Ok(transform) => transform,
                Err(_) => panic!("Error - component not found!"),
            };


            
            transform.position += cgmath::Vector3::<f32> { x: move_vec.x * speed * delta_time, y: move_vec.y * speed * delta_time, z: 0.0};
            
            camera.move_camera(cgmath::Point3::<f32> { x: transform.position.x, y: transform.position.y, z: 10.0});

            transform.update_uniform_buffers(&renderer);
        }
    }
}

impl PlayerMovementSystem{
    pub fn new() -> Self{
        Self{
        }
    }
}