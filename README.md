Flatten JSON is a simple utility that prints flattened key-value pairs of a JSON input.
The key is the JSON Path of the value.
The goal of this utility is to be used alongside a grep-like program to find the specific nested paths for values that are needed in the configuration.
This makes extracting values using utilities like `jq` easier.

## Usage

You can use `fj` in combination with `ripgrep` to quickly locate specific paths of values:

```sh
fj input.json | rg desired_value
```

Additionally, `fj` can read from STDIN:

```sh
kubectl -n kube-system get pods -o json | fj - | rg podIP
```

## Install

```sh
cargo install --git https://github.com/tylerkrop/fj
```
