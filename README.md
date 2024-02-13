# Implementation of AI agent for my own Tetris implementation

This project is a main part of my engineering thesis, and contains implementation of a linear agent model, which is optimized using [genetic algorithm](https://www.geeksforgeeks.org/genetic-algorithms/).
Agent depends on several heuristics, and finds their weights through optimization.

This is currently a work in progress and - for the time being - is missing several functionalities - notably saving a trained model. Currently there is a handcoded solution in graphical showcase, which was found during training.

## How to run
To show previously trained model playing the game:
```bash
cargo run --release --bin=play
```

To train the model:
```bash
cargo run --release --bin=train -- -n=100 --max-drops=20000
```

