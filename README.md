Fractory. A game you have never seen before...

...mostly because it hasn't been made yet.

**Most activity is in `src/logic/` and a lot of pretty pictures are under `assets/concepts`**

TODO:

```
document TilePos::{pop, push}
consider pop_unchecked

reorganize*

create linear factory proof of concept
  separate crate
  array of items (internally just IDs)
  list of activations
  each item has an associated behavior
    list of moves it wants to do
    list of tiles it wants to activate
  perform the whole simulation starting with a random item/activation/behavior combo
  multiple tests with predefined items/activations/behaviors
  interactive TUI binary in a separate crate, using the linear factory as a dependency

create fractal version
  resolve movelist collisions in fractal space
  resolve overlapping small+large activations in fractal space
  try not to get a migraine
  TUI

create main game behavior
  save data
  biomes, labs, fragments, file tree organization and loading
  menu..?


*REORGANIZING: (not final, some decisions are very awkward and will require trial and error)
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
Go to the dev branch.
