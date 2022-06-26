use std::io::Result;

fn main() -> Result<()> {
    let out_dir = "src/proto";

    let _ = std::fs::create_dir(out_dir);

    tonic_build::configure()
        .build_server(true)
        .out_dir(out_dir)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["chunk_handler_service.proto"], &["../protos"])?;

    Ok(())
}
