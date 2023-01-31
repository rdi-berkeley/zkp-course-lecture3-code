use ark_ff::PrimeField;
use ark_r1cs_std::{prelude::{Boolean, EqGadget}, R1CSVar, uint8::UInt8, ToBitsGadget};
use ark_relations::r1cs::SynthesisError;

pub trait CmpGadget<ConstraintF: PrimeField>: R1CSVar<ConstraintF> + EqGadget<ConstraintF> {
    #[inline]
    fn is_geq(&self, other: &Self) -> Result<Boolean<ConstraintF>, SynthesisError> {
        // self >= other => self == other || self > other
        //               => !(self < other)
        self.is_lt(other).map(|b| b.not())
    }

    #[inline]
    fn is_leq(&self, other: &Self) -> Result<Boolean<ConstraintF>, SynthesisError> {
        // self <= other => self == other || self < other
        //               => self == other || other > self
        //               => self >= other
        other.is_geq(self)
    }

    #[inline]
    fn is_gt(&self, other: &Self) -> Result<Boolean<ConstraintF>, SynthesisError> {
        // self > other => !(self == other  || self < other)
        //              => !(self <= other)
        self.is_leq(other).map(|b| b.not())
    }

    fn is_lt(&self, other: &Self) -> Result<Boolean<ConstraintF>, SynthesisError>; 
}

impl<ConstraintF: PrimeField> CmpGadget<ConstraintF> for UInt8<ConstraintF> {
    fn is_lt(&self, other: &Self) -> Result<Boolean<ConstraintF>, SynthesisError> {
        // Determine the variable mode.
        if self.is_constant() && other.is_constant() {
            let self_value = self.value().unwrap();
            let other_value = other.value().unwrap();
            let result = Boolean::constant(self_value < other_value);
            Ok(result)
        } else {
            let diff_bits = self.xor(other)?.to_bits_be()?.into_iter();
            let mut result = Boolean::FALSE;
            let mut a_and_b_equal_so_far = Boolean::TRUE;
            let a_bits = self.to_bits_be()?;
            let b_bits = other.to_bits_be()?;
            for ((a_and_b_are_unequal, a), b) in diff_bits.zip(a_bits).zip(b_bits) {
                let a_is_lt_b = a.not().and(&b)?;
                let a_and_b_are_equal = a_and_b_are_unequal.not();
                result = result.or(&a_is_lt_b.and(&a_and_b_equal_so_far)?)?;
                a_and_b_equal_so_far = a_and_b_equal_so_far.and(&a_and_b_are_equal)?;
            }
            Ok(result)
        }
    }
}

#[cfg(test)]
mod test {
    use ark_r1cs_std::{prelude::{AllocationMode, AllocVar, Boolean, EqGadget}, uint8::UInt8};
    use ark_relations::r1cs::{ConstraintSystem, SynthesisMode};
    use ark_bls12_381::Fr as Fp;
    use itertools::Itertools;

    use crate::cmp::CmpGadget;

    #[test]
    fn test_comparison_for_u8() {
        let modes = [AllocationMode::Constant, AllocationMode::Input, AllocationMode::Witness];
        for (a, a_mode) in (0..=u8::MAX).cartesian_product(modes) {
            for (b, b_mode) in (0..=u8::MAX).cartesian_product(modes) {
                let cs = ConstraintSystem::<Fp>::new_ref();
                cs.set_mode(SynthesisMode::Prove { construct_matrices: true });
                let a_var = UInt8::new_variable(cs.clone(), || Ok(a), a_mode).unwrap();
                let b_var = UInt8::new_variable(cs.clone(), || Ok(b), b_mode).unwrap();
                if a < b {
                    a_var.is_lt(&b_var).unwrap()
                        .enforce_equal(&Boolean::TRUE).unwrap();
                    a_var.is_leq(&b_var).unwrap().enforce_equal(&Boolean::TRUE).unwrap();
                    a_var.is_gt(&b_var).unwrap().enforce_equal(&Boolean::FALSE).unwrap();
                    a_var.is_geq(&b_var).unwrap().enforce_equal(&Boolean::FALSE).unwrap();
                } else if a == b {
                    a_var.is_lt(&b_var).unwrap().enforce_equal(&Boolean::FALSE).unwrap();
                    a_var.is_leq(&b_var).unwrap().enforce_equal(&Boolean::TRUE).unwrap();
                    a_var.is_gt(&b_var).unwrap().enforce_equal(&Boolean::FALSE).unwrap();
                    a_var.is_geq(&b_var).unwrap().enforce_equal(&Boolean::TRUE).unwrap();
                } else {
                    a_var.is_lt(&b_var).unwrap().enforce_equal(&Boolean::FALSE).unwrap();
                    a_var.is_leq(&b_var).unwrap().enforce_equal(&Boolean::FALSE).unwrap();
                    a_var.is_gt(&b_var).unwrap().enforce_equal(&Boolean::TRUE).unwrap();
                    a_var.is_geq(&b_var).unwrap().enforce_equal(&Boolean::TRUE).unwrap();
                }
                assert!(cs.is_satisfied().unwrap(), "a: {a}, b: {b}");
            }
        }
    }
}
