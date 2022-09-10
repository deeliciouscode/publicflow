#[derive(Debug, Clone, Hash)]
pub enum LineName {
    Placeholder,
    U(i32),
    T(i32),
}

impl PartialEq for LineName {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&LineName::U(a), &LineName::U(b)) => a == b,
            (&LineName::T(a), &LineName::T(b)) => a == b,
            (&LineName::Placeholder, &LineName::Placeholder) => true,
            _ => false,
        }
    }
}

impl Eq for LineName {}

impl LineName {
    pub fn in_same_line_class(&self, other: &Self) -> bool {
        match (self, other) {
            (&LineName::U(_), &LineName::U(_)) => true,
            (&LineName::T(_), &LineName::T(_)) => true,
            (&LineName::Placeholder, &LineName::Placeholder) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Pos,
    Neg,
}
