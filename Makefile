init:
	@rustup target add wasm32-unknown-unknown
start:
	@trunk serve
build:
	@trunk build --release
docker_start:
	@docker run --name rby_yew --network host --rm -d rby/yew:latest
docker_build:
	@docker build -t rby/yew:latest .
