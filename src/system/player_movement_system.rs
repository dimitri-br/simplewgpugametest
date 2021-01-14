use crate::{SystemBase, EntityManager, PlayerMovementComponent, Transform, PhysicsComponent, Renderer, Rc, RefCell, InputManager, Camera, Physics, b2};
use wrapped2d::collision::test_overlap;

pub struct PlayerMovementSystem{
}

impl SystemBase for PlayerMovementSystem{
    fn execute(&mut self, renderer: &Renderer, entity_manager: &mut EntityManager, input_manager: &InputManager, physics: &mut Physics, delta_time: f32, camera: &mut Camera){
        for entity_ref in entity_manager.get_entities_with_types_mut(&[PlayerMovementComponent::get_component_id(), Transform::get_component_id(), PhysicsComponent::get_component_id()]){
            

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
                        winit::event::ElementState::Pressed => -1.0,
                        winit::event::ElementState::Released => 0.0,
                    }
                },
                Err(_) => 0.0,
            };
            move_vec.x = match input_manager.try_get_key_value(winit::event::VirtualKeyCode::A){
                Ok(v) => {
                    match v{
                        winit::event::ElementState::Pressed => -1.0,
                        winit::event::ElementState::Released => move_vec.x,
                    }
                },
                Err(_) => move_vec.x,
            };
            move_vec.x = match input_manager.try_get_key_value(winit::event::VirtualKeyCode::Right){
                Ok(v) => {
                    match v{
                        winit::event::ElementState::Pressed => 1.0,
                        winit::event::ElementState::Released => move_vec.x,
                    }
                },
                Err(_) => move_vec.x,
            };
            move_vec.x = match input_manager.try_get_key_value(winit::event::VirtualKeyCode::D){
                Ok(v) => {
                    match v{
                        winit::event::ElementState::Pressed => 1.0,
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
            move_vec.y = match input_manager.try_get_key_value(winit::event::VirtualKeyCode::W){
                Ok(v) => {
                    match v{
                    winit::event::ElementState::Pressed => 1.0,
                    winit::event::ElementState::Released => move_vec.y 
                    }
                },
                Err(_) => move_vec.y,
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
            move_vec.y = match input_manager.try_get_key_value(winit::event::VirtualKeyCode::S){
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


            let mut reset_pos = false;
            if transform.position.y < -15.0{
                reset_pos = true;
            }
            if transform.position.y < -5.0{
                move_vec.y = 0.0;
            }
            //transform.position += cgmath::Vector3::<f32> { x: move_vec.x * speed * delta_time, y: move_vec.y * speed * delta_time, z: 0.0};
            
            //camera.move_camera(cgmath::Point3::<f32> { x: transform.position.x, y: transform.position.y, z: 10.0});

            transform.update_uniform_buffers(&renderer);

            drop(temp);

            let mut temp = temp_entity.borrow_mut();
            let phys_ref = match temp.get_component_mut::<PhysicsComponent>(PhysicsComponent::get_component_id()){
                Ok(transform) => transform,
                Err(_) => panic!("Error - component not found!"),
            };
            let mut points: i32 = 0;
            if reset_pos{
                phys_ref.update_position(physics, b2::Vec2 { x: 0.0, y: 0.0 });
                points -= 5;
            }

            let body = physics.world.body(phys_ref.handle);

            let lin_vel = body.linear_velocity();
            let vel_y = lin_vel.y;
            let vel_x = lin_vel.x;
            let center = *body.world_center();
            drop(body);
            let mut body = physics.world.body_mut(phys_ref.handle);
            body.apply_linear_impulse(&b2::Vec2{ x: ((move_vec.x * speed)) - (if move_vec.x == 0.0 { vel_x } else {vel_x}), y: (move_vec.y * speed) - (if move_vec.y == 0.0 { 1.0 } else {vel_y}) }, &center, true);

            drop(body);
            drop(temp);
            //phys_ref.set_velocity(physics, b2::Vec2{ x: (move_vec.x * speed * delta_time) + x_force, y: (move_vec.y * speed * delta_time) + gravity });

            let mut temp = temp_entity.borrow_mut();
            let mut movement_component = temp.get_component_mut::<PlayerMovementComponent>(PlayerMovementComponent::get_component_id()).unwrap();

            movement_component.points += points;
        }
    }
}

impl PlayerMovementSystem{
    pub fn new() -> Self{
        Self{
        }
    }
}