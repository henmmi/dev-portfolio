curl https://sh.rustup.rs -sSf | sh -s -- -y

cargo install --locked trunk

rustup target add wasm32-unknown-unknown

curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
mv tailwindcss-linux-x64 tailwindcss
chmod +x tailwindcss