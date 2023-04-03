# Bring

A very simple port forwarder in rust, with no crate dependencies. Around 170-300kb unpacked, 70-130kb packed with `upx --best --lzma target/release/pull`.

Provide a remote host, remote port and optionally a local port to listen on (if not provided, uses the remote port) and that remote address will be re-exposed locally, just like with a ssh local port forward.

Useful when there is no ssh on the target machine, and you don't want to setup chisel or whatever (which is several magnitudes larger).