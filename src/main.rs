mod parsing;
mod buechi;
extern crate bit_vec;

fn main() {
    let formula = parsing::parse("a U !((Xb) & c)").expect("Got no result");

    let buechi = buechi::ltl_to_buechi::ltl_to_büchi(&formula);

    dbg!(buechi);

}

