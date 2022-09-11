use std::env;

use doi2bib::Doi2Bib;

#[tokio::main(flavor="current_thread")]
async fn main() {
    // TODO: use iter_intersperse once stable
    let arg = env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");
    let resolver = Doi2Bib::new().unwrap();
    let bibtex = resolver.resolve(&arg).await.unwrap().unwrap();
    println!("{}", bibtex);
}
