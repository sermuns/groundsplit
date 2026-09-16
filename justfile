render-hallway:
    cargo run -- examples/hallway-game.json -o out.mp4

watch:
    watchexec --exts rs -- just render-hallway
