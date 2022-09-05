#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnKind {
    Subway,
    Tram,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineName {
    Placeholder,
    U1 { kind: ConnKind },
    U2 { kind: ConnKind },
    U3 { kind: ConnKind },
    U4 { kind: ConnKind },
    U5 { kind: ConnKind },
    U6 { kind: ConnKind },
    U7 { kind: ConnKind },
    U8 { kind: ConnKind },
    T12 { kind: ConnKind },
    T16 { kind: ConnKind },
    T17 { kind: ConnKind },
    T18 { kind: ConnKind },
    T19 { kind: ConnKind },
    T20 { kind: ConnKind },
    T21 { kind: ConnKind },
    T23 { kind: ConnKind },
    T25 { kind: ConnKind },
    T27 { kind: ConnKind },
    T28 { kind: ConnKind },
    T29 { kind: ConnKind },
}

impl LineName {
    pub fn get_conn_kind(&self) -> &ConnKind {
        match self {
            LineName::U1 { kind } => return kind,
            LineName::U2 { kind } => return kind,
            LineName::U3 { kind } => return kind,
            LineName::U4 { kind } => return kind,
            LineName::U5 { kind } => return kind,
            LineName::U6 { kind } => return kind,
            LineName::U7 { kind } => return kind,
            LineName::U8 { kind } => return kind,
            LineName::T12 { kind } => return kind,
            LineName::T16 { kind } => return kind,
            LineName::T17 { kind } => return kind,
            LineName::T18 { kind } => return kind,
            LineName::T19 { kind } => return kind,
            LineName::T20 { kind } => return kind,
            LineName::T21 { kind } => return kind,
            LineName::T23 { kind } => return kind,
            LineName::T25 { kind } => return kind,
            LineName::T27 { kind } => return kind,
            LineName::T28 { kind } => return kind,
            LineName::T29 { kind } => return kind,
            LineName::Placeholder => panic!("Placeholder has no kind."),
        }
    }
}
