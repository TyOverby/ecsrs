pub struct WorldBuilder {
    components: Vec<Component>,
    systems: Vec<System>,

}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Component {
    path: &'static str
}

#[derive(Clone, Eq, PartialEq)]
pub struct System {
    path: &'static str,
    components: Vec<Component>
}

impl WorldBuilder {
    pub fn new() -> WorldBuilder {
        WorldBuilder {
            components: vec![],
            systems: vec![],
        }
    }

    pub fn add_component(&mut self, path: &'static str) -> Component {
        let component = Component::new(path);
        if self.components.iter().any(|c| c.name() == component.name()) {
            panic!("Two components with the same name ({}) are forbidden.", component.name());
        }
        self.components.push(component);
        component
    }

    pub fn add_system(&mut self, system: System)  {
        self.systems.push(system);
    }

    pub fn render(&self) -> String {
        let mut out = String::new();

        // Prelude
        out.push_str("#![allow(non_snake_case)]\n");
        out.push_str("use std::cell::UnsafeCell;\n");
        out.push_str("use std::marker::Sync;\n");
        out.push_str("use ecsrs::component_container::ComponentContainer;\n");

        // World struct
        out.push_str(r#"
pub struct World {
    store: ComponentStore
}
"#);

        // Entity struct
        out.push_str(r#"
pub struct Entity {
    id: usize
}
"#);


        // Component store struct
        out.push_str("\nstruct ComponentStore {\n");
        for component in self.components.iter() {
            out.push_str(&format!("    {}Coll: UnsafeCell<ComponentContainer<{}>>,\n", component.name(), component.path()));
        }
        out.push_str("}\n");
        out.push_str("\nunsafe impl Sync for ComponentStore {}\n");

        // ComponentId trait
        out.push_str(r#"
pub trait ComponentId {
    #[inline(always)]
    fn component_id() -> u32;
}
"#);

        // SystemId trait
        out.push_str(r#"
pub trait SystemId {
    #[inline(always)]
    fn system_id() -> u32;
}
"#);

        // ComponentId trait implementations
        for (i, component_path) in self.components.iter().map(Component::path).enumerate() {
            out.push_str(&format!(r#"
impl ComponentId for {} {{
    #[inline(always)]
    fn component_id() -> u32 {{ {} }}
}}
"#, component_path, i));
        }

        // SystemId trait implementations
        for (i, system_path) in self.systems.iter().map(System::path).enumerate() {
            out.push_str(&format!(r#"
impl SystemId for {} {{
    #[inline(always)]
    fn system_id() -> u32 {{ {} }}
}}
"#, system_path, i));
        }

        // System accessor definitions

        for system in self.systems.iter() {
            out.push_str(&format!(r#"
struct {}Accessor<'a> {{
    store: &'a mut ComponentStore,
}}
"#, system.name()));

            out.push_str(&format!(r#"
impl <'a> {}Accessor<'a> {{
"#, system.name()));
            for component in system.components.iter() {
                out.push_str(&format!(r#"
    pub fn get_{}(&self) -> &mut {} {{
        unimplemented!();
    }}
"#, &String::from(component.name()).to_lowercase(), component.path()));
            }
            out.push_str("}");
        }

        out
    }
}

impl Component {
    fn new(path: &'static str) -> Component {
        // TODO: do name validation here
        // Path should be valid.  Name should be not one of the reserved names.
        Component{path: path}
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn name(&self) -> &'static str {
        match self.path.rfind(':') {
            Some(idx) => &self.path[idx + 1 ..],
            None => &self.path
        }
    }
}

impl System {
    pub fn new(path: &'static str) -> System {
        System {
            path: path,
            components: vec![]
        }
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn name(&self) -> &'static str {
        match self.path.rfind(':') {
            Some(idx) => &self.path[idx + 1 ..],
            None => &self.path
        }
    }

    pub fn with_component(mut self, component: Component) -> System {
        self.components.push(component);
        self
    }
}
