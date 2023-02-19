## ruzy

⚡ A Lua library for fuzzy search

### API

The library exposes a Lua module with one function:

#### fuzzy (input: `string`, items: `list<string>`)

`fuzzy` function takes 2 arguments. The first is the search
input, the second is the list of string to fuzzy match.

returns a list of the matching items sorted by score. Each
item includes:

- item: `string`
- score: `number`, fuzzy score
- indexes: `list<number>`, the matching indexes

### Usage

Move the `libruzy.so` shared library (latest [release](https://github.com/doums/ruzy/releases/latest))
in your project and import it from Lua code.

```lua
  local fuzzy = require('libruzy').fuzzy
  local res = fuzzy('lou', { 'pierre', 'et', 'le', 'loup' })
  -- res:
  -- { {
  --     item = "loup",
  --     score = 71,
  --     indexes = { 0, 1, 2 }
  -- } }
```

### Algorithm

Skim V2

### Build

#### prerequisites:

- [rust](https://www.rust-lang.org/tools/install)

To build for production run

```
cargo build --release
```

### Credits

- https://crates.io/crates/fuzzy-matcher
- https://github.com/khvzak/mlua

### License

Mozilla Public License 2.0
