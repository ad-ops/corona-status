use std::collections::HashMap;
use std::error::Error;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The country pattern to list cases.
    #[structopt(short, long, default_value = "")]
    country: String,
    /// The number of days ago to compare todays numbers.
    #[structopt(short, long, default_value = "3")]
    days_ago: usize,
}

fn run(query: &str, _days_ago: usize) -> Result<(), Box<dyn Error>> {
    let rdr = csv::Reader::from_path("./data/confirmed.csv");
    let mut todays_cases: HashMap<String, (i32, i32, i32, i32, i32, i32)> = HashMap::new();
    for result in rdr?.records() {
        let record = result?;
        let country = record[1].to_string();
        let todays_number = record[record.len() - 1].parse::<i32>().unwrap();
        let one_days_ago_number = record[record.len() - 2].parse::<i32>().unwrap();
        let two_days_ago_number = record[record.len() - 3].parse::<i32>().unwrap();
        let three_days_ago_number = record[record.len() - 4].parse::<i32>().unwrap();
        let four_days_ago_number = record[record.len() - 5].parse::<i32>().unwrap();
        let five_days_ago_number = record[record.len() - 6].parse::<i32>().unwrap();
        let case = todays_cases.entry(country).or_insert((0, 0, 0, 0, 0, 0));
        case.0 += five_days_ago_number;
        case.1 += four_days_ago_number;
        case.2 += three_days_ago_number;
        case.3 += two_days_ago_number;
        case.4 += one_days_ago_number;
        case.5 += todays_number;
    }
    let selected = query.to_lowercase();
    println!(
        "{:<35} {:>5} {:>5}( +% ) {:>5}( +% ) {:>5}( +% ) {:>5}( +% ) {:>5}( +% )",
        "Country", "D-5", "D-4", "D-3", "D-2", "D-1", "Today",
    );
    println!("{}", "-".repeat(100));
    let _ = todays_cases
        .keys()
        .filter(|key| key.to_lowercase().starts_with(&selected))
        .for_each(|key| {
            println!(
                "{:<35} {:>5} {:>5}({:>3}%) {:>5}({:>3}%) {:>5}({:>3}%) {:>5}({:>3}%) {:>5}({:>3}%)",
                key,
                todays_cases[key].0,
                todays_cases[key].1,
                percent(todays_cases[key].0, todays_cases[key].1),
                todays_cases[key].2,
                percent(todays_cases[key].1, todays_cases[key].2),
                todays_cases[key].3,
                percent(todays_cases[key].2, todays_cases[key].3),
                todays_cases[key].4,
                percent(todays_cases[key].3, todays_cases[key].4),
                todays_cases[key].5,
                percent(todays_cases[key].4, todays_cases[key].5),
            )
        });
    Ok(())
}



fn percent(old: i32, new: i32) -> i32 {
    if old > 0 {
        (new - old) * 100 / old
    } else {
        0
    }
}

fn main() {
    let args = Cli::from_args();
    if let Err(err) = run(&args.country, args.days_ago) {
        println!("error running example: {}", err);
        process::exit(1);
    }

}
