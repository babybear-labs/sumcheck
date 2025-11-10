use ark_ff::Field;
use ark_ff::PrimeField;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MultiVariatePolynomial<F: PrimeField> {
    pub coeffs: HashMap<Vec<u64>, F>
}

impl<F: PrimeField> MultiVariatePolynomial<F> {
    pub fn new(coeffs: HashMap<Vec<u64>, F>) -> Self {
        Self {
            coeffs
        }
    }

    pub fn evaluate(&self, point: &[F]) -> F {
        let mut result = F::zero();
        for (degrees, coeff) in self.coeffs.iter() {
            let mut term = *coeff;
            for (var, deg) in degrees.iter().enumerate() {
                term *= point[var].pow(&[*deg]);
            }
            result += term;
        }
        result
    }

    pub fn get_univariate_at_round(
        &self,
        i: usize,
        previous_values: &[F],
    ) -> UnivariatePolynomial<F> {
        let mut uni_coeffs = HashMap::new();

        for (degrees, coeff) in self.coeffs.iter() {
            let mut term_coeff = *coeff;

            for (j, val) in previous_values.iter().enumerate() {
                if j != i {
                    term_coeff *= val.pow(&[degrees[j]]);
                }
            }

            let degree = degrees[i];
            *uni_coeffs.entry(degree).or_insert(F::zero()) += term_coeff;
        }

        UnivariatePolynomial {
            coeffs: uni_coeffs,
        }
    }
}

#[derive(Clone)]
pub struct UnivariatePolynomial<F: Field> {
    pub coeffs: HashMap<u64, F>,
}

impl<F: Field> UnivariatePolynomial<F> {
    pub fn evaluate(&self, point: F) -> F {
        let mut result = F::zero();
        for (deg, coeff) in self.coeffs.iter() {
            result += *coeff * point.pow(&[*deg]);
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::Fr; // Using BLS12-381 scalar field for testing (PrimeField)
    use ark_ff::UniformRand;
    use rand::thread_rng;

    #[test]
    fn test_multivariate_polynomial_evaluation_and_univariate_extraction() {
        // Example: f(x, y) = 3*x^2*y + 5*y^2 + 7
        //
        // Represented as a HashMap where key = [deg_x, deg_y], value = coefficient
        // {
        //   [2, 1] => 3
        //   [0, 2] => 5
        //   [0, 0] => 7
        // }

        let mut coeffs = HashMap::new();
        coeffs.insert(vec![2, 1], Fr::from(3u64)); // 3 * x^2 * y
        coeffs.insert(vec![0, 2], Fr::from(5u64)); // 5 * y^2
        coeffs.insert(vec![0, 0], Fr::from(7u64)); // 7 (constant)

        let poly = MultiVariatePolynomial::new(coeffs);

        // Evaluate f(2, 3) = 3*(2^2)*(3) + 5*(3^2) + 7 = 3*4*3 + 5*9 + 7 = 36 + 45 + 7 = 88
        let x = Fr::from(2u64);
        let y = Fr::from(3u64);
        let result = poly.evaluate(&[x, y]);

        assert_eq!(result, Fr::from(88u64));

        // --------------------------------------------------------------------
        // Test get_univariate_at_round
        // --------------------------------------------------------------------
        // Suppose we fix y = 3, and extract a univariate polynomial in x:
        // f(x, 3) = (3*3)*x^2 + (5*9 + 7)
        //          = 9*x^2 + 52
        // So univariate coefficients: { 2: 9, 0: 52 }

        let uni_poly = poly.get_univariate_at_round(0, &[x, y]); // Here previous_values includes both vars, but we only fix y.

        // Evaluate at x = 2 => 9*(2^2) + 52 = 9*4 + 52 = 88
        let eval_univariate = uni_poly.evaluate(x);

        assert_eq!(eval_univariate, result);

        // --------------------------------------------------------------------
        // Randomized sanity check: evaluate f(x, y) and extracted univariate at same (x, y)
        // --------------------------------------------------------------------
        let mut rng = thread_rng();
        let rand_x = Fr::rand(&mut rng);
        let rand_y = Fr::rand(&mut rng);

        let fxy = poly.evaluate(&[rand_x, rand_y]);
        let uni_poly_y_fixed = poly.get_univariate_at_round(0, &[rand_x, rand_y]);
        let fx_from_uni = uni_poly_y_fixed.evaluate(rand_x);

        assert_eq!(fxy, fx_from_uni, "Evaluation mismatch between f(x,y) and sliced univariate polynomial");
    }
}