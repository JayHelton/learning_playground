#include <stdio.h>
#include "lib.h"

int main() {
   printf("Test Variable Length Arrays\n");
   testVariableLengthArrays();
   printf("Test Flexible Array Members\n");
   testFlexibleArrayMembers();
   printf("Test Manual Binary Conversions\n");
   testBinaryConverstions();
   testBitwise();
   return 0;
}
