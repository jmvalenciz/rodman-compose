use clap::{Arg, App, SubCommand, AppSettings};

pub enum ActionType{
    Up,
    Down,
    Ps,
    Stop,
    Undefined
}

pub enum UpArgument{
    Detach,
    Network
}

pub enum PsArgument{
    All
}

pub struct Config{
    pub filename: String,
    pub action: ActionType,
    pub up_arguments: Vec<UpArgument>,
    pub ps_arguments: Vec<PsArgument>
}

impl Config{
    pub fn new()-> Self{
        let arg_matches = Config::read_arguments();

        let filename = String::from(arg_matches.value_of("file").unwrap_or("default"));
        let action: ActionType;
        let mut up_arguments: Vec<UpArgument> = Vec::new();
        let mut ps_arguments: Vec<PsArgument> = Vec::new();
        match arg_matches.subcommand(){
            ("up", Some(sub_m)) => {
                action = ActionType::Up;
                if sub_m.is_present("detach"){
                    up_arguments.push(UpArgument::Detach);
                }
                if sub_m.is_present("network"){
                    up_arguments.push(UpArgument::Network);
                }
            },
            ("down",_) => {
                action = ActionType::Down;
            },
            ("ps", Some(sub_m)) =>{
                action = ActionType::Ps;
                if sub_m.is_present("all"){
                    ps_arguments.push(PsArgument::All);
                }
            },
            ("stop", _) =>{
                action = ActionType::Stop;
            },
            (_,_) => {action = ActionType::Undefined;}
        };

        Config{
            filename,
            action,
            up_arguments,
            ps_arguments
        }
    }
    fn read_arguments()->clap::ArgMatches<'static>{
        App::new("Rodman-Compose")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("A tool for defining and running multi-container applications using Podman Pods")
            .settings(&[
                AppSettings::ColoredHelp,
                AppSettings::DisableHelpSubcommand,
                AppSettings::VersionlessSubcommands
            ])
            .subcommand(SubCommand::with_name("up")
                .about("Runs all containers in the composer file")
                .setting(AppSettings::ColoredHelp)
                .arg(Arg::with_name("detach")
                    .short("d")
                    .long("detach")
                    .help("Detach your containers from the console"))
                .arg(Arg::with_name("network")
                    .short("n")
                    .long("network")
                    .help("Use nework instead of pods. It also name the containers as its own service")))
            .subcommand(SubCommand::with_name("down")
                .about("Stops and deletes all containers in the composer file")
                .setting(AppSettings::ColoredHelp))
            .subcommand(SubCommand::with_name("stop")
                .about("Stops all containers in composer file")
                .setting(AppSettings::ColoredHelp))
            .subcommand(SubCommand::with_name("ps")
                .about("Shows the status of the containers in the composer file")
                .setting(AppSettings::ColoredHelp)
                .arg(Arg::with_name("all")
                    .short("a")
                    .long("all")
                    .help("Show all containers")))
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Your compose file. Default: docker-compose.{yml,yaml} or container-compose.{yml,yaml}")
                .takes_value(true))
            .get_matches()
    }
}