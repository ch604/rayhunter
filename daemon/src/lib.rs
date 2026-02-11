use utoipa::OpenApi;
pub mod analysis;
pub mod battery;
pub mod config;
pub mod diag;
pub mod display;
pub mod error;
pub mod key_input;
pub mod notifications;
pub mod pcap;
pub mod qmdl_store;
pub mod server;
pub mod stats;

// Add anotated paths to api docs
#[derive(OpenApi)]
#[openapi(
    info(
        description = "OpenAPI documentation for Rayhunter daemon"
    ),
    paths(
        pcap::get_pcap,
        server::get_qmdl,
        server::get_zip,
        stats::get_system_stats,
        stats::get_qmdl_manifest,
        stats::get_log,
        diag::start_recording,
        diag::stop_recording,
        diag::delete_recording,
        diag::delete_all_recordings,
        diag::get_analysis_report,
        analysis::get_analysis_status,
        analysis::start_analysis,
        server::get_config,
        server::set_config,
        server::test_notification,
        server::get_time,
        server::set_time_offset,
        server::debug_set_display_state
    )
)]
pub struct ApiDocs;

impl ApiDocs {
    pub fn generate() -> String {
        ApiDocs::openapi().to_pretty_json().unwrap()
    }
}
