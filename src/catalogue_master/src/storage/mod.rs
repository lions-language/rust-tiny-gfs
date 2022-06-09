mod mem_disk;

pub struct NSNode {
    name: String,
}

pub struct Data {
    namespace: Vec<NSNode>,
    name: String,
}

pub trait Storage {}
