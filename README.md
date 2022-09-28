# Colorize lines by pattern

Colorize reads lines from stdin and colorizes the matching lines according to the set regular
expression pattern.

Supported colors: red (default), blue, green, yellow, purple, cyan, black, white

Patterns and colors can be specifed more than once to use multiple lines. Eg:

```sh
$ colorize -p error -c yellow -p debug -c green
```


```
USAGE:
    colorize [OPTIONS]

OPTIONS:
    -c, --color <COLOR>
            Color to use (default: red)

    -h, --help
            Print help information

    -p, --pattern <PATTERN>
            Regular expression pattern to match lines

    -V, --version
            Print version information
```

## TODO 

- [ ] input color validation 
- [ ] bold for matched groups
- [ ] truecolor (rgb) support 
- [ ] action for release
