FROM rust:1 AS builder

WORKDIR /usr/src/website/

COPY ./ ./

RUN wget -O trunk.tar.gz https://github.com/trunk-rs/trunk/releases/download/v0.20.2/trunk-x86_64-unknown-linux-gnu.tar.gz && \
    tar -xzf trunk.tar.gz && \
    mv trunk /usr/bin/trunk
RUN make init
RUN make build

FROM nginx:stable AS server

COPY --from=builder /usr/src/website/dist/ /usr/share/nginx/html/

EXPOSE 80
