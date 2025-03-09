use crate::{
    energy_parameters::EnergyParameters,
    rna_base::{RnaBase, NOTON},
    scores::{score_external_unpaired, score_hairpin},
};
use std::{collections::HashMap, time::Instant};

pub const VALUE_MIN: i32 = i32::MIN;

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

        let mut number_of_states_h = 0; // hairpin
        let mut number_of_states_p = 0; // pair
        let mut number_of_states_m2 = 0; // two or more pairs
        let mut number_of_states_m = 0; // one or more pairs
        let mut number_of_states_c = 0; // combine
        let mut number_of_states_multi = 0; // multi-loop

        // start of initialize in the original code
        let mut best_c: Vec<BeamState> = vec![BeamState::empty(); sequence.len()];

        let mut best_h: Vec<HashMap<i32, BeamState>> = vec![HashMap::new(); sequence.len()];
        let mut best_p: Vec<HashMap<i32, BeamState>> = vec![HashMap::new(); sequence.len()];
        let mut best_m2: Vec<HashMap<i32, BeamState>> = vec![HashMap::new(); sequence.len()];
        let mut best_m: Vec<HashMap<i32, BeamState>> = vec![HashMap::new(); sequence.len()];
        let mut best_multi: Vec<HashMap<i32, BeamState>> = vec![HashMap::new(); sequence.len()];
        // end of initialize in the original code
        let mut next_pair: Vec<Vec<Option<usize>>> = Vec::with_capacity(5);

        for nuci in 0..NOTON {
            let mut current_next_pair_vec = Vec::with_capacity(sequence.len());
            let mut next = None;
            let mut j = sequence.len() - 1;

            loop {
                current_next_pair_vec.push(next);
                if RnaBase::from_int(nuci).unwrap().can_pair_with(sequence[j]) {
                    next = Some(j);
                }

                if j == 0 {
                    break;
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
            best_c[0].set(-score_external_unpaired(0, 0), Manner::CEqCPlusU);
        }
        if sequence.len() > 1 {
            best_c[1].set(-score_external_unpaired(0, 1), Manner::CEqCPlusU);
        }
        number_of_states_c += 1;

        // from left to right
        for j in 0..sequence.len() {
            let nucj = sequence[j];
            let nucj1 = if j + 1 < sequence.len() {
                Some(sequence[j + 1])
            } else {
                None
            };

            // beam of H
            let mut beam_step_h = &mut best_h[j];
            if self.beam_size > 0 && beam_step_h.len() > self.beam_size {
                println!(
                    "Beam pruning H with scores: {:?}",
                    beam_step_h
                        .iter()
                        .map(|(_k, v)| v.score)
                        .collect::<Vec<_>>()
                );
                let threshold = self.beam_prune(&best_c, beam_step_h);
                println!(
                    "Beam pruned H with threshold={:}; new scores: {:?}",
                    threshold,
                    beam_step_h
                        .iter()
                        .map(|(_k, v)| v.score)
                        .collect::<Vec<_>>()
                );
            }

            let mut jnext = next_pair[nucj.to_int()][j];
            if self.no_sharp_turn {
                while jnext.is_some() && jnext.unwrap() - j < 4 {
                    jnext = next_pair[nucj.to_int()][jnext.unwrap()];
                }
            }

            if let Some(jnext) = jnext {
                let nucjnext = RnaBase::from_int(nucj.to_int()).unwrap();
                let nucjnext_1 = RnaBase::from_int(nucj.to_int() - 1);

                let tetra_hex_tri = if jnext - j - 1 == 4 {
                    // 6:tetra
                    if_tetraloops[j]
                } else if jnext - j - 1 == 6 {
                    // 8: hexa
                    if_hexaloops[j]
                } else if jnext - j - 1 == 3 {
                    // 5: tri
                    if_triloops[j]
                } else {
                    None
                };
                let new_score =
                    -score_hairpin(j, jnext, nucj, nucj1, nucjnext_1, nucjnext, tetra_hex_tri);

                if best_h[jnext][&(j as i32)].score < new_score {
                    if let Some(v) = best_h[jnext].get_mut(&(j as i32)) {
                        v.set(new_score, Manner::H);
                    } else {
                        best_h[jnext].insert(j as i32, BeamState::new(new_score, Manner::H));
                    }
                }
                number_of_states_h += 1;
            }
        }

        "yak".to_string()
    }

    pub fn beam_prune(&self, best_c: &[BeamState], beam_step: &mut HashMap<i32, BeamState>) -> i32 {
        let mut scores: Vec<(i32, usize)> = Vec::with_capacity(beam_step.len());
        for (i, candidate) in beam_step.iter() {
            let k = i - 1;
            let new_score = if k >= 0
                && best_c.get(k as usize).is_some()
                && best_c[k as usize].score == VALUE_MIN
            {
                VALUE_MIN
            } else {
                candidate.score + if k >= 0 { best_c[k as usize].score } else { 0 }
            };
            scores.push((new_score, *i as usize));
        }

        if scores.len() < self.beam_size {
            return VALUE_MIN;
        }

        let no_scores = scores.len();
        let threshold = quickselect(&mut scores, 0, no_scores - 1, no_scores - self.beam_size);

        beam_step.retain(|_, candidate| candidate.score >= threshold);

        threshold
    }
}

// https://github.com/LinearFold/LinearFold/blob/c3ee9bd80c06c2fc39a7bb7ae5e77b9566227cac/src/LinearFold.h#L27-42

#[derive(Clone, Debug, Copy)]
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

#[derive(Clone, Debug, Copy)]
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

// TODO: Optimize this in the future; keeping it as-is right now so that I'm confident it works
// https://github.com/LinearFold/LinearFold/blob/c3ee9bd80c06c2fc39a7bb7ae5e77b9566227cac/src/LinearFold.cpp#L247-L267
pub fn quickselect(scores: &mut [(i32, usize)], lower: usize, upper: usize, k: usize) -> i32 {
    if lower == upper {
        return scores[lower].0;
    }

    let split = quickselect_partition(scores, lower, upper);
    let length = split - lower + 1;
    if length == k {
        return scores[split].0;
    }
    if k < length {
        return quickselect(scores, lower, split - 1, k);
    }
    return quickselect(scores, split + 1, upper, k - length);
}

pub fn quickselect_partition(scores: &mut [(i32, usize)], lower: usize, upper: usize) -> usize {
    let pivot = scores[upper].0;
    let mut lower = lower;
    let mut upper = upper;

    while lower < upper {
        while scores[lower].0 < pivot {
            lower += 1;
        }
        while scores[upper].0 > pivot {
            upper -= 1;
        }
        if scores[lower].0 == scores[upper].0 {
            lower += 1;
        } else if lower < upper {
            scores.swap(lower, upper);
        }
    }

    upper
}
