run:
    RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve --port 8021 --release

build:
    RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build --release

deploy:
    sudo cp -r ./ /opt/bed/
    sudo cp bed.service /etc/systemd/system/bed.service
    sudo systemctl daemon-reload
    sudo systemctl enable bed.service --now

