//#![windows_subsystem = "windows"] // Disable console

mod renderer;
mod component;
mod transform;
mod input_manager;


use renderer::renderer::Renderer;
use renderer::vertex::Vertex;
use renderer::texture::Texture;
use renderer::material::{Material, MaterialUniform};
use input_manager::input_manager::InputManager;
use renderer::entity::rendermesh::RenderMesh;
use renderer::entity::entity::Entity;
use renderer::camera::camera::{Camera, CameraUniform};
use renderer::camera::cameracontroller::CameraController;
use renderer::uniforms::{UniformUtils, UniformBuffer};
use component::ComponentBase;
use transform::{Translation, Rotation, NonUniformScale, Transform, TransformUniform};



use std::rc::Rc;
use std::cell::RefCell;

use futures::executor::block_on;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};



fn main() {
    if cfg!(debug_assertions) {
        println!("RUNNING: Debug");
    }else{
        println!("RUNNING: Release");
    }

    let event_loop = EventLoop::new();
    
    let window = WindowBuilder::new()
    .with_inner_size(winit::dpi::Size::from(winit::dpi::LogicalSize{ width: 1280, height: 720}))
    .with_title("WGPU TEST APP")        
    .with_decorations(false)
    .with_maximized(true)
    .with_transparent(false)
    .build(&event_loop)
    .unwrap();
    println!("~~~Starting~~~");
    let renderer = block_on(Renderer::new(&window));
    let mut input_manager = InputManager::new();
    let renderer = Rc::new(RefCell::new(renderer));

    /* User Defined */
    let mut uniform_utils = UniformUtils::new();


    let mut entities = Vec::<Entity>::new();

    println!("~~~Pre-Setup finished~~~");

    // Since we share the renderer around, borrow it mutably
    let mut temp_renderer = renderer.borrow_mut();

    // Camera
    let mut camera_controller = CameraController::new(0.1);

    let sc_desc = &temp_renderer.sc_desc;

    let mut camera = Camera::new(
        &temp_renderer,
        // position the camera one unit up and 2 units back
        // +z is out of the screen
        (0.0, 0.0, 10.0).into(),
        // have it look at the origin
        (0.0, 0.0, 0.0).into(),
        // which way is "up"
        cgmath::Vector3::unit_y(),
        sc_desc.width as f32 / sc_desc.height as f32,
        45.0,
        0.1,
        100.0,
    );

    let (camera_bind_group, camera_layout, mut cam_uniform) = camera.create_uniforms(&temp_renderer);
    let camera_bind_group = Rc::new(camera_bind_group);
    cam_uniform.update_view_proj(&camera);

    // load texture
    let diffuse_texture = Rc::new(Texture::load_texture(&temp_renderer, "./data/textures/smiley.png").unwrap());
    let diffuse_texture_layout = Texture::generate_texture_layout(&temp_renderer);

    let diffuse_texture2 = Rc::new(Texture::load_texture(&temp_renderer, "./data/textures/happy-tree.png").unwrap());
    
    // Create transform layout
    let transform_layout = UniformUtils::create_bind_group_layout(&temp_renderer, 0, wgpu::ShaderStage::VERTEX, Some("Transform"));


    // define the array with layouts we want to use in our pipeline
    let mut layouts = vec!(&diffuse_texture_layout, &camera_layout);
    // create material
    let material_layout = Material::create_uniform_layout(&temp_renderer);
    layouts.push(&material_layout);
    layouts.push(&transform_layout);


    let mut uniforms = Vec::<Rc<wgpu::BindGroup>>::new();
    let mut components = Vec::<Box<dyn ComponentBase>>::new();

    // create material
    let material = Material::new(&temp_renderer, Rc::clone(&diffuse_texture), 1.0, 0.0);

    // create new mesh (TODO - mesh loading) and assign material
    let mut mesh = RenderMesh::new(&temp_renderer, material);
    let (material_group, _, _) = mesh.generate_material_uniforms(&temp_renderer);
    let material_group = Rc::new(material_group);

    let translation = Translation::new(cgmath::Vector3::<f32> { x: 2.0, y: 1.0, z: 0.0});

    let rotation = Rotation::new(cgmath::Quaternion::from(cgmath::Euler {
        x: cgmath::Deg(0.0),
        y: cgmath::Deg(0.0),
        z: cgmath::Deg(0.0),
    }));

    let scale = NonUniformScale::new(cgmath::Vector3::<f32> { x: 1.0, y: 1.0, z: 1.0});

    let mut transform = Transform::new(&temp_renderer, translation.value, rotation.value, scale.value);
    let (transform_group, _, transform_uniform) = transform.create_uniforms(&temp_renderer);
    let transform_group = Rc::new(transform_group);

    uniforms.push(Rc::clone(&camera_bind_group));
    uniforms.push(Rc::clone(&material_group));
    uniforms.push(Rc::clone(&transform_group));

    components.push(Box::new(mesh));
    components.push(Box::new(transform));

    entities.push(create_entity(&temp_renderer, &mut uniform_utils, components, uniforms));


    let mut uniforms = Vec::<Rc<wgpu::BindGroup>>::new();
    let mut components = Vec::<Box<dyn ComponentBase>>::new();

    // create material
    let material = Material::new(&temp_renderer, Rc::clone(&diffuse_texture2), 1.0, 0.0);

    // create new mesh (TODO - mesh loading) and assign material
    let mut mesh = RenderMesh::new(&temp_renderer, material);
    let (material_group, _, _) = mesh.generate_material_uniforms(&temp_renderer);
    let material_group = Rc::new(material_group);

    let translation = Translation::new(cgmath::Vector3::<f32> { x: -2.0, y: 1.0, z: 0.0});

    let rotation = Rotation::new(cgmath::Quaternion::from(cgmath::Euler {
        x: cgmath::Deg(0.0),
        y: cgmath::Deg(0.0),
        z: cgmath::Deg(0.0),
    }));

    let scale = NonUniformScale::new(cgmath::Vector3::<f32> { x: 1.0, y: 1.0, z: 1.0});


    let mut transform = Transform::new(&temp_renderer, translation.value, rotation.value, scale.value);
    let (transform_group, _, mut transform_uniform) = transform.create_uniforms(&temp_renderer);
    let transform_group = Rc::new(transform_group);

    uniforms.push(Rc::clone(&camera_bind_group));
    uniforms.push(Rc::clone(&material_group));
    uniforms.push(Rc::clone(&transform_group));

    components.push(Box::new(mesh));
    components.push(Box::new(transform));

    entities.push(create_entity(&temp_renderer, &mut uniform_utils, components, uniforms));

    // recreate pipeline with layouts (needs mut)
    temp_renderer.recreate_pipeline(&layouts);
    // drop the borrowed mut reference (to stay safe)
    drop(temp_renderer);

    /* Game Loop Defined */
    println!("~~~Setup finished~~~");
    let mut x = 0.0;
    event_loop.run(move |event, _, control_flow|  
        match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() =>  {
            camera_controller.process_events(event);
            let mut renderer = renderer.borrow_mut();
            match event{
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput {
                input,
                ..
            } => {
                match input {
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            },
            WindowEvent::Resized(physical_size) => {
                renderer.resize(*physical_size);
                let sc_desc = &renderer.sc_desc;
                camera.aspect = sc_desc.width as f32 / sc_desc.height as f32;
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &&mut so we have to dereference it twice
                renderer.resize(**new_inner_size);
                let sc_desc = &renderer.sc_desc;
                camera.aspect = sc_desc.width as f32 / sc_desc.height as f32;

            },
            
            _ => {}
        }
        },
        Event::RedrawRequested(_) => {


            let window_size = renderer.borrow().get_window_size();
            let mut renderer = renderer.borrow_mut();
            camera_controller.update_camera(&mut camera);
            cam_uniform.update_view_proj(&camera);
            renderer.write_buffer(camera.get_buffer_reference(), 0, &[cam_uniform]);

            let transform = entities[0].get_component_mut::<Transform>(Transform::get_component_id()).unwrap();
            transform.position += cgmath::Vector3::<f32> { x: 0.001, y: 0.001, z: 0.0};
            transform.rotation = cgmath::Quaternion::from(cgmath::Euler {
                x: cgmath::Deg(0.0),
                y: cgmath::Deg(0.0),
                z: cgmath::Deg(x),
            });
            x += 0.6;
            transform_uniform.update(transform.generate_matrix());

            renderer.write_buffer(transform.get_buffer_reference(), 0, &[transform_uniform]);
            renderer.update();

            let mouse_pos = input_manager.get_mouse_position();
            
            let clear_color = wgpu::Color {
                r: 0.5,
                g: 0.3,
                b: 0.6,
                a: 0.5,
            };

            match renderer.render(clear_color, &entities) {
                Ok(_) => {}
                // Recreate the swap_chain if lost
                Err(wgpu::SwapChainError::Lost) => renderer.resize(window_size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
            
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
            
        }
        _ => {}
    }
    
);
}

fn create_entity(temp_renderer: &Renderer, uniform_utils: &mut UniformUtils, mut components: Vec::<Box<dyn ComponentBase>>, mut uniforms: Vec::<Rc<wgpu::BindGroup>>) -> Entity{
    let mut entity = Entity::new(components);
    entity.set_uniforms(uniforms);
    entity
}
