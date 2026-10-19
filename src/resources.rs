#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ResourceType {
    Stone,
    Wood,
    Coal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceStack {
    pub kind: ResourceType,
    pub amount: i32,
}

impl ResourceStack {
    fn new(kind: ResourceType, amount: i32) -> Self {
        Self { kind, amount }
    }
}

