#[derive(Debug, Clone, PartialEq)]
pub enum LineName {
    Placeholder,
    U(i32),
    T(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Pos,
    Neg,
}
