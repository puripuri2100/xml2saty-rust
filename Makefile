.PHONY: build test testwin install example exampletest

build:
	cargo build

install:
	cargo install --path .

example:
	xml2saty-rust -f example/gengou.xml -o example/gengou.saty -c example/law.json
	xml2saty-rust -f example/keihou.xml -o example/keihou.saty -c example/law.json
	satysfi example/gengou.saty -o example/gengou.pdf
	satysfi example/keihou.saty -o example/keihou.pdf

exampletest:
	target/debug/xml2saty-rust -f example/gengou.xml -o example/gengou.saty -c example/law.json
	target/debug/xml2saty-rust -f example/keihou.xml -o example/keihou.saty -c example/law.json
	satysfi example/gengou.saty -o example/gengou.pdf
	satysfi example/keihou.saty -o example/keihou.pdf

test:
	target/debug/xml2saty-rust -f t/t.xml -o t/t.saty -c t/tconfig.json
	cd t && satysfi t.saty
	target/debug/xml2saty-rust -f t/4.xml -o t/4.saty -c t/4.json
	cd t && satysfi 4.saty
	target/debug/xml2saty-rust -f t/4.xml -o t/4.satyh -c t/4.json -p "T2,test"

testwin:
	target\debug\xml2saty-rust.exe -f "t/t.xml" -o "t/t.saty" -c "t/tconfig.json"
	target\debug\xml2saty-rust.exe -f t/4.xml -o t/4.saty -c t/4.json
	target\debug\xml2saty-rust.exe -f t/4.xml -o t/4.satyh -c t/4.json -p "T2,test"

clean:
	@rm -rf target example/*.pdf example/*.satysfi-aux example/*.saty