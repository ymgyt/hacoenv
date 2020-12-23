use hacoenv::cmd;

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            run().await;
        })
}

async fn run() {
    let app = cmd::initialize_from_args();

    init_logger(app.verbose);

    tracing::debug!("Command detail: {:?}", app);

    match app.subcommand {
        cmd::Subcommand::Inspect(opt) => {
            cmd::run_inspect(opt).await;
        }
        cmd::Subcommand::Fleet(fleet) => match fleet.subcommand {
            cmd::fleet::Subcommand::Init(opt) => {
                cmd::fleet::run_init(opt).await;
            }
            cmd::fleet::Subcommand::Inspect(opt) => {
                cmd::fleet::run_inspect(opt).await;
            }
        },
    }
}

// initialize logger.
// control the information to be logged by the value of verbose.
fn init_logger(verbose: u8) {
    use std::borrow::Cow;

    let mut display_target = false;
    let mut display_thread_ids = false;
    let mut env_filter: Cow<str> = "info".into();

    match verbose {
        0 => (), // use default
        1 => {
            env_filter = "hacoenv=debug".into();
        }
        2 => {
            display_target = true;
            env_filter = "hacoenv=trace".into();
        }
        _ => {
            display_target = true;
            display_thread_ids = true;
            env_filter = "trace".into();
        }
    }

    // if user explicitly specify log directive, respect it.
    if let Some(directive) = std::env::var("HACOENV_LOG").ok() {
        env_filter = Cow::Owned(directive);
    }

    tracing_subscriber::fmt()
        .with_target(display_target)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc3339())
        .with_thread_ids(display_thread_ids)
        .with_env_filter(env_filter)
        .init();
}
