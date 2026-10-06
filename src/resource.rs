#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Stone,
    Wood,
    Coal,
}

#[derive(Debug, Clone, Copy ,PartialEq, Eq)]
pub struct ResourceStack {
    pub kind: ResourceType,
    pub amount: u32,
}
