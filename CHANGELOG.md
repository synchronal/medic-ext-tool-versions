# Change log

## Unreleased

- Updated error messages.
- `plugin-installed` can take multiple plugins by repeating `-p`.

## 0.3.0

- `mise` includes `erlang` as a core plugin.

## 0.2.0

- Add `mise` to potential runtime version managers, based on upstream rename
  of `jtx` to `mise`.

## 0.1.0

- Prefer `rtx` to `asdf`.
- `rtx` core plugins always return ok for `plugin-installed`.
- Check for existence of `asdf`, then `rtx` for calls to the runtime manager.
- Initial releaase, beginning with code from `medic-ext-asdf`.
