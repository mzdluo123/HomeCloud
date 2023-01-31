export CROSS_CONTAINER_OPTS="-v /home/rainchan/HCG/build_lib/libpcap/build-release/:/libpcap -e LIBPCAP_LIBDIR=/libpcap"
cross build --release --target aarch64-unknown-linux-musl