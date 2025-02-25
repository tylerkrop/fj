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

## Example

```
$ echo '{
    "null": null,
    "bool": true,
    "number": 42,
    "string": "Hello, world!",
    "array": [
        1,
        2,
        3
    ],
    "object": {
        "foo": "bar",
        "answer": 42
    }
}' | fj -
$.null: null
$.bool: true
$.number: 42
$.string: "Hello, world!"
$.array[0]: 1
$.array[1]: 2
$.array[2]: 3
$.object.foo: "bar"
$.object.answer: 42
```

## Install

```sh
cargo install --git https://github.com/tylerkrop/fj
```
