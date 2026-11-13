//! Whether a program's source needs the network or names a credential
//! store, however it reached the interpreter: a script file, or text fed on
//! standard input.

use super::{code_lines, python_network, Language, CREDENTIAL_STORE, NODE_NETWORK};

/// Whether `source` needs neither the network nor a credential store, the
/// bar a script launched in the workspace sandbox has to meet.
pub(in crate::pretool) fn source_clean(language: Language, source: &str) -> bool {
    let code = code_lines(language, source);
    let network = match language {
        Language::Python => python_network(&code),
        Language::Node => NODE_NETWORK.is_match(&code),
    };
    !network && !CREDENTIAL_STORE.is_match(&code)
}
