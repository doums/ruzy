## ruzy

⚡ A Lua library for fuzzy search

### API

The library exposes a Lua module with one function:

#### fuzzy (input: `string`, items: `list`<`string`>)\

returns: `list`<{\
            item: `string`,\
            score: `number`,\
            indexes: `list`<`number`>\
         }>

`fuzzy` function takes 2 arguments. The first is the search
input, the second is the list of string to fuzzy match.

The function returns the list of filtered matching items with
corresponding score and matching indexes, sorted by score.

### Usage

Move the `ruzy.so` shared library in your project and import it 
from Lua code.

```lua
  local fuzzy = require('ruzy').fuzzy
  local res = fuzzy('lou', { 'pierre', 'et', 'le', 'loup' })
  -- res:
  -- { {
  --     indexes = { 0, 1, 2 },
  --     item = "loup",
  --     score = 71
  -- } }
```

### License

Mozilla Public License 2.0
