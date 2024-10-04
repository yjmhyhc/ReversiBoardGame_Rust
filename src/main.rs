use std::io;
use std::io::Write;

fn main() {
    //initialize the board and the pieces in the middle
    let mut board: [[i32; 8]; 8] = [[0; 8]; 8];
    board[3][3] = 2;board[3][4] = 1;board[4][3] = 1;board[4][4] = 2;

    //set the player, the default player holds black
    let mut player =String::from("B");

    //players play in turn
    loop{
        //print the board
        print_the_board(&mut board);
        //flush the standard output
        io::stdout().flush().expect("Failed to flush stdout.");

        //receive the input
        println!("Enter move for colour {} (RowCol):", player);
        let mut player_move = String::new();
        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");

        //remove the newline character at the end
        player_move = player_move.trim_end().to_string();

        //if the move is invalid, ask the player to try again
        if !check_the_move(&board, &player_move, &player) {
            println!("Invalid move. Try again.");
            continue;
        }
        
        //if the move is valid, execute the move
        execute_the_move(&mut board, &player_move, &player);

        //check if the other player has a valid move
        //identify the other player
        let the_other_player = if player == "B" {
            "W"
        } else {
            "B"
        };

        //if the other player do have a valid move, switch player
        //else continue to check if current player has a valid move
        if check_any_possible_move(&board, &the_other_player) {
            player = String::from(the_other_player);
        }else {
            //current player has no possible move
            if !check_any_possible_move(&board, &player){
                //calculate and announce the result of the game, then break
                println!("{}",calculate_the_result(&board));
                break;
            }
            //if current player has a valid move, go to the next round without switching player
        }
    }
}

//the function to print the board, converting numbers to alphabets
fn print_the_board(board: &mut[[i32; 8]; 8]){
    println!("  abcdefgh");
    let mut row_count: u8 = 0;
    for row in board.iter() {
        print!("{} ", (b'a' + &row_count) as char);
        row_count+=1;
        for val in row.iter() {
            if *val==0 {print!(".");}
            if *val==1 {print!("B");}
            if *val==2 {print!("W");}
        }
        println!();
    }
}

//check if the move is valid
fn check_the_move(board: &[[i32; 8]; 8], player_move: &str, player: &str) -> bool {
    //check the format of the input
    if player_move.chars().count()!=2 || !player_move.chars().all(|c| c >= 'a' && c <= 'h'){
        return false
    }

    //check if the place has already been taken by another piece
    let mut row: usize = 0;
    if let Some(c) = player_move.chars().nth(0){
        row = ((c as u8) - b'a') as usize;
    }
    let mut col: usize = 0;
    if let Some(c) = player_move.chars().nth(1){
        col = ((c as u8) - b'a') as usize;
    }
    if board[row][col]!=0 {
        return false
    }

    //identify the player
    let mut player_num = 1;
    if player=="W" {
        player_num = 2;
    }

    //check and flip opponent's pieces
    //record how many pieces can be flipped in one direction
    let mut flip_count = 0;
    //we will check all directions one by one
    //right
    while col<6 && board[row][col+1]!=0 && board[row][col+1]!=player_num {
        col+=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row][col+1]!=player_num {
            col-=flip_count;
            flip_count = 0;
        }else {
            return true;
        }
    }
    //left
    while col>1 && board[row][col-1]!=0 && board[row][col-1]!=player_num {
        col-=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row][col-1]!=player_num {
            col+=flip_count;
            flip_count = 0;
        }else {
            return true;
        }
    }
    //up
    while row>1 && board[row-1][col]!=0 && board[row-1][col]!=player_num {
        row-=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row-1][col]!=player_num {
            row+=flip_count;
            flip_count = 0;
        }else {
            return true;
        }
    }
    //down
    while row<6 && board[row+1][col]!=0 && board[row+1][col]!=player_num {
        row+=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row+1][col]!=player_num {
            row-=flip_count;
            flip_count = 0;
        }else {
            return true;
        }
    }
    //right up
    while row>1 && col<6 && board[row-1][col+1]!=0 && board[row-1][col+1]!=player_num{
        row-=1;
        col+=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row-1][col+1]!=player_num {
            row+=flip_count;
            col-=flip_count;
            flip_count = 0;
        }else {
            return true;
        }
    }
    //left up
    while row>1 && col>1 && board[row-1][col-1]!=0 && board[row-1][col-1]!=player_num{
        row-=1;
        col-=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row-1][col-1]!=player_num {
            row+=flip_count;
            col+=flip_count;
            flip_count = 0;
        }else {
            return true;
        }
    }
    //left down
    while row<6 && col>1 && board[row+1][col-1]!=0 && board[row+1][col-1]!=player_num{
        row+=1;
        col-=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row+1][col-1]!=player_num {
            row-=flip_count;
            col+=flip_count;
            flip_count = 0;
        }else {
            return true;
        }
    }
    //right down
    while row<6 && col<6 && board[row+1][col+1]!=0 && board[row+1][col+1]!=player_num{
        row+=1;
        col+=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row+1][col+1]!=player_num {
            row-=flip_count;
            col-=flip_count;
            flip_count = 0;
        }else {
            return true;
        }
    }
    return false;
}

