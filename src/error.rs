//! Error types for physdes-rs
//!
//! This module provides comprehensive error handling for geometric operations.

use std::fmt;

/// Error type for geometric operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeomError {
    /// Invalid polygon construction
    InvalidPolygon(String),

    /// Invalid interval (lower bound > upper bound)
    InvalidInterval { lower: i64, upper: i64 },

    /// Self-intersecting polygon
    SelfIntersectingPolygon,

    /// Degenerate polygon (too few vertices)
    DegeneratePolygon { vertex_count: usize },

    /// Numerical stability issue
    NumericalError(String),

    /// Invalid point
    InvalidPoint(String),
}

impl fmt::Display for GeomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeomError::InvalidPolygon(msg) => write!(f, "Invalid polygon: {}", msg),
            GeomError::InvalidInterval { lower, upper } => {
                write!(
                    f,
                    "Invalid interval: lower bound {} > upper bound {}",
                    lower, upper
                )
            }
            GeomError::SelfIntersectingPolygon => {
                write!(f, "Self-intersecting polygon not allowed")
            }
            GeomError::DegeneratePolygon { vertex_count } => {
                write!(
                    f,
                    "Degenerate polygon: insufficient vertices ({} vertices, minimum 3 required)",
                    vertex_count
                )
            }
            GeomError::NumericalError(msg) => write!(f, "Numerical error: {}", msg),
            GeomError::InvalidPoint(msg) => write!(f, "Invalid point: {}", msg),
        }
    }
}

impl std::error::Error for GeomError {}

/// Result type for geometric operations
pub type GeomResult<T> = Result<T, GeomError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = GeomError::InvalidPolygon("test message".to_string());
        assert_eq!(format!("{}", err), "Invalid polygon: test message");

        let err = GeomError::InvalidInterval {
            lower: 10,
            upper: 5,
        };
        assert_eq!(
            format!("{}", err),
            "Invalid interval: lower bound 10 > upper bound 5"
        );

        let err = GeomError::SelfIntersectingPolygon;
        assert_eq!(format!("{}", err), "Self-intersecting polygon not allowed");

        let err = GeomError::DegeneratePolygon { vertex_count: 2 };
        assert_eq!(
            format!("{}", err),
            "Degenerate polygon: insufficient vertices (2 vertices, minimum 3 required)"
        );
    }
}
