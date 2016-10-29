# urlq
urlq is a tool for encoding and decoding URL queries from the terminal.

## Usage

```
$ urlq --help
urlq 0.1.0
James Hall
Encode and decode URL queries

USAGE:
    urlq <INPUT>... <--decode|--encode>

FLAGS:
    -d, --decode     Decode the URL query input
    -e, --encode     Encode the input into a URL query
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <INPUT>...
```

#### Encoding
```
$ urlq -e 'the quick brown fox jumps over the lazy dog!'
the+quick+brown+fox+jumps+over+the+lazy+dog%21
```

#### Decoding
```
$ urlq -d 'the+quick+brown+fox+jumps+over+the+lazy+dog%21'
the quick brown fox jumps over the lazy dog!
```

## Building

urlq is written in Rust and compiles on the stable toolchain.  With rustc and cargo installed, run:

```
$ cargo build --release
```

## Testing

```
$ cargo test
```
