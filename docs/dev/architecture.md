# Architecture

## Game Manager

## SGF

### Parser (Text -> `Sgf`)

The SGF Parser is the component that parses SGFs from the server and outputs them in a format (`Sgf`) that downstream components (the `Engine`) can easily work with.

The parser is written using [nom](https://docs.rs/nom/latest/nom) - a parser combinator library.

### Engine (`Sgf` -> `Board`)

The `Engine` takes an `Sgf` from the parser and plays out the game, to produce a `Board`, with stones and annotations.

### Visualizer (`Board` -> Art)

The `Visualizer` takes a `Board`, and draws out a visualization of it - a board art. 

## `RestApi` and `RealtimeApi`
