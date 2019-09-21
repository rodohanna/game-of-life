# Conway's Game of Life in Rust

I've been wanting to learn Rust for a while now, so I decided that
it would be cool to use Rust to implement [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

I feel like I learned a lot, so I'll probably do this again with
other languages I want to dabble in.

# Full Disclosure

- I wrote an [rle parser](http://www.conwaylife.com/wiki/Run_Length_Encoded) that has, so far, proven to be robust enough to handle any pattern file I've thrown at it; however, it's possible there are bugs that may cause it to crash or render a pattern incorrectly. If you do find one that doesn't work, please file an issue! I'd love to fix it.
- Many of the patterns on the Game of Life Wiki define a grid size, but have the assumption that evolutions can spread to regions that go beyond the grid size specified. In those cases, patterns will always be drawn correctly on first render; however, it is likely that my implementation will do something different than the example gif.
  - I plan to resolve this by moving to an SDL2 renderer so I can handle "infinite" grid evolutions; This should also allow me to simulate evolutions _much_ faster.
- The library I'm using to make HTTP requests is **way** too heavy for my needs which causes the build time to be **way** longer than it should be. It's got like 50 dependencies (wtf?). I'm planning on swapping it out for something much more lightweight.

### Installation

Prerequisites:

- [`rustup`](https://www.rust-lang.org/tools/install)
  - This installs all of the tools you will need to run this. Nice.

Clone and build the project:

```sh
$ git clone git@github.com:RobertDHanna/game-of-life.git
$ cd game-of-life
$ cargo build
```

Run the program:

```sh
$ cargo run
```

You should see a [duodecapole](http://www.conwaylife.com/wiki/Duodecapole) oscillator. Cool.

![Duodecapole](https://media.giphy.com/media/UpJXoUTZWRIju2Jy7W/giphy.gif)

Additionally, you can supply a remote [rle](http://www.conwaylife.com/wiki/Run_Length_Encoded) file as a CLI argument.

You can find pattern files by going to [the Conway Game of Life Wiki](http://www.conwaylife.com/wiki/Category:Patterns) and choosing a pattern.

Example:

- Go to http://www.conwaylife.com/wiki/Category:Patterns
- Find and click on [4-8-12 diamond](http://www.conwaylife.com/wiki/4-8-12_diamond)
- On the right-hand side of the page under `Pattern Files` click `show`.
- Find and click on [4812diamond.rle](http://www.conwaylife.com/patterns/4812diamond.rle)
- Copy the link.
- Run the program with the link supplied as an argument like below:

```sh
$ cargo run http://www.conwaylife.com/patterns/4812diamond.rle
```

You should see something gnarly like this:

![4812diamond](https://media.giphy.com/media/S6MhJJcRPDE39HpIh1/giphy.gif)

Thanks for stopping by!
