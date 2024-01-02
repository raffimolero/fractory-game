Fractory. A game you have never seen before...

...mostly because it hasn't been made yet.

**Most activity is in `common/` and `poc-bevy/` and a lot of pretty pictures are under `assets/concepts/`**
The `main` branch is updated for major milestones, the `dev` branch is for prototyping mechanics, and the `bevy` branch is for the real game.

There is no way to simulate the game in this branch. That would be in the `dev` branch.

Demo:

https://github.com/raffimolero/fractory-game/assets/49224759/9f3a2161-c727-44c8-9fd1-822502fe8e3c

- Esc: quit
- WASD or Right Click + Drag: move
- Q/E: rotate
- F: flip
- Space or Scroll up: zoom in
- LShift or Scroll down: zoom out
- LCtrl + Zoom: change cursor expansion depth
- LAlt + Zoom: change background expansion depth

```
    tile behavior rules:
*orientation matters. upside-down tiles will of course do their thing from their perspective.
*if a tile tries to move to 2 different positions, both moves are cancelled and nothing happens.
*if 2 tiles try to move to the same position, both moves are cancelled and nothing happens.
*if a tile tries to move into an occupied position, the move is cancelled and nothing happens.
```

**[ Further information may be outdated ]**

TODO:
- test the fractal triangle rendering in poc-fractal (done)
- add interactivity, so you can.. (done)
- ..test the algorithms for the fractal manipulation (done)
- common::sim::logic::factory::Fractory (done)
    * should contain all the activated tiles (done)
    * should know how to simulate (done)
    * should be able to hook into a UI to send transition info (what animations should play per tile)
- Fragment

```
create fractal version
  resolve movelist collisions in fractal space - ok
  resolve overlapping small+large activations in fractal space - TO TEST FURTHER
  try not to get a migraine - IN PROGRESS
  TUI - SKIP TO ERGOQUAD_2D GUI

create main game behavior
  save data
  biomes, labs, fragments, file tree organization and loading
  menu..?


File structure:
  common/       crate, holds application logic
    src/
      api/          holds a bunch of traits that downstream users have to implement, "how to"
                    once implemented, it's all just GameStruct::run()... maybe?
        io/           how to access filesystem, how to internet stuff, etc
        ui/           traits for how elements should render?
        data/        defines all the data types loaded by sim::io
      sim/          simulation stuff
        logic/        black box code that just magically gives output
        io/           uses common::api to do all sorts of io stuff
  terminal/     crate, imports common, provides a terminal ui to test logic's API
  desktop/      crate, imports common, provides a native ui
  web/          crate, imports common, will be difficult to implement because filesystem
  poc-linear/   crate, proof of concept for an array based game instead of fractals
    src/
      common/
        <common/src/*, but as a module instead of a crate>
      terminal/
      desktop/
  poc-fractal/  crate, trial and error
    examples/     small scale t&e
    src/          medium scale t&e
      <poc-linear/src/*>
  .git          large scale t&e
```
=======
