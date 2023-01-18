zokrates compile --input root.zok
zokrates setup
zokrates compute-witness -a 1 0 0 2 1 2 1 2
zokrates generate-proof
zokrates verify