// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fuzzy_matcher::skim::SkimMatcherV2;
use mlua::{Lua, Result, Table, ToLua};
use once_cell::sync::Lazy;

static MATCHER: Lazy<SkimMatcherV2> = Lazy::new(SkimMatcherV2::default);

type FuzzyScore = (i64, Vec<usize>);

struct ScoredItem {
    item: String,
    score: i64,
    indexes: Vec<usize>,
}

fn to_lua_table<'lua, T>(lua: &'lua Lua, list: Vec<T>) -> Result<Table<'lua>>
where
    T: ToLua<'lua>,
{
    let t = lua.create_table()?;
    for item in list {
        t.push(item)?
    }
    Ok(t)
}

fn fuzzy(lua: &Lua, (input, items): (String, Vec<String>)) -> Result<Table> {
    let t = lua.create_table()?;

    let scored: Vec<(String, Option<FuzzyScore>)> = items
        .iter()
        .map(|item| (item.clone(), MATCHER.fuzzy(item, &input, true)))
        .collect();

    let mut filtered: Vec<ScoredItem> = scored
        .into_iter()
        // only retains keybind tokens with a matching score
        .filter(|(_, score)| score.is_some())
        .map(|(item, score)| {
            let (score, indexes) = score.unwrap();
            ScoredItem {
                item,
                score,
                indexes,
            }
        })
        .collect();

    // sort by fuzzy score
    filtered.sort_by(|a, b| b.score.cmp(&a.score));

    for item in filtered {
        let table = lua.create_table()?;
        let indexes = to_lua_table(lua, item.indexes)?;
        table.set("item", item.item)?;
        table.set("score", item.score)?;
        table.set("indexes", indexes)?;
        t.push(table)?
    }

    Ok(t)
}

#[mlua::lua_module]
fn ruzy(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;
    exports.set("fuzzy", lua.create_function(fuzzy)?)?;
    Ok(exports)
}

// pierre_d
