use crate::rna_base::RnaBase;
use std::time::Instant;
pub struct BeamCKYParser {
    pub beam_size: usize,
}

impl BeamCKYParser {
    pub fn new(beam_size: usize) -> Self {
        Self { beam_size }
    }
}

impl BeamCKYParser {
    pub fn parse(&self, sequence: &[RnaBase]) -> String {
        let start_time = Instant::now();

        let mut number_of_states_H = 0; // hairpin
        let mut number_of_states_P = 0; // pair
        let mut number_of_states_M2 = 0; // two or more pairs
        let mut number_of_states_M = 0; // one or more pairs
        let mut number_of_states_C = 0; // combine
        let mut number_of_states_multi = 0; // multi-loop

        let mut bestH = Vec::with_capacity(sequence.len());
        let mut bestP = Vec::with_capacity(sequence.len());
        let mut bestM2 = Vec::with_capacity(sequence.len());
        let mut bestM = Vec::with_capacity(sequence.len());
        let mut bestC = Vec::with_capacity(sequence.len());
        let mut bestMulti = Vec::with_capacity(sequence.len());

        let mut next_pair: Vec<Vec<Option<usize>>> = Vec::with_capacity(5);

        for nuci in 0..next_pair.len() {
            let mut current_next_pair_vec = Vec::with_capacity(sequence.len());
            let mut next = None;
            let mut j = sequence.len() - 1;

            while j >= 0 {
                current_next_pair_vec.push(next);
                if sequence[RnaBase::from_int(nuci).unwrap()].can_pair_with(sequence[j]) {
                    next = Some(j);
                }
                j -= 1;
            }

            next_pair.push(current_next_pair_vec);
        }

        // start of v_init_tetra_hex_tri in the original code
        let mut if_tetraloops = Vec::with_capacity(sequence.len() - 5);
        for k in 

        // end of v_init_tetra_hex_tri in the original code

        "yak".to_string()
    }
}
