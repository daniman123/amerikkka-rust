extern crate ndarray;
extern crate statrs;
pub mod bs_pricing;
use crate::bs_pricing::EuropeanBSPricing;

fn main() {
    let is_call_val = false;
    let spot_price_val = 80.0;
    let strike_val = 80.0;
    let expiry_val = 1.0;
    // let lognormal_val = true;
    let r_val = 0.03;
    let sigma_val = 0.3;

    let opt_obj = EuropeanBSPricing::new(
        is_call_val,
        spot_price_val,
        strike_val,
        expiry_val,
        r_val,
        sigma_val,
    );
    println!("{}", opt_obj.option_price);
}
