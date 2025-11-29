.PHONY: build clean install install-service

build:
	cargo build --release

clean:
	cargo clean

install: build
	install -m 755 target/release/pi5-fan-control /usr/local/sbin/rpi-fan-control

install-service:
	install -m 644 fan-control.service /etc/systemd/system/rpi-fan-control.service
	sed -i 's|ExecStart=.*|ExecStart=/usr/local/sbin/rpi-fan-control --daemon|g' /etc/systemd/system/rpi-fan-control.service
	systemctl daemon-reload
