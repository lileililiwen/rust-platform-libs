//! Language-neutral platform contract envelopes, payload types, and
//! the compatibility validator.
//!
//! This crate is the Rust side of the `platform-contracts` repository.
//! It does not own the schemas (the language-neutral repository is the
//! single source of truth) and it does not own any persistence, HTTP,
//! or provider integration. The application is responsible for keeping
//! a copy of, or a checkout of, the `platform-contracts` repository
//! reachable; see [`registry::ContractRegistry::load`] and the
//! [`validate`] module for the runtime contract.
//!
//! The foundation rule "match language-neutral contracts" is enforced
//! by reading the JSON Schemas published by the `platform-contracts`
//! repository and validating every payload against them. Tests under
//! `tests/` point at the live repository via the `PLATFORM_CONTRACTS_DIR`
//! environment variable, or fall back to `../platform-contracts`
//! relative to the workspace root. If neither is reachable the test
//! fails with a clear, single next-action message; the foundation does
//! not silently pass on missing cross-repository evidence.

#![forbid(unsafe_code)]

pub mod envelope;
pub mod payload;
pub mod registry;
pub mod validate;

pub use envelope::{parse_contract_identifier, ContractIdError, ContractVersion, Envelope};
pub use payload::{
    AuditActor, AuditCategory, AuditEvent, AuditOutcome, Authentication, AuthenticationMethod,
    Environment, IdentitySubject, IsolationLevel, PermissionDecision, PermissionReason, Readiness,
    ReadinessCheck, ReadinessStatus, SubjectType, TenantContext,
};
pub use registry::{ContractEntry, ContractRegistry, RegistryError};
pub use validate::{
    validate_document, validate_payload, ValidationError, ValidationResult, ValidationWarning,
};
