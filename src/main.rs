#![windows_subsystem = "windows"] // Disable console

mod renderer;
mod input_manager;


use renderer::renderer::Renderer;
use renderer::vertex::Vertex;
use renderer::texture::Texture;
use renderer::material::Material;
use input_manager::input_manager::InputManager;
use renderer::entity::rendermesh::RenderMesh;
use renderer::camera::camera::Camera;
use renderer::camera::cameracontroller::CameraController;
use renderer::uniforms::camera_uniform::CameraUniform;
use renderer::uniforms::{UniformUtils, UniformBuffer};


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
    .with_decorations(true)
    .with_transparent(false)
    .build(&event_loop)
    .unwrap();

    let renderer = block_on(Renderer::new(&window));
    let mut input_manager = InputManager::new();
    let renderer = Rc::new(RefCell::new(renderer));
    let mut uniform_utils = UniformUtils::new();


    let mut meshes = Vec::<RenderMesh>::new();
    // Camera
    let mut camera_controller = CameraController::new(0.1);
    let temp = Rc::clone(&renderer);
    let temp_borrow = temp.borrow();
    let sc_desc = &temp_borrow.sc_desc;
    let mut camera = Camera {
        // position the camera one unit up and 2 units back
        // +z is out of the screen
        eye: (0.0, 0.0, 10.0).into(),
        // have it look at the origin
        target: (0.0, 0.0, 0.0).into(),
        // which way is "up"
        up: cgmath::Vector3::unit_y(),
        aspect: sc_desc.width as f32 / sc_desc.height as f32,
        fovy: 45.0,
        znear: 0.1,
        zfar: 100.0,
    };
    let mut cam_uniform = CameraUniform::new();
    let cam_buffer = cam_uniform.create_uniform_buffer(&temp_borrow);
    drop(temp_borrow); // Drop the current borrow so we can borrow it mutably to write a new render pipeline with all our new uniforms

    // Uniform utils has handy things fpr creating shader uniforms and storing them
    uniform_utils.add(cam_uniform, vec!(cam_buffer));

    cam_uniform.update_view_proj(&camera);
    
    // Since we share the renderer around, borrow it mutably
    let mut temp_renderer = renderer.borrow_mut();

    
    // Create our camera layout and bind group
    let camera_layout = UniformUtils::create_bind_group_layout(&temp_renderer, 0, wgpu::ShaderStage::VERTEX);
    let camera_uniform = Rc::new(UniformUtils::create_bind_group(&temp_renderer, &uniform_utils.get_buffer_by_index(0)[0], &camera_layout, 0));

    // load texture
    let diffuse_texture = Texture::load_texture(&temp_renderer, "./data/textures/smiley.png").unwrap();
    

    // define the array with layouts we want to use in our pipeline
    let layouts = &[diffuse_texture.get_texture_layout(), &camera_layout];
    // recreate pipeline with layouts (needs mut)
    temp_renderer.recreate_pipeline(layouts);
    // create material
    let material = Material::new(diffuse_texture, 0.5, 0.0);
    // create new mesh (TODO - mesh loading) and assign material
    let render_mesh = RenderMesh::new(&temp_renderer, material);
    // add to global mesh vec
    meshes.push(render_mesh);
    // add a camera uniform pointer to each mesh
    for mesh in meshes.iter_mut(){
        mesh.add_new_uniform(Rc::clone(&camera_uniform));
    }
    // drop the borrowed mut reference (to stay safe)
    drop(temp_renderer);



    event_loop.run(move |event, _, control_flow|  
        match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() =>  {
            input_manager.update(event);
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
            renderer.write_buffer(&uniform_utils.get_buffer_by_index(0)[0], 0, &[cam_uniform]);
            renderer.update();

            let mouse_pos = input_manager.get_mouse_position();
            
            let clear_color = wgpu::Color {
                r: mouse_pos.x / renderer.get_window_size().width as f64,
                g: mouse_pos.y / renderer.get_window_size().height as f64,
                b: 1.0,
                a: 0.5,
            };

            match renderer.render(clear_color, &meshes) {
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
