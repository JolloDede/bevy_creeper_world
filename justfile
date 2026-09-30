
run:
    RUST_LOG=error,bevy_creeper_world=debug cargo run

bundle:
    cargo build --release
    zip -r release.zip assets target/release/bevy_creeper_world
