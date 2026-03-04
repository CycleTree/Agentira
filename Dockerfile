FROM rust:latest

# wasm32ターゲット追加
RUN rustup target add wasm32-unknown-unknown

# trunk インストール
RUN cargo install trunk --locked

# 作業ディレクトリ
WORKDIR /app

# 依存関係キャッシュ用
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# ソースコピー
COPY . .

# ビルド
RUN cargo build --release

EXPOSE 8080
CMD ["trunk", "serve", "--address", "0.0.0.0"]
