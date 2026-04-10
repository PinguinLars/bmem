FROM fedora:latest

RUN dnf update -y
RUN dnf install mingw64-qt6-qtbase mingw64-qt6-qtdeclarative mingw64-gcc-c++ zig rustup mold -y
RUN rustup-init -y --profile minimal
RUN ~/.cargo/bin/rustup target add x86_64-pc-windows-msvc
RUN ~/.cargo/bin/cargo install cargo-zigbuild

COPY . /app
WORKDIR /app

RUN ~/.cargo/bin/cargo build --release
RUN ~/.cargo/bin/cargo zigbuild --release --target x86_64-pc-windows-msvc

ENTRYPOINT ["/bin/bash"]
