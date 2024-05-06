// use std::str::FromStr;
//
// use chrono::Local;
// use tracing::{
//     level_filters::LevelFilter, subscriber::set_global_default, Level,
// };
// use tracing_appender::non_blocking::WorkerGuard;
// use tracing_subscriber::{fmt, fmt::{format::Writer, time::FormatTime}, Layer, layer::SubscriberExt, Registry};
// use tracing_subscriber::fmt::writer::MakeWriterExt;
//
// use crate::library::cfg::AppConfig;
//
// struct LocalTimer;
//
// impl FormatTime for LocalTimer {
//     fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
//         write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
//     }
// }
//
// pub trait MyLayer<S: tracing::Subscriber>: Layer<S> + Send + Sync {}
// impl<S: tracing::Subscriber, L: Layer<S> + Send + Sync> MyLayer<S> for L {}
//
// struct RouterLayer<S> {
//     my_layer: Box<dyn MyLayer<S>>,
//     other_layer: Box<dyn MyLayer<S>>,
//     my_target: String,
// }
//
// impl<S> Layer<S> for RouterLayer<S>
//     where
//         S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
// {
//     fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
//         if event.metadata().target().starts_with(&self.my_target) {
//             self.my_layer.on_event(event, ctx);
//         } else {
//             self.other_layer.on_event(event, ctx);
//         }
//     }
// }
//
// pub fn init(cfg: &AppConfig) -> (WorkerGuard,WorkerGuard) {
//     let ((mine_non_blocking, mine_guard),
//         (other_non_blocking, other_guard), stdout) = {
//
//
//         let stdout = cfg.inpay.env == "dev";
//
//         let mine_appender = tracing_appender::non_blocking(
//             tracing_appender::rolling::daily(&cfg.log.path, &cfg.log.mine_file),
//         );
//
//         let other_appender = tracing_appender::non_blocking(
//             tracing_appender::rolling::daily(&cfg.log.path, &cfg.log.other_file),
//         );
//
//         (mine_appender, other_appender, stdout)
//     };
//     let mine_file_level_filter = LevelFilter::from(Level::from_str(&cfg.log.mine_file_level).unwrap_or(Level::INFO));
//     let mine_file_layer = fmt::layer()
//         .with_timer(LocalTimer)
//         // .with_filter(mine_file_level_filter)
//         .with_ansi(false)
//         .with_writer(mine_non_blocking)
//         .json()
//         .flatten_event(true);
//
//     let other_file_level_filter = LevelFilter::from(Level::from_str(&cfg.log.other_file_level).unwrap_or(Level::INFO));
//     let other_file_layer = fmt::layer()
//         .json()
//         .with_timer(LocalTimer)
//         // .with_filter(other_file_level_filter)
//         .with_ansi(false)
//         .with_writer(other_non_blocking)
//         .flatten_event(true);
//
//     let router_file_layer = RouterLayer {
//         my_layer: Box::new(mine_file_layer),
//         other_layer: Box::new(other_file_layer),
//         my_target: cfg.log.mine_target.to_string(),
//     };
//
//     let level = Level::from(Level::TRACE);
//     let level_filter = LevelFilter::from(level);
//
//     if stdout {
//
//         let mine_formatting_layer = fmt::layer()
//             .with_timer(LocalTimer)
//             // .with_filter(other_file_level_filter)
//             .pretty()
//             .with_writer(std::io::stderr)
//             .with_line_number(true);
//
//         let other_formatting_layer = fmt::layer()
//             .with_timer(LocalTimer)
//             // .with_filter(other_file_level_filter)
//             .pretty()
//             .with_writer(std::io::stderr)
//             .with_line_number(true);
//
//         // let router_formatting_layer = RouterLayer {
//         //     my_layer: Box::new(mine_formatting_layer),
//         //     other_layer: Box::new(other_formatting_layer),
//         //     my_target: cfg.log.mine_target.to_string(),
//         // };
//
//         let registry = Registry::default()
//             .with(level_filter)
//             .with(router_file_layer)
//             .with(mine_formatting_layer);
//
//         set_global_default(registry).unwrap_or_else(|e| {
//             panic!("💥 Failed to setting tracing subscriber: {e:?}");
//         });
//     } else {
//         let registry = Registry::default().with(router_file_layer).with(level_filter);
//
//         set_global_default(registry).unwrap_or_else(|e| {
//             panic!("💥 Failed to setting tracing subscriber: {e:?}");
//         });
//     }
//
//     (mine_guard,other_guard)
// }
//
use std::str::FromStr;

use chrono::Local;
use tracing::{
    level_filters::LevelFilter, subscriber::set_global_default, Level,
};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::{fmt, fmt::{format::Writer, time::FormatTime}, Layer, layer::SubscriberExt, Registry};
use crate::library::cfg::AppConfig;


struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub trait LogLayer<S: tracing::Subscriber>: Layer<S> + Send + Sync {}
impl<S: tracing::Subscriber, L: Layer<S> + Send + Sync> LogLayer<S> for L {}

struct RouterLayer<S> {
    mine_layer: Box<dyn LogLayer<S>>,
    database_layer: Box<dyn LogLayer<S>>,
    other_layer: Box<dyn LogLayer<S>>,
    mine_target: String,
    database_target: String,
}

impl<S> Layer<S> for RouterLayer<S>
    where
        S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if event.metadata().target().starts_with(&self.mine_target) {
            self.mine_layer.on_event(event, ctx);
        } else if event.metadata().target().starts_with(&self.database_target){
            self.database_layer.on_event(event, ctx);
        } else {
            self.other_layer.on_event(event, ctx);
        }
    }
}

// impl<S> RouterLayer<S> {
//     fn init(mine_non_blocking: NonBlocking, other_non_blocking:NonBlocking,cfg: &AppConfig) -> RouterLayer<S>{
//         let mine_file_layer = fmt::layer()
//             .with_timer(LocalTimer)
//             .with_ansi(false)
//             .with_writer(mine_non_blocking)
//             .json()
//             .flatten_event(true);
//
//         let other_file_layer = fmt::layer()
//             .json()
//             .with_timer(LocalTimer)
//             .with_ansi(false)
//             .with_writer(other_non_blocking)
//             .flatten_event(true);
//
//         Self {
//             mine_layer: Box::new(mine_file_layer),
//             other_layer: Box::new(other_file_layer),
//             mine_target: cfg.log.mine_target.to_string(),
//         }
//     }
//
// }

pub fn init(cfg: &AppConfig) -> (WorkerGuard,WorkerGuard,WorkerGuard) {
    let ((mine_non_blocking, mine_guard),
        (database_non_blocking, database_guard),
        (other_non_blocking, other_guard),
        stdout) = {

        let stdout = cfg.inpay.env == "dev";

        let mine_appender = tracing_appender::non_blocking(
            tracing_appender::rolling::daily(&cfg.log.path, &cfg.log.mine_file),
        );
        let database_appender = tracing_appender::non_blocking(
            tracing_appender::rolling::daily(&cfg.log.path, &cfg.log.database_file),
        );

        let other_appender = tracing_appender::non_blocking(
            tracing_appender::rolling::daily(&cfg.log.path, &cfg.log.other_file),
        );

        (mine_appender,database_appender, other_appender, stdout)
    };

    // let router_file_layer = RouterLayer::init(mine_non_blocking,other_non_blocking,cfg);
    let mine_file_layer = fmt::layer()
        .with_timer(LocalTimer)
        .with_ansi(false)
        .with_writer(mine_non_blocking)
        .json()
        .flatten_event(true);

    let database_file_layer = fmt::layer()
        .json()
        .with_timer(LocalTimer)
        .with_ansi(false)
        .with_writer(database_non_blocking)
        .flatten_event(true);

    let other_file_layer = fmt::layer()
        .json()
        .with_timer(LocalTimer)
        .with_ansi(false)
        .with_writer(other_non_blocking)
        .flatten_event(true);

    let router_file_layer = RouterLayer {
        mine_layer: Box::new(mine_file_layer),
        database_layer: Box::new(database_file_layer),
        other_layer: Box::new(other_file_layer),
        mine_target: cfg.log.mine_target.to_string(),
        database_target: cfg.log.database_target.to_string()
    };

    let level_file = Level::from_str(&cfg.log.mine_file_level).unwrap_or(Level::INFO);
    let level_formatting = Level::from_str(&cfg.log.mine_formatting_level).unwrap_or(Level::INFO);
    let level_file_filter = LevelFilter::from(level_file);
    let level_formatting_filter = LevelFilter::from(level_formatting);

    if stdout {
        let formatting_layer = fmt::layer()
            .with_timer(LocalTimer)
            .pretty()
            .with_writer(std::io::stderr)
            .with_line_number(true);

        let registry = Registry::default()
            .with(level_formatting_filter)
            .with(router_file_layer)
            .with(formatting_layer);

        set_global_default(registry).unwrap_or_else(|e| {
            panic!("💥 Failed to setting tracing subscriber: {e:?}");
        });
    } else {
        let registry = Registry::default().with(level_file_filter).with(router_file_layer);

        set_global_default(registry).unwrap_or_else(|e| {
            panic!("💥 Failed to setting tracing subscriber: {e:?}");
        });
    }

    (mine_guard,database_guard,other_guard)
}

