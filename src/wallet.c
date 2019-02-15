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