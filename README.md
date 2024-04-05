# tonic-tls-poc

```
./gen-certs.sh
cargo build --release
./target/release/server  --cert-file certs/server1.pem --key-file certs/server1.key --ca-list certs/ca1.crt certs/ca2.crt

# in a different window
./target/release/client --cert-file certs/server2.pem --key-file certs/server2.key --ca-file certs/ca1.crt
./target/release/client --cert-file certs/server3.pem --key-file certs/server3.key --ca-file certs/ca1.crt

```
