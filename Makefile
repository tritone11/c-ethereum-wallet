wallet: Makefile src/wallet.c src/wallet.h
	g++ -g src/wallet.c -o wallet ./dist/x86_64/librustwallet.so -lpthread -ldl -lssl -lcrypto -lusb-1.0


