mod scan;
mod adb;

use adb::execute_adb_reserved_word;
use clap::Arg;
use clap::Command as ClapCommand;
use scan::discover_and_resolve_services;
use adb::execute_adb_command;

#[tokio::main]
async fn main() {
    let matches = ClapCommand::new("ntdis")
        .version("1.0")
        .author("yomifrogs")
        .about("Discovers mDNS services on the local network")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            ClapCommand::new("scan")
                .about("Performs scan mDNS service discovery")
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .action(clap::ArgAction::SetTrue)
                        .help("Enables verbose output"),
                )
                .arg(
                    Arg::new("duration")
                        .short('d')
                        .long("duration")
                        .value_parser(clap::value_parser!(u64))
                        .default_value("3")
                        .help("Sets the scan duration for service discovery in seconds"),
                )
                .arg(
                    Arg::new("service_type")
                        .short('t')
                        .long("service_type")
                        .help("Specifies the service type to discover"),
                ),
        )
        .subcommand(
            ClapCommand::new("adb")
                .about("Executes adb commands")
                .subcommand(
                    ClapCommand::new("connect")
                        .about("Connects to a device")
                        .arg(
                            Arg::new("scan_duration")
                                .short('s')
                                .long("scan_duration")
                                .value_parser(clap::value_parser!(u64))
                                .default_value("3")
                                .help("Sets the scan duration for service discovery in seconds"),
                        )
                )
                .subcommand(
                    ClapCommand::new("pair")
                        .about("Pairs with a device")
                        .arg(
                            Arg::new("scan_duration")
                                .short('s')
                                .long("scan_duration")
                                .value_parser(clap::value_parser!(u64))
                                .default_value("3")
                                .help("Sets the scan duration for service discovery in seconds"),
                        )
                )
                .arg(
                    Arg::new("args")
                        .num_args(1..)
                        .help("Arguments to pass to adb"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("scan", sub_m)) => {
            let verbose = sub_m.get_flag("verbose");
            let duration = *sub_m
                .get_one::<u64>("duration")
                .expect("Duration must be a number");
            let service_type = sub_m.get_one::<String>("service_type").map(|s| s.as_str());

            println!("ネットワーク上のmDNSサービスを検索中...");
            discover_and_resolve_services(duration, verbose, service_type).await;
        }
        Some(("adb", sub_m)) => {
            match sub_m.subcommand() {
                Some(("connect", sub_m)) => {
                    let scan_duration = *sub_m.get_one::<u64>("scan_duration").expect("Scan duration must be a number");
                    execute_adb_reserved_word("connect", scan_duration);
                }
                Some(("pair", sub_m)) => {
                    let scan_duration = *sub_m.get_one::<u64>("scan_duration").expect("Scan duration must be a number");
                    execute_adb_reserved_word("pair", scan_duration);
                }
                _ => {
                    if let Some(args) = sub_m.get_many::<String>("args") {
                        let args: Vec<&str> = args.map(|s| s.as_str()).collect();
                        execute_adb_command(args);
                    }
                }
            }
        }
        _ => {
            // noop
        }
    }
}
