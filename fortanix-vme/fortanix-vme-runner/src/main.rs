use clap::Parser;
use fortanix_vme_runner::Server;
use fortanix_vme_abi::SERVER_PORT;
use nitro_cli::common::commands_parser::RunEnclavesArgs;
use nitro_cli::common::logger;
use std::io::ErrorKind;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    enclave_file: String,
}

impl From<&Cli> for RunEnclavesArgs {
    fn from(cli: &Cli) -> RunEnclavesArgs {
        RunEnclavesArgs {
            eif_path: cli.enclave_file.clone(),
            enclave_cid: None,
            memory_mib: 64,
            cpu_ids: None,
            debug_mode: None,
            cpu_count: None,
            enclave_name: None,
        }
    }
}

fn create_nitro_enclave(args: &Cli) {
    let logger = logger::init_logger()
        .expect("Log init failed");
    let run_args = args.into();
    nitro_cli::create_enclave(run_args, &logger).expect("Creation failed");
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    create_nitro_enclave(&cli);

    match Server::run(SERVER_PORT) {
        Ok(handle)                                   => { handle.join().unwrap(); },
        Err(e) if e.kind() == ErrorKind::AddrInUse   => println!("Server failed. Do you already have a runner running on vsock port {}? (Error: {:?})", SERVER_PORT, e),
        Err(e)                                       => println!("Server failed. Error: {:?}", e),
    }
}
