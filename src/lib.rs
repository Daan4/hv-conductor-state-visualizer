//! High Voltage Network Visualizer
//! See README.md @ https://github.com/Daan4/hv-network-visualizer

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

/// CLI interface to interact with the component, node and network APIs
pub mod cli;
/// Component API
pub mod component;
/// Measurement struct
pub mod measurement;
/// Network API
pub mod network;
/// Node API
pub mod node;
/// Switchgear Position struct
pub mod position;
/// Component Terminal struct
pub mod terminal;
