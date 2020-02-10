.PHONY: build test testwin

build:
	cargo build

test:
	target/debug/xml2saty -f t/t.xml -o t/t.saty -c t/tconfig.json
	cd t && satysfi t.saty
	target/debug/xml2saty -f t/4.xml -o t/4.saty -c t/4.json
	cd t && satysfi 4.saty
	target/debug/xml2saty -f t/4.xml -o t/4.satyh -c t/4.json -p "T2,test"

testwin:
	target\debug\xml2saty.exe -f "t/t.xml" -o "t/t.saty" -c "t/tconfig.json"
	target\debug\xml2saty.exe -f t/4.xml -o t/4.saty -c t/4.json
	target\debug\xml2saty.exe -f t/4.xml -o t/4.satyh -c t/4.json -p "T2,test"