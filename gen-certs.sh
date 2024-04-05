#!/usr/bin/env bash

set -u
set -e
# set -x

OUT_DIR="certs"

create_ca() {
	local suffix="$1"
	openssl genpkey -algorithm ec -pkeyopt ec_paramgen_curve:P-256 -out "$OUT_DIR/ca$suffix.key"
	openssl req -key "$OUT_DIR/ca$suffix.key" -new -x509 -days 3650 -addext keyUsage=critical,keyCertSign,cRLSign -subj "/C=FR/CN=example.com/O=example" -out "$OUT_DIR/ca$suffix.crt"

	echo "created ca:"
	openssl x509 -text -noout -in "$OUT_DIR/ca$suffix.crt"
}

create_server() {
	local suffix="$1"
	local ca_id="$2"
	openssl genpkey -algorithm ec -pkeyopt ec_paramgen_curve:P-256 -out "$OUT_DIR/server$suffix.key"
	openssl req -key "$OUT_DIR/server$suffix.key" -config csr.cnf -new -out "$OUT_DIR/server$suffix.csr"
	openssl x509 -req -in "$OUT_DIR/server$suffix.csr" -CA "$OUT_DIR/ca$ca_id.crt" -CAkey "$OUT_DIR/ca$ca_id.key" \
		-CAcreateserial -days 3650 -extfile openssl.cnf -out "$OUT_DIR/server$suffix.pem"
	
	echo "created server certificate:"
	openssl x509 -text -noout -in "$OUT_DIR/server$suffix.pem"
}

mkdir -p $OUT_DIR
rm -I $OUT_DIR/*

create_ca 1
create_ca 2
create_server 1 1 # server 1 is signed by CA 1
create_server 2 1 # server 2 is signed by CA 1
create_server 3 2 # server 3 is signed by CA 2
