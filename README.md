# Medic Tool Versions

An extension pack for using [medic](https://github.com/synchronal/medic-rs)
with runtime managers that support `.tool-versions` files. It supports both
[asdf](https://asdf-vm.com) and [rtx](http://rtx.pub/), with a preference
for `rtx` if found in the PATH.

Also see [medic-ext-asdf](https://github.com/synchronal/medic-ext-asdf)
if a specific runtime version manager is preferred.

## Installation

```shell
brew tap synchronal/tap
brew install medic-ext-tool-versions
```

Example `Brewfile`:

```shell
tap 'synchronal/tap'

brew  'synchronal/tap/medic'
brew  'synchronal/tap/medic-ext-tool-versions'
```

## Usage

```toml
[doctor]

checks = [
  { check = "tool-versions", command = "plugin-installed", args = { plugin = "rust" } },
  { check = "tool-versions", command = "package-installed", args = { plugin = "rust" } },
]
```


# medic-check-tool-versions

Checks for whether plugins and specific plugin packages are installed
via the runtime version manager.

All checks do an initial test to find either `rtx` or `asdf`, in that order.


## plugin installed?

Checks whether a plugin is installed.

```shell
medic-check-tool-versions plugin-installed --plugin rust
```


## package installed?

Checks whether a package is installed for a specific plugin. If
`--version` is not passed, the version configured with `.tool-versions`
is used.

```shell
medic-check-tool-versions package-installed --plugin rust
medic-check-tool-versions package-installed --plugin rust --version nightly
```
