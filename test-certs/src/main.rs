//! This binary can be used to generate a test certificate authority and corresponding
//! client and server certificates that can be used for mutual TLS connections.

use clap::Parser;
use test_certs::{
    configuration::{certificates::CertificateRoot, Args},
    generate,
};
use tracing::info;

fn main() -> anyhow::Result<()> {
    // Init basic console subscriber
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let root: CertificateRoot = match args.format {
        test_certs::configuration::ConfigFormat::Yaml => {
            info!(
                "Loading YAML certificate generation file {:?}",
                args.configuration
            );
            serde_yaml::from_reader(std::fs::File::open(args.configuration.as_path())?)?
        }
        test_certs::configuration::ConfigFormat::Json => {
            info!(
                "Loading JSON certificate generation file {:?}",
                args.configuration
            );
            serde_json::from_reader(std::fs::File::open(args.configuration.as_path())?)?
        }
    };

    info!("Loaded {} root certificate(s)", root.certificates.len());

    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&args.outdir)?;

    let certificates = generate(root)?;

    for cert in certificates {
        cert.write(&args.outdir)?;
    }

    Ok(())
}
