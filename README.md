# kicad-lib-docgen

> A simple utility for generating markdown files with a table of all symbols and footprints in KiCAD libraries.

## Install and use

`cargo install --path .`

You can also run from repo with `cargo run -- symbols --help` which will print the help for symbols.


## Usage

### Example

```sh
# can also run with: cargo run -- symbols ....
kicad-lib-docgen symbols -c symbol -c datasheet  -c footprint "My amazing library" ~/kicad/amazinglib/symbols/amazing.kicad_sym ~/kicad/amazinglib/symbols/amazing.md
```

Will render (I used https://github.com/wntrblm/winterbloom_kicad_library/blob/main/symbols/winterbloom.kicad_sym as an example) the following (showing only first 6):

---

# My amazing library


Symbol | Datasheet | Footprint
---|---|---
74HC2G34 | https://assets.nexperia.com/documents/data-sheet/74HC_HCT2G34.pdf | Package_TO_SOT_SMD:SOT-363_SC-70-6
74LVC2G74 | https://assets.nexperia.com/documents/data-sheet/74LVC2G74.pdf | 
AD5685RxRUZ | https://www.analog.com/media/en/technical-documentation/data-sheets/ad5686r_5685r_5684r.pdf | Package_SO:TSSOP-16_4.4x5mm_P0.65mm
ADG1208 | https://www.analog.com/media/en/technical-documentation/data-sheets/ADG1208_1209.pdf | Package_SO:TSSOP-16_4.4x5mm_P0.65mm
ADG1209 | https://www.analog.com/media/en/technical-documentation/data-sheets/ADG1208_1209.pdf | Package_SO:TSSOP-16_4.4x5mm_P0.65mm
ADG1308 | https://www.analog.com/media/en/technical-documentation/data-sheets/ADG1308_1309.pdf | Package_SO:TSSOP-16_4.4x5mm_P0.65mm

---

### kicad-lib-docgen symbols --help


```sh
Create README for symbols

Usage: kicad-lib-docgen symbols [OPTIONS] <TITLE> <IN_FILE> <OUT_FILE>

Arguments:
  <TITLE>     Title of README
  <IN_FILE>   Path to input kicad_sym file
  <OUT_FILE>  Path to output README.md file

Options:
  -c, --column <COLUMN>  Add multiple columns. Default is `-c symbol -c footprint -c datasheet` [possible
                         values: symbol, reference, footprint, datasheet, value]
  -e, --env <ENV>        ENV is key=value, use to replace paths for datasheets
  -h, --help             Print help
  -V, --version          Print version
```

### kicad-lib-docgen symbols --help


```sh
Create README for footprints

Usage: kicad-lib-docgen footprints [OPTIONS] <TITLE> <IN_FILE> <OUT_FILE>

Arguments:
  <TITLE>     Title of README
  <IN_FILE>   Path to input kicad_sym file
  <OUT_FILE>  Path to output README.md file

Options:
  -c, --column <COLUMN>  Add multiple columns. Default is `-c footprint -c step` [possible values:
                         footprint, step]
  -e, --env <ENV>        ENV is key=value, use to replace paths for datasheets
  -h, --help             Print help
  -V, --version          Print version
```