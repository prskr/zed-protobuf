# Protobuf LSP extension for [zed](https://zed.dev)

Zed natively supports [protobuf](https://zed.dev/docs/languages/proto) already but - so far - it only comes with syntax highlighting support for `.proto` files.
This extension adds support for the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP) for protobuf files by using [protobuf-language-server](https://github.com/lasorda/protobuf-language-server).

## LSP installation

Right now [protobuf-language-server](https://github.com/lasorda/protobuf-language-server) doesn't provide pre-built binaries, so you have to build it yourself.
This extension will **not** build the language server for you, so you have to do it manually (due to limitations of the extensions API it's not possible to run arbitrary commands as `go install`).
