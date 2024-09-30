use zed_extension_api::{self as zed, Result};

const DEFAULT_BINARY_NAME: &str = "protobuf-language-server";

struct ProtobufLspBinary {
    path: String,
}

struct ProtobufExtension {}

impl ProtobufExtension {
    fn language_server_binary(
        &self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<ProtobufLspBinary> {
        if let Some(path) = worktree.which(DEFAULT_BINARY_NAME) {
            return Ok(ProtobufLspBinary { path });
        }

        return Err(format!("{} not found in PATH", DEFAULT_BINARY_NAME));
    }
}

impl zed::Extension for ProtobufExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary.path,
            args: vec![String::from("-logs"), String::from("")],
            env: Default::default(),
        })
    }
}

zed::register_extension!(ProtobufExtension);
