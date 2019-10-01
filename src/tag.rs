//! Tags for YAML language-independent types.
//!
//! The contents of this module are based on the list of language-independent
//! YAML tags defined under the `yaml.org` domain.  The list is available at
//! <https://yaml.org/type>.

/// Unordered set of `key: value` pairs without duplicates.
pub const MAP: &str = "tag:yaml.org,2002:map";

/// Ordered sequence of `key: value` pairs without duplicates.
pub const OMAP: &str = "tag:yaml.org,2002:omap";

/// Ordered sequence of `key: value` pairs allowing duplicates.
pub const PAIRS: &str = "tag:yaml.org,2002:pairs";

/// Unordered set of non-equal values.
pub const SET: &str = "tag:yaml.org,2002:set";

/// Sequence of arbitrary values.
pub const SEQ: &str = "tag:yaml.org,2002:seq";

/// A sequence of zero or more octets (8-bit values).
pub const BINARY: &str = "tag:yaml.org,2002:binary";

/// Mathematical booleans.
pub const BOOL: &str = "tag:yaml.org,2002:bool";

/// Floating-point approximation to real numbers.
pub const FLOAT: &str = "tag:yaml.org,2002:float";

/// Mathematical integers.
pub const INT: &str = "tag:yaml.org,2002:int";

/// Specify one or more mappings to be merged with the current one.
pub const MERGE: &str = "tag:yaml.org,2002:merge";

/// Devoid of value.
pub const NULL: &str = "tag:yaml.org,2002:null";

/// A sequence of zero or more Unicode characters.
pub const STR: &str = "tag:yaml.org,2002:str";

/// A point in time.
pub const TIMESTAMP: &str = "tag:yaml.org,2002:timestamp";

/// Specify the default value of a mapping.
pub const VALUE: &str = "tag:yaml.org,2002:value";

/// Keys for encoding YAML in YAML.
pub const YAML: &str = "tag:yaml.org,2002:yaml";
