use crate::{
    energy_parameters::EnergyParameters,
    rna_base::{RnaBase, NOTON},
    scores::score_external_unpaired,
};
use std::time::Instant;
pub struct BeamCKYParser {
    pub beam_size: usize,
    pub no_sharp_turn: bool,
}

impl BeamCKYParser {
    pub fn new(beam_size: usize, no_sharp_turn: bool) -> Self {
        Self {
            beam_size,
            no_sharp_turn,
        }
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

            if_tetraloops.push(EnergyParameters::get_tetraloop_energy(
                sequence[i..i + 6].try_into().unwrap(),
            ));
        }

        // Triloops
        let mut if_triloops = Vec::with_capacity(sequence.len() - 4);
        for i in 0..sequence.len() - 4 {
            if !((sequence[i] == RnaBase::C && sequence[i + 4] == RnaBase::G)
                || (sequence[i] == RnaBase::G && sequence[i + 4] == RnaBase::C))
            {
                continue;
            }

            if_triloops.push(EnergyParameters::get_triloop_energy(
                sequence[i..i + 5].try_into().unwrap(),
            ));
        }

        // Hexaloops
        let mut if_hexaloops = Vec::with_capacity(sequence.len() - 7);
        for i in 0..sequence.len() - 7 {
            if !(sequence[i] == RnaBase::A && sequence[i + 7] == RnaBase::U) {
                continue;
            }

            if_hexaloops.push(EnergyParameters::get_hexaloop_energy(
                sequence[i..i + 8].try_into().unwrap(),
            ));
        }
        // end of v_init_tetra_hex_tri in the original code

        // start CKY decoding
        if sequence.len() > 0 {
            bestC.push((-score_external_unpaired(0, 0), Manner::CEqCPlusU));
        }
        if sequence.len() > 0 {
            bestC.push((-score_external_unpaired(0, 1), Manner::CEqCPlusU));
        }
        number_of_states_C += 1;

        // from left to right
        for j in 0..sequence.len() {
            let nucj = sequence[j];
            let nucj1 = if j + 1 < sequence.len() {
                Some(sequence[j + 1])
            } else {
                None
            };

            // beam of H
            // let mut beam_step_h = if let Some(h) = bestH.get(j) {
            //     vec![h]
            // } else {
            //     vec![]
            // };

            // if self.beamSize > 0 && beam_step_h.len() > self.beamSize {
            //     self.beam_prune(beam_step_h);
            // }
        }

        "yak".to_string()
    }

    // pub fn beam_prune(&self, bestC: &[(i32, Manner)], beam_step: &mut Vec<(i32, Manner)>) {
    //     let scores: Vec<(i32, usize)> = Vec::with_capacity(beam_step.len());
    //     for item in beam_step {
    //         let i = beam_step.0;
    //         let cand = beam_step.1;
    //         let k = i - 1;
    //         let new_score = if k >= 0 && bestC.get(k).is_some() && bestC[k].0 == VALUE_MIN {
    //             new_score = VALUE_MIN;
    //         } else {
    //             cand.
    //         }
    //     }
    // }
}

// https://github.com/LinearFold/LinearFold/blob/c3ee9bd80c06c2fc39a7bb7ae5e77b9566227cac/src/LinearFold.h#L27-42
pub enum Manner {
    None,              // 0: empty
    H,                 // 1: hairpin candidate
    Hairpin,           // 2: hairpin
    Single,            // 3: single
    Helix,             // 4: helix
    Multi,             // 5: multi = ..M2. [30 restriction on the left and jump on the right]
    MultiEqMultiPlusU, // 6: multi = multi + U
    PEqMulti,          // 7: P = (multi)
    M2EqMPlusP,        // 8: M2 = M + P
    MEqM2,             // 9: M = M2
    MEqMPlusU,         // 10: M = M + U
    MEqP,              // 11: M = P
    CEqCPlusU,         // 12: C = C + U
    CEqCPlusP,         // 13: C = C + P
}

pub struct BeamState {
    pub score: i32,
    pub manner: Manner,
    pub split: Option<usize>,
    pub padding_l1: Option<char>,
    pub padding_l2: Option<i32>,
}

impl BeamState {
    pub fn empty() -> Self {
        Self {
            score: VALUE_MIN,
            manner: Manner::None,
            split: None,
            padding_l1: None,
            padding_l2: None,
        }
    }

    pub fn new(score: i32, manner: Manner) -> Self {
        Self {
            score,
            manner,
            split: None,
            padding_l1: None,
            padding_l2: None,
        }
    }

    pub fn set(&mut self, score: i32, manner: Manner) {
        self.score = score;
        self.manner = manner;
    }

    pub fn set_split(&mut self, split: usize) {
        self.split = Some(split);
    }

    pub fn set_padding(&mut self, padding_l1: char, padding_l2: i32) {
        self.padding_l1 = Some(padding_l1);
        self.padding_l2 = Some(padding_l2);
    }
}
