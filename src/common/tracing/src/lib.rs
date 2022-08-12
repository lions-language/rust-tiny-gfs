// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod logging;
mod panic_hook;
mod tracing_to_jaeger;

pub use logging::init_default_ut_tracing;
pub use logging::init_global_tracing;
pub use logging::init_meta_ut_tracing;
pub use logging::init_query_logger;
pub use logging::init_tracing_log;
pub use panic_hook::set_panic_hook;
pub use tracing;
pub use tracing_appender;
pub use tracing_futures;
pub use tracing_subscriber;
pub use tracing_to_jaeger::extract_remote_span_as_parent;
pub use tracing_to_jaeger::inject_span_to_tonic_request;

#[macro_export]
macro_rules! func_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let n = &name[..name.len() - 3];
        let nn = n.replace("::{{closure}}", "");
        nn
    }};
}

static mut g_chunk_handler_log_subscriber: Option<
    std::sync::Arc<dyn tracing::Subscriber + Send + Sync>,
> = None;

// macro_rules! sub_info {
//     ($subscriber:expr, $format:expr, $($field:expr)*) => {
//         common_tracing::tracing::subscriber::with_default($subscriber.clone(), || {
//             info!($format, $($field)*);
//         });
//     };
//     ($subscriber:expr, $format:expr) => {
//         common_tracing::tracing::subscriber::with_default($subscriber.clone(), || {
//             info!($format);
//         });
//     };
// }

macro_rules! sub_info {
    ($format:expr, $($field:expr)*) => {
        unsafe {
            common_tracing::tracing::subscriber::with_default(g_chunk_handler_log_subscriber.as_ref().unwrap(), || {
                info!($format, $($field)*);
            });
        }
    };
    ($format:expr) => {
        unsafe {
            common_tracing::tracing::subscriber::with_default(g_chunk_handler_log_subscriber.as_ref().unwrap().clone(), || {
                info!($format);
            });
        }
    };
}

fn test() {
    let (_guards, mut subscriber) = init_tracing_log(
        "chunk_handler",
        "logs/chunk_handler",
        log::LevelFilter::Info.as_str(),
    );

    // g_chunk_handler_log_subscriber
    //     .as_mut()
    //     .replace(&mut subscriber);

    unsafe {
        g_chunk_handler_log_subscriber = Some(subscriber);
    }
}
