use crate::{ComponentBase, Rc, RefCell};
use std::ops::DerefMut;

fn downcast<T: ComponentBase + 'static>(this: &dyn ComponentBase) -> Option<&T> {
    this.as_any().downcast_ref()
}

fn downcast_mut<T: ComponentBase + 'static>(this: &mut dyn ComponentBase) -> Option<&mut T> {
    this.as_any_mut().downcast_mut()
}

pub struct Entity{
    pub components: Vec<Box<dyn ComponentBase>>,
    pub uniforms: Vec::<Rc::<wgpu::BindGroup>>,
    pub id: usize,
}

impl Entity{
    pub fn new(components: Vec<Box<dyn ComponentBase>>, id: usize) -> Self{
        let uniforms = Vec::<Rc::<wgpu::BindGroup>>::new();

        Self{
            components,
            uniforms,
            id
        }
    }

    pub fn add_component(&mut self, component: Box<dyn ComponentBase>){
        self.components.push(component);
    }

    pub fn try_remove_component(&mut self, component_id: u32) -> Result<(), &str>{
        let index = self.components.iter().position(|x| x.get_id() == component_id);// Compare ID to find the component
        let index = match index{
            Some(v) => v,
            None => {return Err("Component not in entity");} 
        };
        self.components.remove(index);
        Ok(())
    }

    pub fn try_find_component(&self, component_id: u32) -> Result<&Box<dyn ComponentBase>, &str>{
        let index = self.components.iter().position(|x| x.get_id() == component_id);// Compare ID to find the component
        let index = match index{
            Some(v) => v,
            None => {return Err("Component not in entity");} 
        };
        Ok(&self.components[index])
    }

    pub fn try_find_component_mut(&mut self, component_id: u32) -> Result<&Box<dyn ComponentBase>, &str>{
        let index = self.components.iter().position(|x| x.get_id() == component_id);// Compare ID to find the component
        let index = match index{
            Some(v) => v,
            None => {return Err("Component not in entity");} 
        };
        Ok(&self.components[index])
    }


    pub fn get_index(&self, component_id: u32) -> Result<usize, &str>{
        let index = self.components.iter().position(|x| x.get_id() == component_id);// Compare ID to find the component
        match index{
            Some(v) =>  Ok(v),
            None => Err("Component not in entity")
        }
    }

    pub fn get_component<T: ComponentBase + 'static>(&self, component_id: u32) -> Result<&T, &str>{
        let component = self.try_find_component(component_id)?;
        // Try and convert the rendermesh component we have (Which is currently a trait object) into a rendermesh struct so we can use it
        if let Some(r) = downcast::<T>(&**component) {
            return Ok(r);
        }
        Err("Error downcasting component result")
    }

    pub fn get_component_mut<T: ComponentBase + 'static>(&mut self, component_id: u32) -> Result<&mut T, &str>{
        let index = match self.get_index(component_id){
            Ok(v) => v,
            Err(e) => return Err("Component Not In Entity"),
        };

        let mut component = self.components[index].deref_mut();
        
        // Try and convert the rendermesh component we have (Which is currently a trait object) into a rendermesh struct so we can use it
        if let Some(r) = downcast_mut::<T>(&mut *component) {
            return Ok(r);
        }
        Err("Error downcasting component result")
    }

    pub fn add_new_uniform(&mut self, uniform: Rc<wgpu::BindGroup>){
        self.uniforms.push(uniform);
    }

    pub fn set_uniforms(&mut self, uniforms: Vec::<Rc<wgpu::BindGroup>>){
        self.uniforms = uniforms;
    }

    pub fn get_uniforms(&self) -> &Vec::<Rc<wgpu::BindGroup>>{
        &self.uniforms
    }

}