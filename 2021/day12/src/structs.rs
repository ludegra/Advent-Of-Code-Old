#[derive(Debug)]
pub struct Cave {
    _name: String,
    connections: Vec<String>
}
impl Cave {
    pub fn new(name: String) -> Self {
        Self {
            _name: name,
            connections: Vec::new(),
        }
    }
    pub fn add_connection(&mut self, connection: String) {
        if !self.connections.contains(&connection) {
            self.connections.push(connection);
        }
    }
    pub fn get_connections(&self) -> &Vec<String> {
        &self.connections
    }
}