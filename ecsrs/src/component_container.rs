pub struct ComponentContainer<T> {
    components: Vec<Option<T>>,
    holes: Vec<usize>,
}

impl <T> ComponentContainer<T> {
    pub fn new() -> ComponentContainer<T> {
        ComponentContainer {
            components: vec![],
            holes: vec![],
        }
    }

    pub fn add(&mut self, t: T) -> usize {
        if let Some(p) = self.holes.pop() {
            self.components[p] = Some(t);
            p
        } else {
            let p = self.components.len();
            self.components.push(Some(t));
            p
        }
    }

    pub fn remove(&mut self, pos: usize) {
        self.components[pos] = None;
        self.holes.push(pos);
    }
}
