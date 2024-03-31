mod buechi;
mod kripke;
mod model_checking_error;
mod parsing;

pub use kripke::KripkeBuilder;
pub use model_checking_error::ModelCheckingError;
pub use model_checking_error::ModelCheckingErrorKind;

extern crate bit_vec;

use buechi::ltl_to_buechi::ltl_to_büchi;
use buechi::Büchi;
use parsing::LTLFormula;

pub fn ltl_model_check(
    ks: KripkeBuilder,
    formula: &str,
) -> Result<Option<Vec<(u64, (u64, u8))>>, ModelCheckingError> {
    let (ltl, ap_map) = parsing::parse(formula)?;
    let notltl = LTLFormula::Not(Box::new(ltl));

    let model = ks.create_büchi(&ap_map)?;

    let generalized_büchi = ltl_to_büchi(&notltl);
    let büchi = Büchi::from_generalized_büchi(generalized_büchi);
    let product = buechi::product::product(&model, &büchi);
    let opt_loop = product.get_loop();
    Ok(opt_loop)
}
