# Architecture

## Game Manager

## SGF

### Parser

The SGF Parser is the component that parses SGFs from the server and outputs them in a format (`Sgf`) that downstream components (the `Engine`) can easily work with.

The parser is a hand-written recursive decent parser in a lexer/parser architecture.

see:

```
src/
 sgf/
  lexer/
  parser/
```

### Engine

The `Engine` takes an `Sgf` from the parser and plays out the game, to produce a `Board`, with stone and annotation information.

### Visualizer

The `Visualizer` takes a `Board`, and draws out a visualization of it - a board art. 

## `RestApi` and `RealtimeApi`

