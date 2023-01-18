SHELL = zsh

circom = sudoku.circom
r1cs = sudoku.r1cs
wasm = sudoku_js/sudoku.wasm
wit_gen = sudoku_js/generate_witness.js
compile_outputs = sudoku_js/witness_calculator.js $(r1cs) $(wasm) $(wit_gen)
pk = sudoku.pk
vk = sudoku.vk
ptau = sudoku.ptau
keys = $(pk) $(vk)
p_input = sudoku.input.json
wit = sudoku.wtns
pf = sudoku.pf.json
inst = sudoku.inst.json
prove_outputs = $(pf) $(inst)

all: verify

$(compile_outputs): $(circom)
	circom $< --r1cs --wasm

$(ptau):
	snarkjs powersoftau new bn128 12 tmp.ptau
	snarkjs powersoftau prepare phase2 tmp.ptau $(ptau)
	rm tmp.ptau

$(keys): $(ptau) $(r1cs)
	snarkjs groth16 setup $(r1cs) $(ptau) $(pk)
	snarkjs zkey export verificationkey $(pk) $(vk)

$(wit): $(p_input) $(wasm) $(wit_gen)
	node $(wit_gen) $(wasm) $(p_input) $@

$(prove_outputs): $(wit) $(pk)
	snarkjs groth16 prove $(pk) $(wit) $(pf) $(inst)

.PHONY = verify clean

verify: $(pf) $(inst) $(vk)
	snarkjs groth16 verify $(vk) $(inst) $(pf)

clean:
	rm -f $(compile_outputs) $(ptau) $(keys) $(wit) $(prove_outputs)
	rmdir sudoku_js

