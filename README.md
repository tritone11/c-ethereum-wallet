# c-ethereum-wallet

## Build

```
sudo apt install libusb-1.0-0-dev build-essential pkg-config
```

and 

```
cargo build --release
```

## How to use

```
#include <stdlib.h>
#include <stdio.h>

extern "C" {
extern char* generate_keystore(char *secret);
extern char* get_address();
extern char* sign_transaction(int nonce, char *recipient, char *secret);
};
```

### Example

```
sign_transaction(nonce,gwei_amount,gas,gas_price,chain_id,(char*)"RECIPIENT_ADDRESS",(char*)"KEYSTORE_PASSPHRASE");
```

```
#include "wallet.h"

int main(int argc, char *argv[]) {
  char *ret = generate_keystore((char*)"passphrase");
  printf("generate_keystore: %s\n", ret);
  char *addr = get_address();
  printf("address: %s\n", addr);
  char *signed_tx = sign_transaction(1,1000000000,1,56000,1,(char*)"0x7380f05998eB619d6a0E2F8acD19B2abe79E45DF",(char*)"passphrase");
  printf("signed_tx: %s\n",signed_tx);
  return(0);
}
```


