mod db;
mod research_checker_service;
pub mod schema;
use std::net::{SocketAddr, ToSocketAddrs};

use poem::{endpoint::BoxEndpoint, listener::TcpListener, IntoEndpoint, Response};
use poem_grpc::{RouteGrpc, Service};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), String> {
    #[cfg(feature = "dotenv")]
    if let Err(_) = dotenv::dotenv() {
        eprintln!("Unable to load env");
    } else {
        println!("Loaded env");
    }

    #[cfg(feature = "parse_retraction_table")]
    parse_retraction_table().await?;

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

#[cfg(feature = "parse_retraction_table")]
async fn parse_retraction_table() -> Result<(), String> {
    let file = std::fs::File::open("retraction_watch.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    for record in rdr.records() {
        if let Ok(r) = record {
            let retraction = create_retraction_from_record(&r)?;
            diesel::insert_into(schema::retraction::table)
                .values(retraction)
                .returning(Retraction::as_returning())
                .get_result(&mut pg_connection)
                .expect("a");
        }
    }
    Ok(())
}

#[cfg(feature = "parse_retraction_table")]
fn create_retraction_from_record(record: &csv::StringRecord) -> Result<Retraction, String> {
    let idd = record.get(0).unwrap().parse().unwrap();
    let titlee = record.get(1).map(|f| f.to_string());
    let institution = record.get(3);
    let ins_vector = institution
        .map(|a| a.split(";").map(|f| f.to_string()).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>();

    let journall = record.get(4).map(|f| f.to_string());
    let authorss = record.get(7);
    let authors_vector = authorss
        .map(|a| a.split(";").map(|f| f.to_string()).collect::<Vec<_>>())
        .unwrap_or_default();

    let doii = record.get(11).map(|f| f.to_string());
    let pubmed_idd = record.get(12).map(|f| f.to_string());
    let reasons = record.get(17);
    let reasons_vector = reasons
        .map(|a| {
            a.split(";")
                .map(|f| f.replace("+", "").to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
        .into_iter()
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>();

    Ok(Retraction {
        id: idd,
        title: titlee,
        journal: journall,
        link: None,
        doi: doii,
        pubmed_id: pubmed_idd,
        affiliations: ins_vector.iter().map(|a| Some(a.clone())).collect(),
        countries: vec![],
        authors: authors_vector.iter().map(|a| Some(a.clone())).collect(),
        issues: reasons_vector.iter().map(|a| Some(a.clone())).collect(),
    })
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
