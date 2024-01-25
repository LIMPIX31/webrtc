use std::collections::VecDeque;

use crate::packet::Packet;

/// control queue
pub type ControlQueue = VecDeque<Packet>;
