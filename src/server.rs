pub mod pb {
    tonic::include_proto!("grpc.examples.unaryecho");
}

use clap::Parser;
use pb::{EchoRequest, EchoResponse};
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};
use x509_parser::certificate::X509Certificate;
use x509_parser::der_parser::asn1_rs::FromDer;
use x509_parser::x509::X509Name;

type EchoResult<T> = Result<Response<T>, Status>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    cert_file: std::path::PathBuf,

    #[arg(short, long, required = true)]
    key_file: std::path::PathBuf,

    #[clap(short, long, required = true, value_delimiter = ' ', num_args = 1..)]
    ca_list: Vec<std::path::PathBuf>,

    #[clap(short, long, default_value = "[::1]:50051")]
    addr: String,
}

fn get_first_cn_as_str<'a>(name: &'a X509Name<'_>) -> Option<&'a str> {
    name.iter_common_name()
        .next()
        .and_then(|cn| cn.as_str().ok())
}

#[derive(Default)]
pub struct EchoServer;

#[tonic::async_trait]
impl pb::echo_server::Echo for EchoServer {
    async fn unary_echo(&self, request: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        let certs = request
            .peer_certs()
            .expect("Client did not send its certs!");

        let cert_buf = certs[0].clone().into_inner();
        let res = X509Certificate::from_der(&cert_buf);
        match res {
            Ok((rem, cert)) => {
                assert!(rem.is_empty());
                println!("cn: {:?}", get_first_cn_as_str(cert.issuer()));
                println!("pk: {:?}", cert.public_key());
            }
            _ => panic!("x509 parsing failed: {:?}", res),
        }

        let message = request.into_inner().message;
        Ok(Response::new(EchoResponse { message }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();
    let cert = std::fs::read_to_string(cli.cert_file)?;
    let key = std::fs::read_to_string(cli.key_file)?;

    let server_identity = Identity::from_pem(cert, key);

    let client_ca_cert_buf = {
        let list = cli
            .ca_list
            .iter()
            .map(std::fs::read_to_string)
            .collect::<Result<Vec<_>, _>>()?;
        list.join("")
    };
    let client_ca_cert = Certificate::from_pem(client_ca_cert_buf);

    let addr = cli.addr.parse().unwrap();
    let server = EchoServer;

    let tls = ServerTlsConfig::new()
        .identity(server_identity)
        .client_ca_root(client_ca_cert);

    Server::builder()
        .tls_config(tls)?
        .add_service(pb::echo_server::EchoServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}