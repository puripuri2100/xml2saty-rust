.PHONY: build test testwin

build:
	cargo build

test:
	target/debug/xml2saty -f t/t.xml -o t/t.saty -c t/t.x2s-config

testwin:
	target\debug\xml2saty.exe -f "t/t.xml" -o "t/t.saty" -c "t/t.x2s-config"