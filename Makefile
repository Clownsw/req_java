all:
	cargo build
	sudo cp /home/smile/works/req_java/target/debug/libreq_java.so /usr/lib/libreq_java.so 
