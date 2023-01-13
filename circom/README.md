# Circom Example

## Dependencies

* [`circom`](https://github.com/iden3/circom)
* [`node`](https://nodejs.org/en/)
* [`snarkjs`](https://github.com/iden3/snarkjs)

## Instructions

The end-to-end target is `make verify`. See the Makefile for steps.

## Files to edit

* `sudoku.circom`: the template
* `sudoku.input.json`: the prover's input
  * the verifier's input, `sudoku.inst.json`, is computed from it
