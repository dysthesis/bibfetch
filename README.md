# Bibfetch

A command-line bibliographic citation data fetcher. Pass in an identifier, and get corresponding citation data in the form of JSON.

## Handlers

Lua modules which handles the parsing of identifiers of a given type and fetching of data. These also validate whether a given identifier matches said type, returning `nil` if it does not.

A handler is a table which must contain the attributes

- `info`, which is a table containing information on the handler, specifically
  - its `name` as a `string`, and
  - its `priority` as an `number`,
- `parse()`, which is a function which parses the given identifier, returning the results if successful and `nil` otherwise, and
- `fetch()`, which fetches the bibliographic metadata for the parsed identifier, returning a table.

There is a built-in function for handlers to use to make HTTP get requests, called `request(url)`. This returns the JSON output parsed into a Lua table.

## Work in progress

- Moving built-in functions like `request()` into WASM plugins.
- A separate CLI to translate between different serial data formats, including JSON, BibTex, TOML, YAML, etc. This is why `bibfetch` outputs JSON instead of BibTex.

## Credits

The design of this is largely inspired by [Zotero's translators](https://github.com/zotero/translators).
