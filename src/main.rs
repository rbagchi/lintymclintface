use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use clap::Parser;
use std::fs;
use tracing::{info, error};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use prometheus::{Encoder, TextEncoder, Gauge, Counter, Opts, Registry, IntCounterVec};
use lazy_static::lazy_static;
use std::time::Instant;

mod linters;
use lintymclintface::{SyntaxError, LinterError}; // Import from the library

#[derive(Deserialize)]
struct LintRequest {
    language: String,
    code: String,
}

// --- Prometheus Metrics --- 
lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref LINT_REQUESTS_TOTAL: Counter = Counter::new(
        "lint_requests_total",
        "Total number of linting requests."
    ).unwrap();
    pub static ref LINT_REQUESTS_BY_LANGUAGE: IntCounterVec = IntCounterVec::new(
        Opts::new("lint_requests_by_language", "Total number of linting requests by language."),
        &["language"]
    ).unwrap();
    pub static ref LINT_DURATION_SECONDS: Gauge = Gauge::new(
        "lint_duration_seconds",
        "Duration of linting requests in seconds."
    ).unwrap();
    pub static ref LINT_ERRORS_TOTAL: Counter = Counter::new(
        "lint_errors_total",
        "Total number of linting errors found."
    ).unwrap();
}

fn register_metrics() {
    REGISTRY.register(Box::new(LINT_REQUESTS_TOTAL.clone())).unwrap();
    REGISTRY.register(Box::new(LINT_REQUESTS_BY_LANGUAGE.clone())).unwrap();
    REGISTRY.register(Box::new(LINT_DURATION_SECONDS.clone())).unwrap();
    REGISTRY.register(Box::new(LINT_ERRORS_TOTAL.clone())).unwrap();
}

async fn lint_service(req: web::Json<LintRequest>) -> impl Responder {
    LINT_REQUESTS_TOTAL.inc();
    LINT_REQUESTS_BY_LANGUAGE.with_label_values(&[&req.language]).inc();

    let start_time = Instant::now();

    let result = match req.language.as_str() {
        "java" => linters::java::lint(&req.code),
        "python" => linters::python::lint(&req.code),
        "r" => linters::r::lint(&req.code),
        _ => Err(LinterError::UnsupportedLanguage(req.language.clone())),
    };

    let duration = start_time.elapsed().as_secs_f64();
    LINT_DURATION_SECONDS.set(duration);

    match result {
        Ok(errors) => {
            LINT_ERRORS_TOTAL.inc_by(errors.len() as f64);
            web::Json(errors)
        },
        Err(e) => {
            LINT_ERRORS_TOTAL.inc(); // Increment for linter errors themselves
            let error = match e {
                LinterError::Io(msg) => SyntaxError { line: 0, column: 0, message: msg },
                LinterError::Parse(msg) => SyntaxError { line: 0, column: 0, message: msg },
                LinterError::TreeSitterParseError(msg) => SyntaxError { line: 0, column: 0, message: msg },
                LinterError::UnsupportedLanguage(msg) => SyntaxError { line: 0, column: 0, message: format!("Unsupported language: {}", msg) },
            };
            web::Json(vec![error])
        }
    }
}

// New handler for /metrics endpoint
async fn metrics() -> impl Responder {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Language to lint (java, python, r)
    #[arg(short, long)]
    language: Option<String>,

    /// Path to the file to lint
    #[arg(short, long)]
    file: Option<String>,

    /// Start as a web service
    #[arg(short, long)]
    service: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr) // Direct logs to stderr
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    register_metrics(); // Register Prometheus metrics

    let cli = Cli::parse();

    if cli.service || (cli.language.is_none() && cli.file.is_none()) {
        info!("Starting lintymclintface in web service mode on 127.0.0.1:8080");
        HttpServer::new(|| {
            App::new()
                .route("/lint", web::post().to(lint_service))
                .route("/metrics", web::get().to(metrics)) // Add metrics endpoint
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    } else if let (Some(language), Some(file_path)) = (cli.language, cli.file) {
        info!("Linting {} file: {}", language, file_path);
        let code = fs::read_to_string(&file_path)
            .map_err(|e| LinterError::Io(format!("Failed to read file: {}", e)))?;

        let result = match language.as_str() {
            "java" => linters::java::lint(&code),
            "python" => linters::python::lint(&code),
            "r" => linters::r::lint(&code),
            _ => Err(LinterError::UnsupportedLanguage(language.clone())),
        };

        match result {
            Ok(errors) => {
                if errors.is_empty() {
                    info!("No syntax errors found.");
                } else {
                    // Serialize errors to JSON and print to stdout
                    let json_errors = serde_json::to_string_pretty(&errors)
                        .map_err(|e| LinterError::Io(format!("Failed to serialize errors to JSON: {}", e)))?;
                    println!("{}", json_errors);
                }
            },
            Err(e) => {
                error!("Linter error: {:?}", e);
                // For CLI, also print a structured error if possible
                let error_msg = match e {
                    LinterError::Io(msg) => format!("IO Error: {}", msg),
                    LinterError::Parse(msg) => format!("Parse Error: {}", msg),
                    LinterError::TreeSitterParseError(msg) => format!("Tree-sitter Parse Error: {}", msg),
                    LinterError::UnsupportedLanguage(msg) => format!("Unsupported Language Error: {}", msg),
                };
                let structured_error = serde_json::to_string_pretty(&vec![SyntaxError { line: 0, column: 0, message: error_msg }])
                    .map_err(|e| LinterError::Io(format!("Failed to serialize error to JSON: {}", e)))?;
                eprintln!("{}", structured_error);
                std::process::exit(1);
            }
        }
        Ok(())
    } else {
        error!("Usage: lintymclintface --language <lang> --file <path> or lintymclintface --service");
        Ok(())
    }
}