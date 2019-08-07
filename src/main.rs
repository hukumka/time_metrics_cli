use std::fs::File;
use std::io::{BufReader, BufRead};
use reformation::Reformation;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::time::Duration;
use std::env::args;
use structopt::StructOpt;
use std::path::PathBuf;
use std::io;

#[derive(Reformation, Debug, Clone)]
#[reformation("{name} started={start_micro} ended={end_micro}")]
struct Event{
    name: String,
    start_micro: u64,
    end_micro: u64,
}

struct Events{
    events: HashMap<String, Vec<Event>>,
    total_time_micros: u64
}

impl Events{
    /// Get total duration of system and fraction of its
    /// runtime compared to entire game
    fn get_system_times(&self) -> Vec<(&str, Duration, f64)>{
        let mut res: Vec<_> = self.events.iter()
            .map(|(k, v)|{
                let total: u64 = v.iter()
                    .map(|e| e.end_micro - e.start_micro)
                    .sum();
                let fraction = total as f64 / self.total_time_micros as f64;
                (k.as_str(), Duration::from_micros(total), fraction)
            }).collect();
        res.sort_by_key(|&(_, duration, _)| std::cmp::Reverse(duration));
        res
    }
}

impl FromIterator<Event> for Events{
    fn from_iter<I: IntoIterator<Item=Event>>(iter: I) -> Self{
        let mut map = HashMap::new();
        let mut total_time_micros = 0;
        for i in iter{
            let key = i.name.clone();
            total_time_micros = i.end_micro;
            map.entry(key).or_insert(vec![])
                .push(i);
        }
        Self{events: map, total_time_micros}
    }
}

#[derive(StructOpt)]
struct Opt{
    #[structopt(name="FILE", parse(from_os_str))]
    input: PathBuf,
}

fn main() -> io::Result<()>{
    let opt = Opt::from_args();
    let input = BufReader::new(File::open(&opt.input)?);
    let events: Events = input.lines()
        .map(|s| Event::parse(&s.unwrap()).unwrap())
        .collect();
    let res = events.get_system_times();
    for (name, total, frac) in res{
        println!("{}:  {:?};  {:.3}%;", name, total, frac*100.0);
    }
    Ok(())
}
