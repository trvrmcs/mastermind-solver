# Solver for Mastermind game.

https://en.wikipedia.org/wiki/Mastermind_(board_game)

Basically a re-implementation of the ideas introduced here: 
https://www.goranssongaspar.com/mastermind

Solver works by on each round choosing the guess that has the lowest entropy, i.e. the highest information content.

The game starts with 1296 possible states (six possible colors in each of four holes: 6^4 = 1296)

This is equivalent to there being approximately 10.33985 unknown bits of information. (2 ^ 10.33985 = 1296)

The goal is to reduce the possible states to one, i.e. 0 bits of unknown information. (2^0 = 1)

It turns out with optimal play we can usually extract just over 3 bits per guess, so this solver usually takes 4 guesses
to find the solution.

## Usage

`cargo run --release`


The program will create a board with a randomly chosen secret, and then the solver 
will iteratively propose a guess until it is certain it has deduced the secret.

It runs very slowly in debug mode; but in release mode will find a solution in a few hundred milliseconds.



