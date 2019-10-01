use libyaml_sys as sys;

/// Event type.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EventType {
    /// *STREAM-START* event.
    StreamStart = sys::YAML_STREAM_START_EVENT as _,

    /// *STREAM-END* event.
    StreamEnd = sys::YAML_STREAM_END_EVENT as _,

    /// *DOCUMENT-START* event.
    DocumentStart = sys::YAML_DOCUMENT_START_EVENT as _,

    /// *DOCUMENT-END* event.
    DocumentEnd = sys::YAML_DOCUMENT_END_EVENT as _,

    /// *ALIAS* event.
    Alias = sys::YAML_ALIAS_EVENT as _,

    /// *SCALAR* event.
    Scalar = sys::YAML_SCALAR_EVENT as _,

    /// *SEQUENCE-START* event.
    SequenceStart = sys::YAML_SEQUENCE_START_EVENT as _,

    /// *SEQUENCE-END* event.
    SequenceEnd = sys::YAML_SEQUENCE_END_EVENT as _,

    /// *MAPPING-START* event.
    MappingStart = sys::YAML_MAPPING_START_EVENT as _,

    /// *MAPPING-END* event.
    MappingEnd = sys::YAML_MAPPING_END_EVENT as _,
}

impl EventType {
    /// Wrap a raw `yaml_event_type_t`, returning `None` for unknown values.
    pub fn from_raw(raw: sys::yaml_event_type_t) -> Option<Self> {
        match raw {
            sys::YAML_STREAM_START_EVENT => Some(Self::StreamStart),
            sys::YAML_STREAM_END_EVENT => Some(Self::StreamEnd),
            sys::YAML_DOCUMENT_START_EVENT => Some(Self::DocumentStart),
            sys::YAML_DOCUMENT_END_EVENT => Some(Self::DocumentEnd),
            sys::YAML_ALIAS_EVENT => Some(Self::Alias),
            sys::YAML_SCALAR_EVENT => Some(Self::Scalar),
            sys::YAML_SEQUENCE_START_EVENT => Some(Self::SequenceStart),
            sys::YAML_SEQUENCE_END_EVENT => Some(Self::SequenceEnd),
            sys::YAML_MAPPING_START_EVENT => Some(Self::MappingStart),
            sys::YAML_MAPPING_END_EVENT => Some(Self::MappingEnd),
            _ => None,
        }
    }
}
