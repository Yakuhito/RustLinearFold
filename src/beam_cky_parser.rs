use crate::{
    energy_parameters::EnergyParameters,
    rna_base::{RnaBase, NOTON},
};
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
        // Tetraloops
        let mut if_tetraloops = Vec::with_capacity(sequence.len() - 5);
        for i in 0..sequence.len() - 5 {
            if !(sequence[i] == RnaBase::C && sequence[i + 5] == RnaBase::G) {
                continue;
            }

            let energy =
                EnergyParameters::get_tetraloop_energy(sequence[i..i + 6].try_into().unwrap());
            if let Some(energy) = energy {
                if_tetraloops[i] = energy;
            }
        }

        // Triloops
        let mut if_triloops = Vec::with_capacity(sequence.len() - 4);
        for i in 0..sequence.len() - 4 {
            if !((sequence[i] == RnaBase::C && sequence[i + 4] == RnaBase::G)
                || (sequence[i] == RnaBase::G && sequence[i + 4] == RnaBase::C))
            {
                continue;
            }

            let energy =
                EnergyParameters::get_triloop_energy(sequence[i..i + 5].try_into().unwrap());
            if let Some(energy) = energy {
                if_triloops[i] = energy;
            }
        }

        // Hexaloops
        let mut if_hexaloops = Vec::with_capacity(sequence.len() - 7);
        for i in 0..sequence.len() - 7 {
            if !(sequence[i] == RnaBase::A && sequence[i + 7] == RnaBase::U) {
                continue;
            }

            let energy =
                EnergyParameters::get_hexaloop_energy(sequence[i..i + 8].try_into().unwrap());
            if let Some(energy) = energy {
                if_hexaloops[i] = energy;
            }
        }
        // end of v_init_tetra_hex_tri in the original code

        "yak".to_string()
    }
}
