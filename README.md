# doi2bib

[![CI](https://github.com/OpenByteDev/doi2bib/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/doi2bib/actions/workflows/ci.yml) [![crates.io](https://img.shields.io/crates/v/doi2bib.svg)](https://crates.io/crates/doi2bib) [![Documentation](https://docs.rs/doi2bib/badge.svg)](https://docs.rs/doi2bib) [![dependency status](https://deps.rs/repo/github/openbytedev/doi2bib/status.svg)](https://deps.rs/repo/github/openbytedev/doi2bib) [![MIT](https://img.shields.io/crates/l/doi2bib.svg)](https://github.com/OpenByteDev/doi2bib/blob/master/LICENSE)

Generate a bibtex entry from a doi.

# Usage

```rust
let doi2bib = doi2bib::Doi2Bib::new().unwrap();
let bibtex = doi2bib.resolve_doi("10.1109/5.771073").await.unwrap();
println!("{}", bibtex);
// @article{Paskin_1999, 
//     title={Toward unique identifiers},
//     volume={87},
//     ISSN={0018-9219}, 
//     url={http://dx.doi.org/10.1109/5.771073},
//     DOI={10.1109/5.771073},
//     number={7},
//     journal={Proceedings of the IEEE},
//     publisher={Institute of Electrical and Electronics Engineers (IEEE)},
//     author={Paskin, N.},
//     year={1999},
//     month=jul,
//     pages={1208–1227}
// }
```

## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/doi2bib/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
