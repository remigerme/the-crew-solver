# The Crew: Mission Deep Sea solver

This is a solver for the board game [The Crew: Mission Deep Sea](https://store.thamesandkosmos.com/products/the-crew-mission-deep-sea). For more information on the game and a general overview of the solver, check [the blog article](https://blog.remigerme.xyz/cs/the-crew) accompanying this project.

> [!IMPORTANT]
> **Call for contributors**: I am currently looking for contributors for a (static) website, check the [contribution guidelines](CONTRIBUTING.md) if you are interested.

## The solver

You give to the solver: a game status (that is: a list of players with their cards in hand and their tasks, as well as tricks they previously won + the current trick). The game status can describe a fresh new game, or an in-progress game.

The solver returns: the first solution it found, if any, or an error to signal the absence of solution otherwise.

## Todo

- [x] implement all 96 tasks
- [ ] web assembly integration
- [ ] web front end
