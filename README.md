# SI Unit Prefix

This library is ideal for simple Rust handling of SI unit prefixes.

## Features

- You can use `SiUnitPrefix::G` to represent "Giga".
  - Supported: "Q" "R" "Y" "Z" "E" "P" "T" "G" "M" "k" "h" "da" "d" "c" "m" "u" "n" "p" "f" "a" "z" "y" "r" "q"
    - Extra: "㌐" "㍋" "㌔" "㌥" "㌢" "㍉" "㍃" "㌨" "㌰"
- `.as_f64()`: eg. `SiUnitPrefix::G.as_f64()` -> `1e+9f64`
- `.as_f32()`: eg. `SiUnitPrefix::G.as_f32()` -> `1e+9f32`
- `.as_exp()`: eg. `SiUnitPrefix::G.as_exp()` -> `9i8`
- `.parse(str)`: eg. `SiUnitPrefix::parse_from_str("G")` -> `SiUnitPrefix::G`
- with derives:
  - `serde::Serialize`/`serde::Deserialize`
  - `Clone`/`Copy`
  - `PartialEq`/`Eq`
  - `PartialOrd`/`Ord`
  - `Debug`

## Examples/Tests

See [tests/test.rs](tests/test.rs) file. It's so simple. :)

## License

MIT

## Author

- [Usagi Ito / USAGI.NETWORK](https://usagi.network/)
