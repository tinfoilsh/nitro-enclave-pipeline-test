use aws_nitro_enclaves_image_format::utils::eif_reader::EifReader;
use aws_nitro_enclaves_image_format::utils::get_pcrs;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha384};
use std::fs::File;
use std::io::BufReader;
use clap::Parser;

#[derive(Debug, Serialize, Deserialize)]
struct DsseDocument {
    #[allow(non_snake_case)]
    dsseEnvelope: DsseEnvelope,
}

#[derive(Debug, Serialize, Deserialize)]
struct DsseEnvelope {
    payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DecodedPayload {
    predicate: NitroMeasurements,
}

#[derive(Debug, Serialize, Deserialize)]
struct NitroMeasurements {
    #[serde(rename = "HashAlgorithm")]
    hash_algorithm: String,
    #[serde(rename = "PCR0")]
    pcr0: String,
    #[serde(rename = "PCR1")]
    pcr1: String,
    #[serde(rename = "PCR2")]
    pcr2: String,
}

impl NitroMeasurements {
    fn from_dsse_envelope(dsse_envelope: DsseEnvelope) -> Result<NitroMeasurements, anyhow::Error> {
        let decoded = STANDARD.decode(dsse_envelope.payload)?;
        let decoded_str = String::from_utf8(decoded)?;

        let decoded_payload: DecodedPayload = serde_json::from_str(&decoded_str)?;

        Ok(decoded_payload.predicate)
    }

    fn from_btree(btree: &std::collections::BTreeMap<String, String>) -> Result<NitroMeasurements, anyhow::Error> {
        let hash_algorithm = btree.get("HashAlgorithm").unwrap().to_string();
        let pcr0 = btree.get("PCR0").unwrap().to_string();
        let pcr1 = btree.get("PCR1").unwrap().to_string();
        let pcr2 = btree.get("PCR2").unwrap().to_string();

        Ok(NitroMeasurements {
            hash_algorithm,
            pcr0,
            pcr1,
            pcr2,
        })
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// EIF file
    #[arg(short, long)]
    eif: String,

    /// Attestation document
    #[arg(short, long)]
    bundle: String,
}

fn main() {
    let args = Args::parse();

    let mut eif_reader = EifReader::from_eif(args.eif).unwrap();

    let m = get_pcrs(
        &mut eif_reader.image_hasher,
        &mut eif_reader.bootstrap_hasher,
        &mut eif_reader.app_hasher,
        &mut eif_reader.cert_hasher,
        Sha384::new(),
        eif_reader.signature_section.is_some(),
    ).unwrap();
    let eif_measurements = NitroMeasurements::from_btree(&m).unwrap();

    let file = File::open(args.bundle).unwrap();
    let reader = BufReader::new(file);
    let document: DsseDocument = serde_json::from_reader(reader).unwrap();
    let sig_measurements = NitroMeasurements::from_dsse_envelope(document.dsseEnvelope).unwrap();

    let mut ok = true;

    if eif_measurements.hash_algorithm != sig_measurements.hash_algorithm {
        println!("Hash algorithm mismatch");
    }

    if eif_measurements.pcr0 != sig_measurements.pcr0 {
        println!("PCR0 mismatch");
        ok = false;
    }
    if eif_measurements.pcr1 != sig_measurements.pcr1 {
        println!("PCR1 mismatch");
        ok = false;
    }
    if eif_measurements.pcr2 != sig_measurements.pcr2 {
        println!("PCR2 mismatch");
        ok = false;
    }

    if ok {
        println!("PCR match ok");
    } else {
        println!("PCR match failed");
    }
}
