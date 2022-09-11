use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::header;

static DOI_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new("\\b(10[.][0-9]{3,}(?:[.][0-9]+)*/\\S+)\\b").unwrap()
});

pub struct Doi2Bib {
    client: reqwest::Client,
}

impl Doi2Bib {
    pub fn new() -> Result<Self, Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Accept", header::HeaderValue::from_static("application/x-bibtex"));

        
        let client = reqwest::Client::builder()
            .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
            .default_headers(headers)
            .build()?;

        Ok(Doi2Bib {
            client
        })
    }

    pub async fn resolve_doi(&self, doi: &str) -> Result<String, Error> {
        self.resolve_doi_url(&format!("https://doi.org/{}", doi)).await
    }

    pub async fn resolve_doi_url(&self, doi_url: &str) -> Result<String, Error> {
        let response = self.client.get(doi_url).send().await?.text().await?;
        Ok(response)
    }

    pub async fn resolve(&self, text: &str) -> Result<Option<String>, Error> {
        match DOI_PATTERN.find(text) {
            Some(m) => {
                let doi = &text[m.start()..m.end()];
                let response = self.resolve_doi(doi).await?;
                Ok(Some(response))
            },
            None => Ok(None)
        }
    }
}

pub type Error = reqwest::Error;

#[cfg(test)]
mod tests {
    use biblatex::ChunksExt;

    use super::*;

    #[tokio::test]
    async fn test_resolve() {
        let doi2bib = Doi2Bib::new().unwrap();
        let bibtex = doi2bib.resolve_doi("10.1109/5.771073").await.unwrap();
        let bib = biblatex::Bibliography::parse(&bibtex).unwrap();
        assert_eq!(bib.len(), 1);
        let entry = bib.into_iter().next().unwrap();
        assert_eq!(entry.doi().ok(), Some("10.1109/5.771073".to_string()));
        let title = entry.title()
            .ok()
            .map(|titles| titles.format_sentence());
        assert_eq!(title, Some("Toward unique identifiers".to_string()));
    }

    #[test]
    fn test_doi_pattern() {
        let dois = [
            "10.1109/5.771073",
            "10.1007/s10618-018-0568-8",
            "10.1016.12.31/nature.S0735-1097(98)2000/12/31/34:7-7",
            "10.1007/978-3-642-28108-2_19",
            "10.1579/0044-7447(2006)35\\[89:RDUICP\\]2.0.CO;2",
            "10.1007.10/978-3-642-28108-2_19",
            "10.1579/0044-7447(2006)35\\[89:RDUICP\\]2.0.CO;2",
            "10.1016/S0735-1097(98)00347-7",
            "10.1038/ejcn.2010.73",
            "10.1038/ejcn.2010.73",
            "10.1000/123456",
            "10.1038/issn.1476-4687"
        ];
        for doi in dois.iter() {
            assert!(DOI_PATTERN.is_match(doi));
        }
    }
}
