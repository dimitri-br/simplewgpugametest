use crate::*;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub struct SceneLoader{

}

impl SceneLoader{
    pub fn load(path: &str, entity_manager: &mut EntityManager, physics_manager: &mut Physics, renderer_reference: &Renderer, camera_bind_group: Rc<wgpu::BindGroup>){
        let entity_defs = SceneLoader::load_component(path);
        for entity_def in entity_defs{
            SceneLoader::parse_entity(entity_def, entity_manager, renderer_reference, Rc::clone(&camera_bind_group));
        }
    }

    fn load_component(path: &str) -> Vec<String>{
        // Load all non blank and non comment lines (That include entity)
        let file = File::open(path).unwrap();
        let buf_reader = BufReader::new(file);
        let mut lines = Vec::<String>::new();
        for line in buf_reader.lines(){
            if let Ok(ip) = line {
                if ip != "" && ip.contains("entity") && !ip.contains("//"){
                    lines.push(ip);
                }
            }
        }
        lines
    }

    
    fn parse_entity(def: String, entity_manager: &mut EntityManager, renderer_reference: &Renderer, camera_bind_group: Rc<wgpu::BindGroup>){
        let mut uniforms = vec!(camera_bind_group);
        let mut entity_components = Vec::<Box<dyn ComponentBase>>::new();


        let _components: Vec::<&str> = def.split("[").collect();
        let components: String = _components[1].to_string();
        let _components: Vec::<&str> = components.split("]").collect();
        let components: String = _components[0].to_string();
        let components: Vec::<String> = components.split(" ").map(|x| x.to_string()).collect();
        println!("{:?}", components);


        let mut position: Translation = Translation::new(cgmath::Vector3::<f32> { x: 0.0, y: 0.0, z: 0.0});

        let mut rotation: Rotation = Rotation::new(cgmath::Quaternion::from(cgmath::Euler {
            x: cgmath::Deg(0.0),
            y: cgmath::Deg(0.0),
            z: cgmath::Deg(0.0),
        }));

        let mut scale: NonUniformScale = NonUniformScale::new(cgmath::Vector3::<f32> { x: 1.0, y: 1.0, z: 1.0});


        for component in components{
            let split_comp: Vec::<String> = component.split("(").map(|x| x.to_string()).collect();
            let comp = split_comp[0].clone();

            match comp.as_str(){
                "name" => {
                    let name: Vec<String> = split_comp[1].split(")").map(|x| x.to_string()).collect();
                    let name = name[0].clone(); 
                    println!("Adding component: {:?}", name);
                },

                "pos" =>  {
                    let pos: Vec<String> = split_comp[1].split(")").map(|x| x.to_string()).collect();
                    let pos = pos[0].clone(); 
                    let xyz: Vec<f32> = pos.split(",").map(|x| x.parse::<f32>().unwrap()).collect();
                    let x = xyz[0];
                    let y = xyz[1];
                    let z = xyz[2];
                    position = Translation::new(cgmath::Vector3::<f32> {x, y, z});
                },

                "rot" =>  {
                    let rot: Vec<String> = split_comp[1].split(")").map(|x| x.to_string()).collect();
                    let rot = rot[0].clone(); 
                    let xyz: Vec<f32> = rot.split(",").map(|x| x.parse::<f32>().unwrap()).collect();

                    let x = xyz[0];
                    let y = xyz[1];
                    let z = xyz[2];

                    rotation = Rotation::new(cgmath::Quaternion::from(cgmath::Euler {
                        x: cgmath::Deg(x),
                        y: cgmath::Deg(y),
                        z: cgmath::Deg(z),
                    }));
                },

                "scale" =>  {
                    let scl: Vec<String> = split_comp[1].split(")").map(|x| x.to_string()).collect();
                    let scl = scl[0].clone(); 
                    let xyz: Vec<f32> = scl.split(",").map(|x| x.parse::<f32>().unwrap()).collect();

                    let x = xyz[0];
                    let y = xyz[1];
                    let z = xyz[2];

                    scale = NonUniformScale::new(cgmath::Vector3::<f32> { x, y, z});
                },

                "material" =>  {
                    let tex_path_raw: Vec<String> = split_comp[1].split(",").map(|x| x.to_string()).collect();
                    let color_raw: Vec<String> = split_comp[2].split(")").map(|x| x.to_string()).collect();

                    
                    let tex_path: String = tex_path_raw[0].clone();
                    println!("{:?}", tex_path);

                    let color: Vec<f32> = color_raw[0].split(",").map(|x| x.parse::<f32>().unwrap()).collect();

                    let x = color[0];
                    let y = color[1];
                    let z = color[2];

                    let mut material = Material::new(&renderer_reference, Rc::new(Texture::load_texture(&renderer_reference, &tex_path, TextureMode::RGB).unwrap()), cgmath::Vector3::<f32> { x, y, z }, 1.0, 0.0, -1, "main".to_string());

                    println!("{:?} -> {:?}", tex_path, color);

                    let mut mesh = RenderMesh::new(&renderer_reference, material);
                    let (bindgroup, layout, _) = mesh.generate_material_uniforms(&renderer_reference);

                    entity_components.push(Box::new(mesh));
                    uniforms.push(Rc::new(bindgroup));         

                    
                },

                "physics" =>  {},
                _ => panic!("Not valid!"),
            }
        }
        println!("Pos: {:?}\nScale: {:?}\nRot: {:?}", position.value, scale.value, rotation.value);
        let mut transform = Transform::new(renderer_reference, position.value, rotation.value, scale.value);
        let (trans_bind_group, layout, _) = transform.create_uniforms(&renderer_reference);
        uniforms.push(Rc::new(trans_bind_group));
        entity_components.push(Box::new(transform));

        {

            entity_manager.create_entity(entity_components, uniforms);
        }
    }
}