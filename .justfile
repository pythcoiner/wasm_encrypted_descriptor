run:
    RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve --address 0.0.0.0 --port 8021 --release

build:
    RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build --release

deploy:
    id -u bed > /dev/null 2>&1 || sudo useradd bed
    sudo cp -r ./ /opt/bed/
    sudo chown -R bed:bed /opt/bed/
    sudo cp bed.service /etc/systemd/system/bed.service
    sudo systemctl daemon-reload
    sudo systemctl enable bed.service --now

