//#![feature(unboxed_closures)]
//#![feature(fn_traits)]
use std::time::Duration;

use tokio::{sync::mpsc::Receiver, time::timeout};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter::Directive, fmt, EnvFilter};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // set_up_logging(log::LevelFilter::Trace);

    // We create a pipe which will create blocking behavior
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    // Perform a nonblocking read on the socket
    eprintln!("step-1",);
    read_again(&mut rx).await;

    // Write some data and make sure its reading correctly
    eprintln!("step-2",);
    timeout(Duration::from_secs(1), tx.send(vec![1u8; 128]))
        .await
        .unwrap()
        .unwrap();

    eprintln!("step-3",);
    read_compare(&mut rx, &[1u8; 128]).await;

    eprintln!("step-4",);
    read_again(&mut rx).await;

    // Read the data (which should not fail as we are blocking)
    eprintln!("step-5",);
    tokio::join! {
        async {
            eprintln!("step-A1");
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;

            eprintln!("step-A2");
            read_compare(&mut rx, &[2u8; 128]).await;

            eprintln!("step-A3");
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;

            eprintln!("step-A4");
            read_compare(&mut rx, &[3u8; 128]).await;

            eprintln!("step-A5");
            read_again(&mut rx).await;

            eprintln!("step-A6");
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;

            eprintln!("step-A7");
            read_again(&mut rx).await;

            eprintln!("step-A8");
        },
        async {
            println!("step-B1");
            tx.send(vec![2u8; 128]).await.unwrap();

            println!("step-B2");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            println!("step-B3");
            tx.send(vec![3u8; 128]).await.unwrap();

            println!("step-B4");
        }
    };

    // We are good
    eprintln!("all done");
}

pub async fn read_again(rx: &mut Receiver<Vec<u8>>) {
    match rx.try_recv() {
        Err(err) => {
            eprintln!("received ErrorKind::WouldBlock - {}", err);
        }
        ret => {
            panic!("expected ErrorKind::WouldBlock but received {:?}", ret);
        }
    }
}

pub async fn read_compare(rx: &mut Receiver<Vec<u8>>, data: &[u8]) {
    let ret = timeout(Duration::from_secs(1), rx.recv()).await;
    match ret {
        Ok(Some(test_buf)) => {
            assert_eq!(test_buf.len(), data.len());
            if data.iter().zip(test_buf.iter()).all(|(a, b)| *a == *b) == false {
                panic!("data does not match");
            }
            eprintln!("data match verified (len={})", data.len());
        }
        ret => {
            panic!("expected data(len={}) but received {:?}", data.len(), ret);
        }
    }
}

/// Subroutine to instantiate the loggers
pub fn set_up_logging(level: log::LevelFilter) {
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_span_events(fmt::format::FmtSpan::CLOSE)
        .with_ansi(true)
        .with_thread_ids(false)
        .with_writer(std::io::stderr)
        .compact();

    let filter_layer = EnvFilter::builder()
        .with_default_directive(log_directive(level))
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}

fn log_directive(level: log::LevelFilter) -> Directive {
    let tracing_level = match level {
        log::LevelFilter::Off => tracing::level_filters::LevelFilter::OFF,
        log::LevelFilter::Error => tracing::level_filters::LevelFilter::ERROR,
        log::LevelFilter::Warn => tracing::level_filters::LevelFilter::WARN,
        log::LevelFilter::Info => tracing::level_filters::LevelFilter::INFO,
        log::LevelFilter::Debug => tracing::level_filters::LevelFilter::DEBUG,
        log::LevelFilter::Trace => tracing::level_filters::LevelFilter::TRACE,
    };

    tracing_level.into()
}
