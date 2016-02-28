pub struct WorldBuilder {
    components: Vec<Component>
}

struct Component {
    path: &'static str
}

impl WorldBuilder {
    pub fn new() -> WorldBuilder {
        WorldBuilder {
            components: vec![],
        }
    }

    pub fn add_component(&mut self, path: &'static str) -> &mut Self {
        let component = Component::new(path);
        if self.components.iter().any(|c| c.name() == component.name()) {
            panic!("Two components with the same name ({}) are forbidden.", component.name());
        }
        self.components.push(component);
        self
    }

    pub fn render(&self) -> String {
        let mut out = String::new();

        // Prelude
        out.push_str("#![allow(non_snake_case)]\n");

        // World struct
        out.push_str("pub struct World {\n");
        for component in self.components.iter() {
            out.push_str(&format!("    {}Coll: Vec<{}>,\n", component.name(), component.path()));
        }

        out.push_str("}\n");

        // Component trait
        out.push_str(r#"
pub trait Component {
    #[inline(always)]
    fn id() -> u32;
}
"#);

        // Component trait implementations
        for (i, component_path) in self.components.iter().map(Component::path).enumerate() {
            out.push_str(&format!(r#"
impl Component for {} {{
    #[inline(always)]
    fn id() -> u32 {{ {} }}
}}
"#, component_path, i));
        }

        out
    }
}

impl Component {
    pub fn new(path: &'static str) -> Component {
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
