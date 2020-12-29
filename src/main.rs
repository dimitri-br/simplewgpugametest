mod renderer;
mod input_manager;

use renderer::renderer::Renderer;
use renderer::vertex::Vertex;
use renderer::texture::Texture;
use renderer::material::Material;
use input_manager::input_manager::InputManager;
use renderer::entity::rendermesh::RenderMesh;


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
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut renderer = block_on(Renderer::new(&window));
    let mut input_manager = InputManager::new();
    let mut renderer = Rc::new(RefCell::new(renderer));
    let render_mesh = RenderMesh::new(Rc::clone(&renderer));

    let mut meshes = Vec::<RenderMesh>::new();

    meshes.push(render_mesh);

    let diffuse_bytes = include_bytes!("./happy-tree.png"); // CHANGED!
    let diffuse_texture = Texture::from_bytes(renderer, diffuse_bytes, "happy-tree.png").unwrap();

    event_loop.run(move |event, _, control_flow|  
        match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() =>  {
            input_manager.update(event);
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
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &&mut so we have to dereference it twice
                renderer.resize(**new_inner_size);
            },
            
            _ => {}
        }
        },
        Event::RedrawRequested(_) => {
            let window_size = renderer.borrow().get_window_size();
            let mut renderer = renderer.borrow_mut();
            renderer.update();

            let mouse_pos = input_manager.get_mouse_position();
            let clear_color = wgpu::Color {
                r: mouse_pos.x / renderer.get_window_size().width as f64,
                g: mouse_pos.y / renderer.get_window_size().height as f64,
                b: 1.0,
                a: 1.0,
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