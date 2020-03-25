FROM scratch

ADD target/x86_64-unknown-linux-musl/release/data-gen /

CMD ["/data-gen"]