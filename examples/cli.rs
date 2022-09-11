#![feature(iter_intersperse)]

use std::env;

use doi2bib::Doi2Bib;

#[tokio::main(flavor="current_thread")]
async fn main() {
    let arg = env::args()
        .skip(1)
        .intersperse(" ".to_string())
        .collect::<String>();
    let resolver = Doi2Bib::new().unwrap();
    let bibtex = resolver.resolve(&arg).await.unwrap().unwrap();
    println!("{}", bibtex);
}
