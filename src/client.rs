pub mod pb {
    tonic::include_proto!("grpc.examples.unaryecho");
}

use clap::Parser;
use pb::{echo_client::EchoClient, EchoRequest};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    cert_file: std::path::PathBuf,

    #[arg(short, long, required = true)]
    key_file: std::path::PathBuf,

    #[clap(short, long, required = true)]
    ca_file: std::path::PathBuf,

    #[clap(short, long, default_value = "https://[::1]:50051")]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();
    let server_root_ca_cert = std::fs::read_to_string(cli.ca_file)?;
    let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);

    let client_cert = std::fs::read_to_string(cli.cert_file)?;
    let client_key = std::fs::read_to_string(cli.key_file)?;

    let client_identity = Identity::from_pem(client_cert, client_key);

    let tls = ClientTlsConfig::new()
        .domain_name("localhost")
        .ca_certificate(server_root_ca_cert)
        .identity(client_identity);

    let channel = Channel::from_shared(cli.addr)?
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = EchoClient::new(channel);

    {
        let request = tonic::Request::new(EchoRequest {
            message: "hello1".into(),
        });
        let response = client.unary_echo(request).await?;
        println!("RESPONSE1={:?}", response);
    }

    {
        let request = tonic::Request::new(EchoRequest {
            message: "hello2".into(),
        });
        let response = client.unary_echo(request).await?;
        println!("RESPONSE2={:?}", response);
    }

    Ok(())
}
