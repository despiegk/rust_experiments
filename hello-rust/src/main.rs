use clap::Parser;




#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}


fn main() {
    let args = Args::parse();

    // println!("Searching for {}", query);
    // println!("In file {}", file_path);

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

    // Change file permissions (example, not related to search functionality)
    // Note: This is just an illustrative example. Changing file ownership requires more context.
    // fs::set_permissions("~/sandbox", fs::Permissions::from_mode(0o644)).expect("Error changing file permissions");
}
