#include <stdlib.h>
#include <stdio.h>

extern "C" {
extern char* generate_keystore(char *secret);
extern char* get_address();
extern char* sign_transaction(int nonce, int gwei_amount, int gas, int gas_price, int chain_id, char *recipient, char *secret);
};