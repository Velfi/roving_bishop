# Roving Bishop

I followed [this paper][drunken-bishop] but messed up some of the bit-reading parts so it won't work exactly the same as the SSH one does.

## Building it yourself

```sh
cargo build --release
```

## Usage

```sh
$ roving_bishop
╬═════════════════╬
║         ...   . ║
║         ........║
║          = o.. o║
║         * =   . ║
║        S + .    ║
║        .  .     ║
║       ..E.      ║
║      ..=o.      ║
║      .+.oo.     ║
╬═════════════════╬
```

[drunken-bishop]: http://www.dirk-loss.de/sshvis/drunken_bishop.pdf
