pub mod track_local;
pub mod track_remote;

use std::sync::Arc;

use interceptor::stream_info::StreamInfo;
use interceptor::{RTCPReader, RTPReader};
use track_remote::*;

pub const RTP_OUTBOUND_MTU: usize = 1200;
pub const RTP_PAYLOAD_TYPE_BITMASK: u8 = 0x7F;

#[derive(Clone)]
pub struct TrackStream {
    pub stream_info: Option<StreamInfo>,
    pub rtp_read_stream: Option<Arc<srtp::stream::Stream>>,
    pub rtp_interceptor: Option<Arc<dyn RTPReader + Send + Sync>>,
    pub rtcp_read_stream: Option<Arc<srtp::stream::Stream>>,
    pub rtcp_interceptor: Option<Arc<dyn RTCPReader + Send + Sync>>,
}

/// TrackStreams maintains a mapping of RTP/RTCP streams to a specific track
/// a RTPReceiver may contain multiple streams if we are dealing with Simulcast
#[derive(Clone)]
pub struct TrackStreams {
    pub track: Arc<TrackRemote>,
    pub stream: TrackStream,
    pub repair_stream: TrackStream,
}
