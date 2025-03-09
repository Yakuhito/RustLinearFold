#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RnaBase {
    A,
    C,
    G,
    U,
    N,
}

pub const NOTON: usize = 5; // # values in RnaBase

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

    // pairs: 0:NP 1:CG 2:GC 3:GU 4:UG 5:AU 6:UA 7:NN
    // #define NUC_TO_PAIR(x,y) (x==1? (y==4?5:0) : (x==2? (y==3?1:0) : (x==3 ? (y==2?2:(y==4?3:0)) : (x==4 ? (y==3?4:(y==1?6:0)) : 0))))
    pub fn to_pair_int(&self, other: RnaBase) -> usize {
        match (self, other) {
            (RnaBase::N, RnaBase::N) => 7,
            (RnaBase::U, RnaBase::A) => 6,
            (RnaBase::A, RnaBase::U) => 5,
            (RnaBase::U, RnaBase::G) => 4,
            (RnaBase::G, RnaBase::U) => 3,
            (RnaBase::G, RnaBase::C) => 2,
            (RnaBase::C, RnaBase::G) => 1,
            _ => 0,
        }
    }
}

#[macro_export]
macro_rules! rna_base_sequence_matches {
    ($sequence:expr, $pattern:expr) => {{
        let pattern_bases: Vec<RnaBase> = $pattern
            .chars()
            .map(|c| RnaBase::from_char(c).unwrap())
            .collect();
        $sequence == &pattern_bases[..]
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rna_base_sequence_matches() {
        let sequence = vec![
            RnaBase::C,
            RnaBase::A,
            RnaBase::A,
            RnaBase::C,
            RnaBase::G,
            RnaBase::G,
        ];
        assert!(rna_base_sequence_matches!(&sequence, "CAACGG"));
        assert!(!rna_base_sequence_matches!(&sequence, "CAACGU"));
    }
}
