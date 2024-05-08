use crate::statrs::distribution::{Continuous, ContinuousCDF};
use statrs::distribution::Normal;
use std::collections::HashMap;

pub struct EuropeanBSPricing {
    is_call: bool,
    spot_price: f64,
    strike: f64,
    expiry: f64,
    r: f64,
    sigma: f64,
    pub option_price: f64,
    pub greeks: HashMap<String, f64>,
}

impl EuropeanBSPricing {
    pub fn new(
        is_call: bool,
        spot_price: f64,
        strike: f64,
        expiry: f64,
        r: f64,
        sigma: f64,
    ) -> EuropeanBSPricing {
        let mut ebsp = EuropeanBSPricing {
            is_call,
            spot_price,
            strike,
            expiry,
            r,
            sigma,
            option_price: 0.0,
            greeks: HashMap::new(),
        };
        ebsp.option_price = ebsp.get_option_price();
        ebsp.greeks = ebsp.get_greeks();
        ebsp
    }

    fn get_d1_d2(&self) -> (f64, f64) {
        let sigma_sqrt = self.sigma * self.expiry.sqrt();
        let d1 = (self.spot_price / self.strike).ln()
            + (self.r + self.sigma.powi(2) / 2.0) * self.expiry / sigma_sqrt;
        let d2 = d1 - sigma_sqrt;
        (d1, d2)
    }

    fn get_option_price(&self) -> f64 {
        let (d1, d2) = self.get_d1_d2();
        let norm = Normal::new(0.0, 1.0).unwrap();
        if self.is_call {
            self.spot_price * norm.cdf(d1)
                - self.strike * (-self.r * self.expiry).exp() * norm.cdf(d2)
        } else {
            self.strike * (-self.r * self.expiry).exp() * norm.cdf(-d2)
                - self.spot_price * norm.cdf(-d1)
        }
    }

    fn get_greeks(&self) -> HashMap<String, f64> {
        let (d1, d2) = self.get_d1_d2();
        let sqrtt = self.expiry.sqrt();
        let norm = Normal::new(0.0, 1.0).unwrap();

        let gamma = norm.pdf(d1) / (self.spot_price * self.sigma * sqrtt);
        let vega = self.spot_price * sqrtt * norm.pdf(d1);
        let rho_temp = -self.strike * self.expiry * (-self.r * self.expiry).exp();
        let theta_temp1 = (self.spot_price * self.sigma * norm.pdf(d1)) / (2.0 * sqrtt);
        let theta_temp2 = self.r * self.strike * (-self.r * self.expiry).exp();

        let mut greeks = HashMap::new();
        if self.is_call {
            greeks.insert("Delta".to_string(), norm.cdf(d1));
            greeks.insert("Gamma".to_string(), gamma);
            greeks.insert(
                "Theta".to_string(),
                -theta_temp1 - theta_temp2 * norm.cdf(d2),
            );
            greeks.insert("Vega".to_string(), vega);
            greeks.insert("Rho".to_string(), rho_temp * norm.cdf(d2));
        } else {
            greeks.insert("Delta".to_string(), -norm.cdf(-d1));
            greeks.insert("Gamma".to_string(), gamma);
            greeks.insert(
                "Theta".to_string(),
                -theta_temp1 + theta_temp2 * norm.cdf(-d2),
            );
            greeks.insert("Vega".to_string(), vega);
            greeks.insert("Rho".to_string(), rho_temp * norm.cdf(-d2));
        }
        greeks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bs_pricing() {
        let is_call_val = false;
        let spot_price_val = 80.0;
        let strike_val = 78.0;
        let expiry_val = 2.0;
        let r_val = 0.02;
        let sigma_val = 0.25;
        let opt_obj = EuropeanBSPricing::new(
            is_call_val,
            spot_price_val,
            strike_val,
            expiry_val,
            r_val,
            sigma_val,
        );
        println!("{}", opt_obj.option_price);
        for (key, value) in &opt_obj.greeks {
            println!("{}: {}", key, value);
        }
    }
}
