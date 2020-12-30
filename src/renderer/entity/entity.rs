use crate::ComponentBase;

fn downcast<T: ComponentBase + 'static>(this: &ComponentBase) -> Option<&T> {
    this.as_any().downcast_ref()
}

pub struct Entity{
    components: Vec<Box<dyn ComponentBase>>
}

impl Entity{
    pub fn new(components: Vec<Box<dyn ComponentBase>>) -> Self{
        Self{
            components
        }
    }

    pub fn add_component(&mut self, component: Box<dyn ComponentBase>){
        self.components.push(component);
    }

    pub fn try_remove_component(&mut self, component: Box<dyn ComponentBase>) -> Result<(), &str>{
        let index = self.components.iter().position(|x| x.get_id() == component.get_id());// Compare ID to find the component
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

    pub fn get_component<T: ComponentBase + 'static>(&self, component_id: u32) -> Result<&T, &str>{
        let component = self.try_find_component(component_id)?;
        // Try and convert the rendermesh component we have (Which is currently a trait object) into a rendermesh struct so we can use it
        if let Some(r) = downcast::<T>(&**component) {
            return Ok(r);
        }
        Err("Error downcasting component result")
    }


}