use ark_ff::PrimeField;
use ark_r1cs_std::{
    prelude::{Boolean, EqGadget, AllocVar},
    uint8::UInt8
};
use ark_relations::r1cs::{SynthesisError, ConstraintSystem};
use cmp::CmpGadget;

mod cmp;
mod alloc;

pub struct Puzzle<const N: usize, ConstraintF: PrimeField>([[UInt8<ConstraintF>; N]; N]);
pub struct Solution<const N: usize, ConstraintF: PrimeField>([[UInt8<ConstraintF>; N]; N]);

fn check_rows<const N: usize, ConstraintF: PrimeField>(
    solution: &Solution<N, ConstraintF>,
) -> Result<(), SynthesisError> {
    for row in &solution.0 {
        for (j, cell) in row.iter().enumerate() {
            for prior_cell in &row[0..j] {
                cell.is_neq(&prior_cell)?
                    .enforce_equal(&Boolean::TRUE)?;
            }
        }
    }
    Ok(())
}

fn check_puzzle_matches_solution<const N: usize, ConstraintF: PrimeField>(
    puzzle: &Puzzle<N, ConstraintF>,
    solution: &Solution<N, ConstraintF>,
) -> Result<(), SynthesisError> {
    for (p_row, s_row) in puzzle.0.iter().zip(&solution.0) {
        for (p, s) in p_row.iter().zip(s_row) {
            // Ensure that the solution `s` is in the range [1, N]
            s.is_leq(&UInt8::constant(N as u8))?
                .and(&s.is_geq(&UInt8::constant(1))?)?
                .enforce_equal(&Boolean::TRUE)?;

            // Ensure that either the puzzle slot is 0, or that
            // the slot matches equivalent slot in the solution
            (p.is_eq(s)?.or(&p.is_eq(&UInt8::constant(0))?)?)
                .enforce_equal(&Boolean::TRUE)?;
        }
    }
    Ok(())
}

fn check_helper<const N: usize, ConstraintF: PrimeField>(
    puzzle: &[[u8; N]; N],
    solution: &[[u8; N]; N],
) {
    let cs = ConstraintSystem::<ConstraintF>::new_ref();
    let puzzle_var = Puzzle::new_input(cs.clone(), || Ok(puzzle)).unwrap();
    let solution_var = Solution::new_witness(cs.clone(), || Ok(solution)).unwrap();
    check_puzzle_matches_solution(&puzzle_var, &solution_var).unwrap();
    check_rows(&solution_var).unwrap();
    assert!(cs.is_satisfied().unwrap());
}

fn main() {
    use ark_bls12_381::Fq as F;
    // Check that it accepts a valid solution.
    let puzzle = [
        [1, 0],
        [0, 2],
    ];
    let solution = [
        [1, 2],
        [1, 2],
    ];
    check_helper::<2, F>(&puzzle, &solution);

    // Check that it rejects a solution with a repeated number in a row.
    let puzzle = [
        [1, 0],
        [0, 2],
    ];
    let solution = [
        [1, 0],
        [1, 2],
    ];
    check_helper::<2, F>(&puzzle, &solution);
}
