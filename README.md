Fractory. A game you have never seen before...

...mostly because it hasn't been made yet.

**Most activity is in `common/ and poc-fractal/` and a lot of pretty pictures are under `assets/concepts`**
The main branch is updated for major milestones, while the dev branch is updated constantly and is not guaranteed to compile.

- Esc to quit
- mouse over the fractal to explore the subdivisions
- WASD/right mouse drag to move the camera around
- Scroll to zoom, automatically expanding the fractal as you go (going too far crashes the program)
- Shift+Scroll to change recursion depth without zooming
- Ctrl+Scroll to zoom without changing recursion depth
- Q/E to rotate the camera (do tell me if you would like to reverse this direction)
- Space to zoom in
- Shift+Space to zoom out
- F to flip the camera horizontally
- -/+ to increase/decrease the depth of fractal recursion, capped to [0, 6]
- Tab to switch modes (Edit/Act/View)
  * `Edit` mode: Clicking a tile cycles it between 0, 1, 2. specifically, `tile.id = if tile.id < 2 { tile.id + 1 } else { 0 }`
  * `Act` mode: Activates a clicked tile, priming it for action in the next tick
  * `View` mode: does nothing, just lets you explore the fractal in its solid arrangement.
- Enter to simulate one tick of the simulation
  * Orange and Yellow tiles will try to swap between the tile on their right and the tile on their lower left:
```
  *activating an orange or yellow T will try to swap a and b.
       /\    /\
      /T \  /a \
     /____\/____\
    /\
   /b \
  /____\

*orientation matters. upside-down tiles will of course do an upside-down version of this.
*if a tile tries to move to 2 different positions, both moves are cancelled and nothing happens.
*if 2 tiles try to move to the same position, both moves are cancelled and nothing happens.
*if a tile tries to move into an occupied position, the move is cancelled and nothing happens.

*currently, symmetry is being broken. tile orientations will be fixed later.
```

TODO:
- test the fractal triangle rendering in poc-fractal /
- add interactivity, so you can.. /
- ..test the algorithms for the fractal manipulation /
- common::sim::logic::factory::Fractory /
    * should contain all the activated tiles /
    * should know how to simulate /
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
