#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[allow(unused_imports)]
use rayon::iter::ParallelIterator;
use std::{env::var_os, fs, time::Duration};

use eso_lib::*;

fn load_log() -> String {
    fs::read_to_string(
        var_os("BENCH_LOG_PATH").expect("BENCH_LOG_PATH enviroment variable not set")
    ).expect("log file does not exist")
}

fn benchmark_parse(c: &mut Criterion) {
    let data = load_log();

    c.bench_function("parse", |b| {
        b.iter(|| {
            Event::parse_many(&data)
                .for_each(|r| { black_box(r.unwrap()); })
        });
    });
}

fn benchmark_parse_state(c: &mut Criterion) {
    let data = load_log();

    c.bench_function("parse_handle_state", |b| {
        b.iter(|| {
            let state = &mut State::new();
    
            data.lines()
                .for_each(|line| {
                    black_box(handle_line(state, line));
                });
        });
    });
}

fn handle_line<'a>(state: &'a mut State, line: &str) -> (&'a State, Event) {
    let event = Event::parse(line).unwrap();
    state.handle_event(&event);

    (state, event)
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(25));
    targets = benchmark_parse, benchmark_parse_state
}

criterion_main!(benches);
