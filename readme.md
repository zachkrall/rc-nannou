# nannou sketchbook

nannou sketchbook during my Summer 1 Batch @RecurseCenter

## the workspace

The root directory is a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
which allows all of the sketches to share `nannou` as a dependency and
improves build time for new sketches.

Sketches share the same `/target` directory which is located in the root
folder

example of document heirarchy:
```bash
rc-nannou
  ├── Cargo.toml          # Defines workspace
  ├── sketch-one/         # Container for sketch
  │    ├── Cargo.toml     # Defines sketch dependencies
  │    └── src/
  │         └── main.rs   # Main sketch file
  └── target/             # Where sketches are built
```

## run a sketch
Use the following command to run & display a nannou sketch (Where
  [SKETCHNAME] is the name of a sketch directory)

```bash
cargo run -p [SKETCHNAME] --release
```

The sketch name must be included in the `Cargo.toml` file
in the root directory under "members".

example:
```toml
members = [
  "sketch-one",           # It's ok to have trailing commas
]
```

## cli

because it's kind of tedious to continue updating the main
`Cargo.toml` file with all of the "members" of the workspace,
[I wrote a bash script](https://github.com/zachkrall/dotfiles/blob/zkmbp/bin/nannou)
that offers two functions

```bash
nannou run [SKETCHNAME]   # runs a sketch
```

```bash
nannou update             # updates members list
                          #   in main Cargo.toml
```


TODO: instructions for how to install nannou cli?