//execute the move
fn execute_the_move(board: &mut[[i32; 8]; 8], player_move: &str, player: &str){
    //find the position of this move
    let mut row: usize = 0;
    if let Some(c) = player_move.chars().nth(0){
        row = ((c as u8) - b'a') as usize;
    }
    let mut col: usize = 0;
    if let Some(c) = player_move.chars().nth(1){
        col = ((c as u8) - b'a') as usize;
    }

    //identify the player
    let mut player_num = 1;
    if player=="W" {
        player_num = 2;
    }

    //check and flip opponent's pieces
    //record how many pieces can be flipped in one direction
    let mut flip_count = 0;
    //we will check all directions one by one
    //right
    while col<6 && board[row][col+1]!=0 && board[row][col+1]!=player_num {
        col+=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row][col+1]!=player_num {
            col-=flip_count;
            flip_count = 0;
        }else {
            while flip_count>=1 {
                flip_count-=1;
                board[row][col] = player_num;
                col-=1;
            }
        }
    }
    //left
    while col>1 && board[row][col-1]!=0 && board[row][col-1]!=player_num {
        col-=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row][col-1]!=player_num {
            col+=flip_count;
            flip_count = 0;
        }else {
            while flip_count>=1{
                flip_count-=1;
                board[row][col] = player_num;
                col+=1;
            }
        }
    }
    //up
    while row>1 && board[row-1][col]!=0 && board[row-1][col]!=player_num {
        row-=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row-1][col]!=player_num {
            row+=flip_count;
            flip_count = 0;
        }else {
            while flip_count>=1{
                flip_count-=1;
                board[row][col] = player_num;
                row+=1;
            }
        }
    }
    //down
    while row<6 && board[row+1][col]!=0 && board[row+1][col]!=player_num {
        row+=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row+1][col]!=player_num {
            row-=flip_count;
            flip_count = 0;
        }else {
            while flip_count>=1{
                flip_count-=1;
                board[row][col] = player_num;
                row-=1;
            }
        }
    }
    //right up
    while row>1 && col<6 && board[row-1][col+1]!=0 && board[row-1][col+1]!=player_num{
        row-=1;
        col+=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row-1][col+1]!=player_num {
            row+=flip_count;
            col-=flip_count;
            flip_count = 0;
        }else {
            while flip_count>=1{
                flip_count-=1;
                board[row][col] = player_num;
                row+=1;
                col-=1;
            }
        }
    }
    //left up
    while row>1 && col>1 && board[row-1][col-1]!=0 && board[row-1][col-1]!=player_num{
        row-=1;
        col-=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row-1][col-1]!=player_num {
            row+=flip_count;
            col+=flip_count;
            flip_count = 0;
        }else {
            while flip_count>=1{
                flip_count-=1;
                board[row][col] = player_num;
                row+=1;
                col+=1;
            }
        }
    }
    //left down
    while row<6 && col>1 && board[row+1][col-1]!=0 && board[row+1][col-1]!=player_num{
        row+=1;
        col-=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row+1][col-1]!=player_num {
            row-=flip_count;
            col+=flip_count;
            flip_count = 0;
        }else {
            while flip_count>=1{
                flip_count-=1;
                board[row][col] = player_num;
                row-=1;
                col+=1;
            }
        }
    }
    //right down
    while row<6 && col<6 && board[row+1][col+1]!=0 && board[row+1][col+1]!=player_num{
        row+=1;
        col+=1;
        flip_count+=1;
    }
    if flip_count>0 {
        if board[row+1][col+1]!=player_num {
            row-=flip_count;
            col-=flip_count;
            flip_count = 0;
        }else {
            while flip_count>=1{
                flip_count-=1;
                board[row][col] = player_num;
                row-=1;
                col-=1;
            }
        }
    }
    board[row][col] = player_num
}

//check if the player has any possible move
fn check_any_possible_move(board: &[[i32; 8]; 8], player: &str) -> bool {
    //record whether there is at least one possible valid move
    let mut flag = false;
    for letter1 in 'a'..='h'{
        if flag {break;}
        for letter2 in 'a'..='h'{
            let mut one_possible_move = String::new();
            one_possible_move.push(letter1);
            one_possible_move.push(letter2);
            if check_the_move(&board, &one_possible_move, &player){
                flag = true;
                break;
            }
        }
    }
    return flag
}

fn calculate_the_result(board: &[[i32; 8]; 8]) -> String{
    //record the score of two players
    let mut score_player1 = 0;let mut score_player2 = 0;

    //iterate through the board to accumulate the score
    for row in board.iter() {
        for val in row.iter() {
            if *val == 1{
                score_player1+=1;
            }else if *val==2 {
                score_player2+=1;
            }
        }
    }

    //identify the winner and how much he/she wins by
    let mut result = String::new();
    let mut difference = 0;
    if score_player1>score_player2 {
        difference = score_player1-score_player2;
        result = String::from("Black wins by ");
        result = result + &difference.to_string() + " points!";
    }else if score_player2>score_player1 {
        difference = score_player2-score_player1;
        result = String::from("White wins by ");
        result = result + &difference.to_string() + " points!";
    }else {
        result = String::from("Draw!")
    }
    return result
}
