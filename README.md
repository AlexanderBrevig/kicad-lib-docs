# kicad-lib-docs

> A simple utility for generating markdown files with a table of all symbols and footprints in KiCAD libraries.

## Install and use

Simply install with `cargo install --path .`.

You can also run from repo with `cargo run -- symbols --help` which will print the help for symbols.


## Usage

You can test it from the repo using `cargo run` as such:

```sh
cargo run -- symbols "My title" lib.kicad_sym lib.md
```
 
Or use the binary after installing with `cargo install --path .` like this:

```sh
# if you run after `cargo install --path .` 
kicad-lib-docs footprints \
  -c footprint -c step \
  "My amazing library" \
  ~/kicad/amazinglib/footprints/amazing.pretty \
  ~/kicad/amazinglib/footprints/amazing.md
```

### Example

Here is an example by using https://github.com/wntrblm/winterbloom_kicad_library/blob/main/symbols/winterbloom.kicad_sym as the input.

---

# Winterbloom


Symbol | Datasheet | Footprint
---|---|---
74HC2G34 | https://assets.nexperia.com/documents/data-sheet/74HC_HCT2G34.pdf | Package_TO_SOT_SMD:SOT-363_SC-70-6
74LVC2G74 | https://assets.nexperia.com/documents/data-sheet/74LVC2G74.pdf | 
AD5685RxRUZ | https://www.analog.com/media/en/technical-documentation/data-sheets/ad5686r_5685r_5684r.pdf | Package_SO:TSSOP-16_4.4x5mm_P0.65mm
ADG1208 | https://www.analog.com/media/en/technical-documentation/data-sheets/ADG1208_1209.pdf | Package_SO:TSSOP-16_4.4x5mm_P0.65mm
ADG1209 | https://www.analog.com/media/en/technical-documentation/data-sheets/ADG1208_1209.pdf | Package_SO:TSSOP-16_4.4x5mm_P0.65mm
ADG1308 | https://www.analog.com/media/en/technical-documentation/data-sheets/ADG1308_1309.pdf | Package_SO:TSSOP-16_4.4x5mm_P0.65mm

