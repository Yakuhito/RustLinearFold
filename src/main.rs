use rna_base::RnaBase;

mod beam_cky_parser;
mod rna_base;

fn main() {
    println!("RustLinearFold v0.1.0");
    let beam_size = 100;
    let sequence_str = "GUUUUUAUCUUACACACGCUUGUGUAAGAUAGUUA";

    let sequence = sequence_str
        .chars()
        .map(|c| RnaBase::from_char(c).unwrap())
        .collect::<Vec<_>>();

    println!(
        "Folding sequence of length {} with beam size {}",
        sequence.len(),
        beam_size
    );
}
