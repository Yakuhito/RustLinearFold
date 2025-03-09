use crate::{
    energy_parameters::{TerminalAU37, HAIRPIN37, MISMATCH_H37},
    rna_base::RnaBase,
};

pub fn score_external_unpaired(_i: usize, _j: usize) -> i32 {
    return 0;
}

/*
inline int v_score_hairpin(int i, int j, int nuci, int nuci1, int nucj_1, int nucj, int tetra_hex_tri_index = -1) {
    int size = j-i-1;
    int type = NUC_TO_PAIR(nuci, nucj);
    /* int si1 = NUM_TO_NUC(nuci1); */
    /* int sj1 = NUM_TO_NUC(nucj_1); */

    int energy;

    if(size <= 30)
        energy = hairpin37[size];
    else
        energy = hairpin37[30] + (int)(lxc37*log((size)/30.));

    if(size < 3) return energy; /* should only be the case when folding alignments */
#ifdef SPECIAL_HP
    // if(special_hp){
        if (size == 4 && tetra_hex_tri_index > -1)
            return Tetraloop37[tetra_hex_tri_index];
        else if (size == 6 && tetra_hex_tri_index > -1)
            return Hexaloop37[tetra_hex_tri_index];
        else if (size == 3) {
            if (tetra_hex_tri_index > -1)
                return Triloop37[tetra_hex_tri_index];
            return (energy + (type>2 ? TerminalAU37 : 0));
        }
    // }
#endif

    energy += mismatchH37[type][nuci1][nucj_1];

    return energy;
}
*/
pub fn score_hairpin(
    i: usize,
    j: usize,
    nuci: RnaBase,
    nuci1: Option<RnaBase>,
    nucj_1: Option<RnaBase>,
    nucj: RnaBase,
    tetra_hex_tri_energy: Option<i32>,
) -> i32 {
    let size = j - i - 1;
    let mut energy = if size <= 30 {
        HAIRPIN37[size]
    } else {
        let lxc37: f64 = 107.856;
        HAIRPIN37[30] + (lxc37 * ((size as f64) / 30.0).ln()).floor() as i32
    };

    if size < 3 {
        return energy;
    }

    if size == 4 && tetra_hex_tri_energy.is_some() {
        return tetra_hex_tri_energy.unwrap();
    }

    if size == 6 && tetra_hex_tri_energy.is_some() {
        return tetra_hex_tri_energy.unwrap();
    }

    if size == 3 {
        if tetra_hex_tri_energy.is_some() {
            return tetra_hex_tri_energy.unwrap();
        }

        // 3:GU 4:UG 5:AU 6:UA 7:NN
        // add TerminalAU37 only if pair is AU or GU or NN
        return energy
            + match (nuci, nucj) {
                (RnaBase::U, RnaBase::A)
                | (RnaBase::A, RnaBase::U)
                | (RnaBase::G, RnaBase::U)
                | (RnaBase::U, RnaBase::G)
                | (RnaBase::N, RnaBase::N) => TerminalAU37,
                _ => 0,
            };
    }

    energy += MISMATCH_H37[nuci.to_pair_int(nucj)][nuci1.map(|n| n.to_int()).unwrap_or(0)]
        [nucj_1.map(|n| n.to_int()).unwrap_or(0)];

    energy
}
