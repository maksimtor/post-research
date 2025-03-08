mod db;
mod research_checker_service;
pub mod schema;
use std::{
    net::{SocketAddr, ToSocketAddrs},
};

use config::builder::AsyncState;
use poem::{endpoint::BoxEndpoint, listener::TcpListener, IntoEndpoint, Response};
use poem_grpc::{RouteGrpc, Service};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), String> {
    #[cfg(feature = "dotenv")]
    if let Err(e) = dotenv::dotenv() {
        eprintln!("Unable to load env");
    } else {
        println!("Loaded env");
    }

    #[cfg(feature = "parse_retraction_table")]
    {
        let file = std::fs::File::open("retraction_watch.csv").unwrap();
        let mut rdr = csv::Reader::from_reader(file);
        let mut of = true;

        for record in rdr.records() {
            if let Ok(r) = record {
                //Record ID,Title,Subject,Institution,Journal,Publisher,Country,Author,URLS,
                //ArticleType,RetractionDate,RetractionDOI,RetractionPubMedID,OriginalPaperDate,
                //OriginalPaperDOI,OriginalPaperPubMedID,RetractionNature,Reason,Paywalled,Notes
                let idd = r.get(0);
                let titlee = r.get(1);
                let institution = r.get(3);
                let mut ins_vector = vec![];
                if let Some(a) = institution {
                    let av: Vec<String> = a.split(";").map(|f| f.to_string()).collect();
                    ins_vector = av.into_iter().filter(|p| !p.is_empty()).collect();
                }

                let journall = r.get(4);

                let authorss = r.get(7);
                let mut authors_vector = vec![];
                if let Some(a) = authorss {
                    let av: Vec<String> = a.split(";").map(|f| f.to_string()).collect();
                    authors_vector = av;
                }
                let doii = r.get(11);
                let pubmed_idd = r.get(12);
                let reasons = r.get(17);
                let mut reasons_vector = vec![];
                if let Some(a) = reasons {
                    let av: Vec<String> = a
                        .split(";")
                        .map(|f| f.replace("+", "").to_string())
                        .collect();
                    reasons_vector = av.into_iter().filter(|p| !p.is_empty()).collect();
                }

                let retractionn = Retraction {
                    id: idd.unwrap().parse().unwrap(),
                    title: titlee.map(|f| f.to_string()),
                    journal: journall.map(|f| f.to_string()),
                    link: None,
                    doi: doii.map(|f| f.to_string()),
                    pubmed_id: pubmed_idd.map(|f| f.to_string()),
                    affiliations: ins_vector.iter().map(|a| Some(a.clone())).collect(),
                    countries: vec![],
                    authors: authors_vector.iter().map(|a| Some(a.clone())).collect(),
                    issues: reasons_vector.iter().map(|a| Some(a.clone())).collect(),
                };

                diesel::insert_into(schema::retraction::table)
                    .values(retractionn)
                    .returning(Retraction::as_returning())
                    .get_result(&mut pg_connection)
                    .expect("a");
            }
        }
    }

    let config_builder = config::ConfigBuilder::<AsyncState>::default();
    let config_builded = config_builder
        .add_source(
            config::Environment::with_prefix(&"name".to_uppercase())
                .separator("__")
                .prefix_separator("_"),
        )
        .build()
        .await
        .unwrap();

    let mut server = GrpcServer {
        router: Some(RouteGrpc::new()),
        settings: Config {
            addr: "localhost".to_string(),
            port: "3001".to_string(),
        },
    };

    server.add_service(
        research_checker_service::proto::ResearchCheckerServiceServer::new(
            research_checker_service::ResearchCheckerService {},
        ),
    );

    server.run().await
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub(crate) addr: String,
    pub(crate) port: String,
}

impl TryInto<SocketAddr> for Config {
    type Error = String;

    fn try_into(self) -> Result<SocketAddr, Self::Error> {
        Ok(format!("{}:{}", self.addr, self.port)
            .to_socket_addrs()
            .unwrap()
            .filter(|a| a.is_ipv4())
            .next()
            .unwrap())
    }
}

pub struct GrpcServer {
    pub router: Option<RouteGrpc>,
    pub settings: Config,
}

impl GrpcServer {
    pub fn add_service<S>(&mut self, service: S)
    where
        S: IntoEndpoint<Endpoint = BoxEndpoint<'static, Response>> + Service,
    {
        self.router = self.router.take().map(|r| r.add_service(service))
    }

    pub async fn run(self) -> Result<(), String> {
        let router = self.router.unwrap();
        let addr: SocketAddr = self.settings.try_into()?;

        Ok(poem::Server::new(TcpListener::bind(addr))
            .run(router)
            .await
            .unwrap())
    }
}
