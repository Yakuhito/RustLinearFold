use crate::rna_base::{RnaBase, NOTON};
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

        // start of initialize in the original code
        let mut bestH = Vec::with_capacity(sequence.len());
        let mut bestP = Vec::with_capacity(sequence.len());
        let mut bestM2 = Vec::with_capacity(sequence.len());
        let mut bestM = Vec::with_capacity(sequence.len());
        let mut bestC = Vec::with_capacity(sequence.len());
        let mut bestMulti = Vec::with_capacity(sequence.len());
        // end of initialize in the original code
        let mut next_pair: Vec<Vec<Option<usize>>> = Vec::with_capacity(5);

        for nuci in 0..NOTON {
            let mut current_next_pair_vec = Vec::with_capacity(sequence.len());
            let mut next = None;
            let mut j = sequence.len() - 1;

            while j >= 0 {
                current_next_pair_vec.push(next);
                if RnaBase::from_int(nuci).unwrap().can_pair_with(sequence[j]) {
                    next = Some(j);
                }
                j -= 1;
            }

            next_pair.push(current_next_pair_vec);
        }

        // start of v_init_tetra_hex_tri in the original code
        /*
            // TetraLoops
        if_tetraloops.resize(seq_length-5<0?0:seq_length-5, -1);
        for (int i = 0; i < seq_length-5; ++i) {
            if (!(seq[i] == 'C' && seq[i+5] == 'G'))
                continue;
            char *ts;
            if ((ts=strstr(Tetraloops, seq.substr(i,6).c_str())))
                if_tetraloops[i] = (ts - Tetraloops)/7;
        }

        // Triloops
        if_triloops.resize(seq_length-4<0?0:seq_length-4, -1);
        for (int i = 0; i < seq_length-4; ++i) {
            if (!((seq[i] == 'C' && seq[i+4] == 'G') || (seq[i] == 'G' && seq[i+4] == 'C')))
                continue;
            char *ts;
            if ((ts=strstr(Triloops, seq.substr(i,5).c_str())))
                if_triloops[i] = (ts - Triloops)/6;
        }

        // Hexaloops
        if_hexaloops.resize(seq_length-7<0?0:seq_length-7, -1);
        for (int i = 0; i < seq_length-7; ++i) {
            if (!(seq[i] == 'A' && seq[i+7] == 'U'))
                continue;
            char *ts;
            if ((ts=strstr(Hexaloops, seq.substr(i,8).c_str())))
                if_hexaloops[i] = (ts - Hexaloops)/9;
        }
            */
        let mut if_tetraloops = Vec::with_capacity(sequence.len() - 5);

        // end of v_init_tetra_hex_tri in the original code

        "yak".to_string()
    }
}
