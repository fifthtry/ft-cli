fn main() {
    let cmd = clap::App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .required(false)
                .value_name("FILE")
                .default_value("ft-sync.p1")
                .help("path to the ft-sync config file")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("test")
                .short("t")
                .long("test")
                .required(false)
                .value_name("TEST")
                .help("if to run in test mode")
                .takes_value(false),
        )
        .subcommand(clap::SubCommand::with_name("status").about("show the sync status"))
        .subcommand(
            clap::SubCommand::with_name("sync").about("sync files").arg(
                clap::Arg::with_name("dry_run")
                    .help("run in dry run mode")
                    .short("n")
                    .long("dry-run"),
            ),
        )
        .get_matches();

    let config_file = cmd.value_of("config").unwrap();
    let config = ft_sync::Config::from_file(config_file).expect("failed to read config");

    match cmd.subcommand() {
        ("status", _) => {
            match ft_sync::status(&config, config_file) {
                Ok(()) => {}
                Err(e) => println!("{}", e.to_string()),
            }
            println!("Command: status()");
        }
        ("sync", _args) => {
            println!("syncing");
            match ft_sync::sync(&config, false) {
                Ok(()) => {}
                Err(e) => println!("{}", e.to_string()),
            }
        }
        (_, _) => todo!("impossible!"),
    };
}
