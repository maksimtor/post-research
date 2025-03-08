use super::proto::{
    CheckAffiliation, CheckAuthor, CheckCitation, CheckPubBase, CheckRequest,
    CheckResponse, ParseRequest, ResearchParserServiceClient,
};
use crate::db::Retraction;
use crate::schema::retraction::dsl::*;
use poem_grpc::{ClientConfig, Request, Response, Status};

use diesel::prelude::*;
use std::env;

use diesel::PgConnection;

pub struct ResearchCheckerService {}

#[poem::async_trait]
impl super::proto::ResearchCheckerService for ResearchCheckerService {
    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckResponse>, Status> {
        let rps = ResearchParserServiceClient::new(
            ClientConfig::builder()
                .uri("http://localhost:50051")
                .build()
                .unwrap(),
        );

        let res = rps
            .parse(Request::new(ParseRequest {
                url: request.url.clone(),
            }))
            .await
            .unwrap();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut pg_connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        let mut check_authors: Vec<CheckAuthor> = vec![];
        let mut check_affiliations: Vec<CheckAffiliation> = vec![];
        let mut check_citations: Vec<CheckCitation> = vec![];

        for author in res.authors.clone() {
            let results: Vec<Retraction> = retraction
                .filter(authors.contains(vec![author.name.clone()]))
                .select(Retraction::as_select())
                .load(&mut pg_connection)
                .unwrap();

            if !results.is_empty() {
                check_authors.push(CheckAuthor {
                    name: author.name,
                    id: "".to_string(),
                    retracted_articles: results.into_iter().map(|v| v.into()).collect(),
                });
            }
        }

        for affiliation in res.affiliations.clone() {
            let split: Vec<&str> = affiliation.name.split(".").collect();

            let results: Vec<Retraction> = retraction
                .filter(affiliations.contains(vec![
                    split.first().map(|v| *v).unwrap_or_default().to_string(),
                ]))
                .select(Retraction::as_select())
                .load(&mut pg_connection)
                .unwrap();

            if !results.is_empty() {
                check_affiliations.push(CheckAffiliation {
                    name: affiliation.name,
                    retracted_articles: results.into_iter().map(|v| v.into()).collect(),
                });
            }
        }

        for citation in res.citations.clone() {
            let results: Vec<Retraction> = retraction
                .filter(doi.eq(citation.doi.clone()))
                .select(Retraction::as_select())
                .load(&mut pg_connection)
                .unwrap();

            if !results.is_empty() {
                check_citations.push(CheckCitation {
                    name: citation.name,
                    doi: citation.doi,
                    pubmed_id: citation.pubmed_id,
                    retracted: true,
                });
            }
        }

        let mut pbb = None;

        pub fn capitalize(s: &str) -> String {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        }

        if let Some(pb) = res.pub_base.clone() {
            let split: Vec<&str> = pb.name.split(' ').collect();
            let capitalized: Vec<String> = split.iter().map(|v| capitalize(*v)).collect();
            let c = capitalized.join(" ");
            let pb_results: Vec<Retraction> = retraction
                .filter(journal.eq_any([pb.name.clone(), pb.name.clone().to_lowercase(), c]))
                .select(Retraction::as_select())
                .load(&mut pg_connection)
                .unwrap();

            pbb = Some(CheckPubBase {
                name: pb.name,
                retracted_articles: pb_results.into_iter().map(|v| v.into()).collect(),
            });
        }

        Ok(Response::new(CheckResponse {
            name: res.name.clone(),
            authors: check_authors,
            affiliations: check_affiliations,
            citations: check_citations,
            fundings: vec![],
            pub_base: pbb,
            doi: res.doi.clone(),
            pubmed_id: res.pubmed_id.clone(),
        }))
    }
}
