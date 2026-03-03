# Architecture

## Game Manager

## SGF

### Engine (`Sgf` -> `Board`)

The `Engine` takes an `Sgf` from the parses it (using `sgf-parse`) and plays out the game, to produce a `Board`, with stones and annotations.

### Visualizer (`Board` -> Art)

The `Visualizer` takes a `Board`, and draws out a visualization of it - a board art. 

## `RestApi` and `RealtimeApi`

