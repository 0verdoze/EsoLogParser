
//! Serializer and deserializer for
//! The Elder Scrolls Online encounter log format
//! 
//! # Example usage
//! ```
//! let data = fs::read_to_string(path).unwrap();
//! let events = Event::parse_many(&data)
//!     .collect::<Result<Vec<Event>, _>>()
//!     .unwrap();
//! ```


pub mod events;
pub mod state;

pub use events::*;
pub use state::*;

pub use eso_parser;

pub use events::Event;
pub use state::State;
