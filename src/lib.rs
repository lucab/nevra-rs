//! Library for parsing and comparing RPM version labels (`NEVRA`).
//!
//! RPM package/version labels are composed of five components:
//!  * Name
//!  * Epoch
//!  * Version
//!  * Release
//!  * Architecture
//!
//! # Example
//! ```rust
//! use nevra::{PackageVersion, Version};
//!
//! let pkgname = "cargo";
//! let pkgver = "1:1.30.0-f29.aarch64";
//! let pkgnamever = format!("{}-{}", pkgname, pkgver);
//!
//! let nevra = PackageVersion::parse(pkgnamever).unwrap();
//! let evra = Version::parse(pkgver).unwrap();
//!
//! assert_eq!(nevra.evra(), &evra);
//! ```

#[macro_use]
extern crate error_chain;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fmt;

pub mod errors;
mod parse;
use parse::{NevraParser, Rule};

/// A `NEVRA` package name and version.
#[derive(Clone, Debug, PartialEq)]
pub struct PackageVersion {
    pub(crate) name: String,
    pub(crate) evra: Version,
}

impl PackageVersion {
    /// Construct a `PackageVersion`.
    pub fn new<S: AsRef<str>>(
        name: S,
        epoch: Option<String>,
        version: S,
        release: Option<String>,
        architecture: Option<String>,
    ) -> errors::Result<Self> {
        ensure!(!name.as_ref().is_empty(), "empty name");
        ensure!(!version.as_ref().is_empty(), "empty version");

        let mut label = name.as_ref().to_string();
        if let Some(e) = epoch {
            label.push_str(&e);
            label.push(':');
        }
        label.push_str(version.as_ref());
        if let Some(r) = release {
            label.push('-');
            label.push_str(&r);
        }
        if let Some(a) = architecture {
            label.push('.');
            label.push_str(&a);
        }

        Self::parse(&label)
    }

    /// Package name
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Package EVRA.
    #[inline(always)]
    pub fn evra(&self) -> &Version {
        &self.evra
    }

    /// Package epoch.
    #[inline(always)]
    pub fn epoch(&self) -> &Option<String> {
        &self.evra.epoch()
    }

    /// Package version.
    #[inline(always)]
    pub fn version(&self) -> &str {
        self.evra.version()
    }

    /// Package release.
    #[inline(always)]
    pub fn release(&self) -> &Option<String> {
        self.evra.release()
    }

    /// Package architecture
    #[inline(always)]
    pub fn architecture(&self) -> &Option<String> {
        self.evra.architecture()
    }

    /// Parse a NEVRA label.
    pub fn parse<S: AsRef<str>>(label: S) -> errors::Result<Self> {
        use errors::ResultExt;
        use pest::Parser;

        // Parse label.
        let mut rules = NevraParser::parse(Rule::nevra_input, label.as_ref())
            .chain_err(|| "NEVRA parsing error")?;

        // Temporary fields.
        let mut name = String::new();
        let mut evra = Version {
            epoch: None,
            version: String::new(),
            release: None,
            architecture: None,
        };

        // Walk tokens.
        for field in rules.next().unwrap().into_inner() {
            match field.as_rule() {
                Rule::name => {
                    name = field.as_str().to_string();
                }
                Rule::evra => {
                    evra = Version::parse_evra_rule(field)?;
                }
                Rule::EOI => {}
                _ => {
                    bail!("NEVRA, invalid token rule: {}", field.to_string());
                }
            }
        }

        // Contruct the NEVRA.
        let nevra = Self { name, evra };
        Ok(nevra)
    }
}

impl fmt::Display for PackageVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.evra())
    }
}

/// An `EVRA` package version.
#[derive(Clone, Debug, PartialEq)]
pub struct Version {
    /// Package epoch.
    pub(crate) epoch: Option<String>,
    /// Package version.
    pub(crate) version: String,
    /// Package release.
    pub(crate) release: Option<String>,
    /// Package architecture.
    pub(crate) architecture: Option<String>,
}

impl Version {
    /// Construct a `Version`.
    pub fn new<S: AsRef<str>>(
        epoch: Option<String>,
        version: S,
        release: Option<String>,
        architecture: Option<String>,
    ) -> errors::Result<Self> {
        ensure!(!version.as_ref().is_empty(), "empty version");

        let mut label = String::new();
        if let Some(e) = epoch {
            label.push_str(&e);
            label.push(':');
        }
        label.push_str(version.as_ref());
        if let Some(r) = release {
            label.push('-');
            label.push_str(&r);
        }
        if let Some(a) = architecture {
            label.push('.');
            label.push_str(&a);
        }

        Self::parse(&label)
    }

    /// Package epoch.
    pub fn epoch(&self) -> &Option<String> {
        &self.epoch
    }

    /// Package version.
    pub fn version(&self) -> &str {
        self.version.as_str()
    }

    /// Package release.
    pub fn release(&self) -> &Option<String> {
        &self.release
    }

    /// Package architecture
    pub fn architecture(&self) -> &Option<String> {
        &self.architecture
    }

    /// Parse an EVRA label.
    pub fn parse<S: AsRef<str>>(label: S) -> errors::Result<Self> {
        use errors::ResultExt;
        use pest::Parser;

        let mut rules = NevraParser::parse(Rule::evra_input, label.as_ref())
            .chain_err(|| "EVRA parsing error")?;

        // Temporary fields.
        let mut evra = Version {
            epoch: None,
            version: String::new(),
            release: None,
            architecture: None,
        };

        // Walk tokens.
        for field in rules.next().unwrap().into_inner() {
            match field.as_rule() {
                parse::Rule::evra => {
                    evra = Version::parse_evra_rule(field).unwrap();
                }
                parse::Rule::EOI => {}
                _ => {
                    bail!("EVRA, invalid token rule: {}", field.to_string());
                }
            }
        }
        Ok(evra)
    }

    /// Parse tokenized evra label.
    fn parse_evra_rule(rule: pest::iterators::Pair<'_, parse::Rule>) -> errors::Result<Self> {
        // Temporary fields.
        let mut evra = Self {
            epoch: None,
            version: String::new(),
            release: None,
            architecture: None,
        };

        // Walk tokens.
        for field in rule.into_inner() {
            match field.as_rule() {
                Rule::epoch => {
                    evra.epoch = Some(field.as_str().to_string());
                }
                Rule::version => {
                    evra.version = field.as_str().to_string();
                }
                Rule::release => {
                    evra.release = Some(field.as_str().to_string());
                }
                Rule::architecture => {
                    evra.architecture = Some(field.as_str().to_string());
                }
                _ => {
                    bail!("EVRA label, invalid token rule: {}", field.to_string());
                }
            }
        }
        Ok(evra)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        if let Some(ref e) = self.epoch {
            buf.push_str(e);
            buf.push(':');
        }
        buf.push_str(&self.version);
        if let Some(ref r) = self.release {
            buf.push('-');
            buf.push_str(r);
        }
        if let Some(ref a) = self.architecture {
            buf.push('.');
            buf.push_str(a);
        }
        write!(f, "{}", buf)
    }
}
