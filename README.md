# doi2bib

[![CI](https://github.com/OpenByteDev/doi2bib/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/doi2bib/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/doi2bib.svg)](https://crates.io/crates/doi2bib)
[![Documentation](https://docs.rs/doi2bib/badge.svg)](https://docs.rs/doi2bib)
[![dependency status](https://deps.rs/repo/github/openbytedev/doi2bib/status.svg)](https://deps.rs/repo/github/openbytedev/doi2bib)
[![MIT](https://img.shields.io/crates/l/doi2bib.svg)](https://github.com/OpenByteDev/doi2bib/blob/master/LICENSE)

Generate a bibtex entry from a doi.

# Usage

```rust
let doi2bib = doi2bib::Doi2Bib::new().unwrap();
let bibtex = doi2bib.resolve_doi("10.1109/5.771073").await.unwrap();
println!("{}", bibtex);
// @article{Yu_2013,
//     doi = {10.1038/nature12097},
//     url = {https://doi.org/10.1038%2Fnature12097},
//     year = 2013,
//     month = {apr},
//     publisher = {Springer Science and Business Media {LLC}},
//     volume = {497},
//     number = {7448},
//     pages = {196--197},
//     author = {Shannon F. Yu and Mary K. Baylies},
//     title = {Death brings new life to muscle},
//     journal = {Nature}
// }
```

## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/doi2bib/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
