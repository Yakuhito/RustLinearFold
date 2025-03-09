pub enum RnaBase {
    A,
    C,
    G,
    U,
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
}
