# Peg-Solitare game solver

The objective of peg solitaire game is to leave onlyn one piece on the board.
To do so you need to use a piece to skip over another piece and land on an empty space, then the skipped piece gets removed.

This program here implments the rules of the game so that a solver could be written. the strategy of the solver is simple to traverse the move graph 
until it reaches a solution state.


# Porfiling

Part of the idea for this project is to learn to profile a Rust program and with that make decisions on how to optimize the program.

```
# installing tool for profiling 
cargo install flamegraph

# Runnning tool
flamegraph --root -- target/release/peg-solitaire
```

---

Initial implementation measurements:
```
1000 runs of solving full board:
avg time 7.78ms
commit of measurement: a6d3467
```

Possible points of improvement:
- keep a hashmap of already searched positions. (maybe we can avoid researching)
- find_pieces function scans full board for pieces, maybe keep a list or hashset of piece positions and updated it on after each move.
- creating vectors for moves: we know the max size is 31 moves, we could initialize with capacity set.

Profiler shows lots time on `RawVec::reserve_for_push` and `raw_vec::finish_grow`
