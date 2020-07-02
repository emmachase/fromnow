use structopt::StructOpt;
use humantime::parse_duration;
use chrono::{Local, Duration};

#[derive(StructOpt, Debug)]
#[structopt(name = "fromnow")]
struct Opt {
    #[structopt(required = true)]
    durations: Vec<String>
}

fn main() {
    let opts: Opt = Opt::from_args();

    let full_duration = opts.durations.join(" ");
    let full_duration = parse_duration(&full_duration);

    let full_duration = if let Ok(x) = full_duration { x } else {
        eprintln!("'{}' is not a valid duration.", opts.durations.join(" "));
        eprintln!("Please use this format: '12d 23h 34m 45s'");
        return;
    };

    let full_duration = Duration::from_std(full_duration).unwrap();

    let future = Local::now() + full_duration;
    println!("{}", future.format("%Y %a %b %d %I:%M:%S %p %Z").to_string());
}
