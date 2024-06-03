# Rust Tic Tac Toe

This is a simple Tic Tac Toe game implemented in Rust. It allows two players to take turns placing their marks on a 3x3 grid until one player wins or the game ends in a draw.

## Features

- **Interactive Gameplay:** Players can input their moves via the command line interface.
- **Basic Win Detection:** The game automatically detects when a player has won by getting three of their marks in a row, column, or diagonal.
- **Error Handling:** The program handles input errors gracefully, prompting users to input valid moves.
- **Modular Design:** The code is divided into separate modules (`board` and `tic_tac_toe`) for better organization and reusability.
- **Clear Display:** The current state of the game board is displayed after each move, making it easy for players to see the progress of the game.
- **Game Over Detection:** Once a player wins or the game ends in a draw, the program displays a message indicating the end of the game and exits.

## How to Play

1. Clone the repository to your local machine.
2. Make sure you have Rust installed. If not, you can download it [here](https://www.rust-lang.org/tools/install).
3. Navigate to the project directory in your terminal.
4. Run the program using the command `cargo run`.
5. Follow the prompts to input your moves. Coordinates should be entered as row and column numbers separated by a comma (e.g., `0,1` for the top-center position).

## Example Gameplay

```txt
Hello, Tic Tac Toe
   |   |   
-----------
   |   |   
-----------
   |   |   
input the position separated by comma:
1,1
You typed: Position { row: 1, col: 1 }
   |   |   
-----------
   | O |   
-----------
   |   | X 

...

input the position separated by comma:
0,0
You typed: Position { row: 2, col: 1 }
 X | O  |   
-----------
   | O |   
-----------
   | O | X 
Player, O wins
Game Over!
```

## TODO

- **Handle exception when inputting a value out of range:** Implement validation to prevent users from inputting coordinates outside the range of the board.
- **Handle draw condition when all the positions are filled:** Detect when all positions on the board are filled and declare the game as a draw.
- **Nice to have: Improve the UI to be more interactive:** Explore options to enhance the user interface, such as adding colors or graphical elements to the display.

## Contribution

Contributions to this project are welcome! Feel free to open issues for bug fixes or feature requests, and submit pull requests with improvements.
