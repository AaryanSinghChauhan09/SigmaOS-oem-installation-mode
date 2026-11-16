// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-oem-installation-mode: oem installation mode component model.
//!
//! Real implementation: modes, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mode {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OemInstallationModeError {
    EmptyModeName,
    DuplicateMode(String),
    EmptyEventSubject,
}

impl fmt::Display for OemInstallationModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyModeName => write!(f, "mode name must not be empty"),
            Self::DuplicateMode(id) => write!(f, "duplicate mode {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for OemInstallationModeError {}

pub struct OemInstallationMode {
    pub name: String,
    modes: BTreeMap<String, Mode>,
    events: BTreeMap<String, Event>,
}

impl OemInstallationMode {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            modes: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_mode(&mut self, item: Mode) -> Result<(), OemInstallationModeError> {
        if item.name.trim().is_empty() {
            return Err(OemInstallationModeError::EmptyModeName);
        }
        let id = item.id.clone();
        if self.modes.contains_key(&id) {
            return Err(OemInstallationModeError::DuplicateMode(id.clone()));
        }
        self.modes.insert(id, item);
        Ok(())
    }

    pub fn mode_count(&self) -> usize {
        self.modes.len()
    }

    pub fn get_mode(&self, id: &str) -> Option<&Mode> {
        self.modes.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), OemInstallationModeError> {
        if event.subject.trim().is_empty() {
            return Err(OemInstallationModeError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_mode(id: &str, name: &str) -> Mode {
        Mode {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = OemInstallationMode::new("OemInstallationMode");
        assert_eq!(m.name, "OemInstallationMode");
        assert_eq!(m.mode_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_mode() {
        let mut m = OemInstallationMode::new("OemInstallationMode");
        m.add_mode(sample_mode("a1", "Alpha")).unwrap();
        assert_eq!(m.mode_count(), 1);
        assert!(m.get_mode("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = OemInstallationMode::new("OemInstallationMode");
        assert!(m.add_mode(sample_mode("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = OemInstallationMode::new("OemInstallationMode");
        m.add_mode(sample_mode("x", "One")).unwrap();
        assert!(m.add_mode(sample_mode("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = OemInstallationMode::new("OemInstallationMode");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
