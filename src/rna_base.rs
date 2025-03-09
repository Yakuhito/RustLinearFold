#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RnaBase {
    A,
    C,
    G,
    U,
    N,
}

impl RnaBase {
    pub fn from_char(c: char) -> Option<RnaBase> {
        match c {
            'A' => Some(RnaBase::A),
            'C' => Some(RnaBase::C),
            'G' => Some(RnaBase::G),
            'U' => Some(RnaBase::U),
            _ => None,
        }
    }

    pub fn can_pair_with(&self, other: RnaBase) -> bool {
        match (self, other) {
            (RnaBase::G, RnaBase::C) => true,
            (RnaBase::C, RnaBase::G) => true,
            (RnaBase::G, RnaBase::U) => true,
            (RnaBase::U, RnaBase::G) => true,
            (RnaBase::A, RnaBase::U) => true,
            (RnaBase::U, RnaBase::A) => true,
            _ => false,
        }
    }

    // lhuang: Vienna: 0N 1A 2C 3G 4U
    // #define GET_ACGU_NUM(x)   GET_ACGU_NUM_V(x) //((x=='A'? 1 : (x=='C'? 2 : (x=='G'? 3 : (x=='U'?4: 0)))))
    pub fn to_int(&self) -> usize {
        match self {
            RnaBase::A => 1,
            RnaBase::C => 2,
            RnaBase::G => 3,
            RnaBase::U => 4,
            _ => 0,
        }
    }

    pub fn from_int(i: usize) -> Option<RnaBase> {
        match i {
            1 => Some(RnaBase::A),
            2 => Some(RnaBase::C),
            3 => Some(RnaBase::G),
            4 => Some(RnaBase::U),
            _ => None,
        }
    }
}
