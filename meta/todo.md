# Basic SGF Visualizer

- [ ] Hello world
- [ ] Read [SGF FF[4]](https://www.red-bean.com/sgf/index.html)
- [ ] Read file from disk
- [ ] Parser: Only down to property indentifiers and values, treat all properties as unknown except simple stone placment
- [ ] Engine: Play out (root) game tree and calucate game state, stones only
- [ ] Visualizer: Again, stones only
- [ ] Tests: Add test for basic properties
- [ ] Use Ratatui
- [ ] Parser: Parse all [general](https://www.red-bean.com/sgf/properties.html) and [go specific (GM[1])](https://www.red-bean.com/sgf/go.html#properties) properties
- [ ] Engine: Handle all move types, include `Info` (etc.) data too?
- [ ] Visualizer: visualize all `Move`, `Setup`, `Root` and `Info` properties (?), as well as everything needed for end-of-game annotations
- [ ] Tests: Test!

# Basic OGS Client

- [ ] Go over [OGS API Docs](https://online-go.com/api-docs/)
- [ ] Basic client: Track a game (by ID), show game state and visualize board
- [ ] User input: Create input window below board
- [ ] User input: Parse user input, allow just moves, and make them
- [ ] Show more info about the game
- [ ] Allow moves, passes, and resignations
- [ ] Support listing and choosing games from games the user participates in
- [ ] Support creating games

# Apt Pkg

- [ ] Package as an `apt` package
- [ ] Upload to a local repo and manually test

# Advanced Move-Making

- [ ] Allow making moves with arrow keys (and numpad for star points?)

# Chat

- [ ] Show chat
- [ ] Allow to write messages
- [ ] Allow normal, malkovich, and personal settings

# Variations

- [ ] Parse variations
- [ ] Visualize tree structure
- [ ] Enable traversing tree with arrow keys

