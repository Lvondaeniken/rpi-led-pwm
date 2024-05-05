
.PHONY: .build deploy

.build: 
	cargo build --target=armv7-unknown-linux-gnueabihf --release

deploy: .build
	scp ./target/armv7-unknown-linux-gnueabihf/debug/rpi-led-pwm vonl@192.168.1.180:/home/vonl/Documents/

run: deploy
	ssh -i ~/.ssh/pi2.pub pi@pi2.local "~/Documents/rpi-led-pwm"
