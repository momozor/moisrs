# Moisrs

*moisrs* is a CLI program to generate and view informal software requirement
specification (SRS) easily.

## Version

0.5.0

## Installation

> This software is not available on crates.io on purpose.

> Please make sure that `.cargo/bin/` is in your `PATH`.

### Directly from Github with git

`cargo install --git https://github.com/momozor/moisrs`

### Locally

`cargo install --path .`


## Usage

*moisrs* comes as a CLI program.
Once installed, you can invoke *moisrs* from the command line to generate a
`SPECIFICATION` YAML formatted file based on your software/project requirements.

To view the YAML file as table, just run `moisrs --show` in the directory
where `SPECIFICATION` file is located.

For more details, invoke `moisrs --help`.


## Contributing

Bug reports and pull requests are welcome on GitHub
at https://github.com/momozor/moisrs.


## License

This software is released under the BSD-3-Clause license. Please see
LICENSE file for more details.
