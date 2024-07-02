pub(crate) mod backend;
pub mod errs;
pub mod primitives;

use ark_ff::PrimeField;
use ark_std::fmt::Debug;
use errs::KZGResult;

use crate::primitives::poly::FpPolynomial;

/// The trait for serialization to bytes
pub trait ToBytes {
    /// Convert to bytes.
    fn to_bytes(&self) -> Vec<u8>;
}

/// The trait for homomorphic polynomial commitment or polynomial.
pub trait HomomorphicPolyComElem:
    ToBytes + Clone + Sync + Send + Default + serde::Serialize + serde::de::DeserializeOwned
{
    /// This is the scalar field of the polynomial.
    type Scalar;

    /// Get base (generator) of the group.
    fn get_base() -> Self;

    /// Get identity of the group.
    fn get_identity() -> Self;

    /// Add the underlying polynomials.
    fn add(&self, other: &Self) -> Self;

    /// Add assign the underlying polynomials.
    fn add_assign(&mut self, other: &Self);

    /// Subtract the underlying polynomials.
    fn sub(&self, other: &Self) -> Self;

    /// Subtract assign the underlying polynomials.
    fn sub_assign(&mut self, other: &Self);

    /// Multiply underlying polynomial by scalar `scalar` represented
    /// in least significant byte first.
    fn mul(&self, scalar: &Self::Scalar) -> Self;

    /// Multiply underlying polynomial by scalar `scalar`.
    fn mul_assign(&mut self, scalar: &Self::Scalar);
}

/// Trait for polynomial commitment scheme.
pub trait PolyComScheme: Sized + Eq + PartialEq + Clone {
    /// Type of prime field.
    type Field: PrimeField + Debug + Sync + Send;

    /// Type of commitment produces, need to implement `HomomorphicPolyComElem`.
    type Commitment: HomomorphicPolyComElem<Scalar = Self::Field>
        + Debug
        + Default
        + PartialEq
        + Eq
        + Clone
        + Sync;

    /// Return maximal supported degree
    fn max_degree(&self) -> usize;

    /// Commit to the polynomial, commitment is binding.
    fn commit(&self, polynomial: &FpPolynomial<Self::Field>) -> KZGResult<Self::Commitment>;

    /// Evaluate the polynomial using the commitment opening to it.
    fn eval(&self, polynomial: &FpPolynomial<Self::Field>, point: &Self::Field) -> Self::Field;

    /// Evaluate the polynomial producing a proof for it.
    fn prove(
        &self,
        polynomial: &FpPolynomial<Self::Field>,
        point: &Self::Field,
        max_degree: usize,
    ) -> KZGResult<Self::Commitment>;

    /// Verify an evaluation proof that polynomial inside commitment
    /// evaluates to `value` on input `point `.
    fn verify(
        &self,
        commitment: &Self::Commitment,
        degree: usize,
        point: &Self::Field,
        value: &Self::Field,
        proof: &Self::Commitment,
    ) -> KZGResult<()>;

    /// Apply blind factors over the vanishing part
    fn apply_blind_factors(
        &self,
        commitment: &Self::Commitment,
        blinds: &[Self::Field],
        zeroing_degree: usize,
    ) -> Self::Commitment;

    /// Shrink this to only for verifier use.
    fn shrink_to_verifier_only(&self) -> KZGResult<Self>;
}

// #[cfg(test)]
// #[allow(non_snake_case)]
// mod test {
//     use super::{KZGCommitmentScheme, PolyComScheme};
//     use crate::primitives::poly::FpPolynomial;
//     use ark_bn254::Fr;
//     use ark_ff::{One, Zero};
//     use ark_std::test_rng;
//     use ark_std::{ops::*, UniformRand};

//     #[test]
//     fn test_pcs_eval() {
//         let mut prng = test_rng();
//         let zero = Fr::zero();
//         let one = Fr::one();
//         let two = one.add(&one);

//         let poly = FpPolynomial::from_zeroes(&[zero, one, two]);
//         let degree = poly.degree();
//         let pcs = KZGCommitmentScheme::new(degree, &mut prng);
//         let com = pcs.commit(&poly).unwrap();
//         let point = Fr::rand(&mut prng);
//         let proof = pcs.prove(&poly, &point, degree).unwrap();
//         let eval = pcs.eval(&poly, &point);
//         assert!(pcs.verify(&com, degree, &point, &eval, &proof).is_ok());
//     }
// }
