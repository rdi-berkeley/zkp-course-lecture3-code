pragma circom 2.0.0;

template Multiplier2 () {  
   // Declaration of signals.  
   signal input in0;  
   signal input in1;  
   signal output out;

   // Constraints.  
   in1 === in0 * in0;  
   out <== in1;
}


template NonEqual(){
    signal input in0;
    signal input in1;
    signal inv;
    inv <-- 1/ (in0 - in1);
    inv*(in0 - in1) === 1;
}

template Distinct(n) {
    signal input in[n];
    component nonEqual[n][n];
    for(var i = 0; i < n; i++){
        for(var j = 0; j < i; j++){
            nonEqual[i][j] = NonEqual();
            nonEqual[i][j].in0 <== in[i];
            nonEqual[i][j].in1 <== in[j];
        }
    }
}

template Sudoku(n) {
    // board is a 2D array: indices are (row_i, col_i)
    signal input board[n][n];
    signal output firstColumn[n];

    component distinct[n];

    for (var row_i = 0; row_i < n; row_i++) {
        firstColumn[row_i] <== board[row_i][0];
    }

    for (var row_i = 0; row_i < n; row_i++) {
        for (var col_i = 0; col_i < n; col_i++) {
            if (row_i == 0) {
                distinct[col_i] = Distinct(n);
            }
            distinct[col_i].in[row_i] <== board[row_i][col_i];
        }
    }
}

component main = Sudoku(2);

