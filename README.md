# Number Guessing Game 🎲

A command-line based number guessing game written in Rust. Players enter their guesses, and the player closest to the randomly generated lucky number (between 1 and 100) wins!

## 🚀 Features
* Built with Rust.
* Simple CLI interface.
* Calculates absolute difference to find the closest guess.

## 🛠️ Prerequisites
* [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

## 🎮 How to Play
1. Open your terminal in the project directory.
2. Run the game using Cargo, providing numbers (guesses) as arguments:

```bash
cargo run -- <Player1> <Player2> <Player3> <Player4>
```

**Example Output:**
```bash
$ cargo run -- 15 42 77 90
Lucky number is: 63
Congratulations! Player 3 wins 🎉
Difference: 14
```

## 📜 License
This project is licensed under the MIT License.
