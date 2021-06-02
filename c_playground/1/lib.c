#include "lib.h" 
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

struct flexibles {
   int arraySize;
   int array[];
};

void testFlexibleArrayMembers() {
   int desiredSize = 5;
   struct flexibles *ptr;
   ptr = malloc( sizeof( struct flexibles ) + desiredSize + sizeof( int ) );
   printf("%p\n", ptr);
}

void testVariableLengthArrays() {
   size_t size = 4;
   int x = 0;
   char values[size];
   while (x < size) {
      values[x] = 'A' +x;
      printf("%c\n", values[x]);
      x++;
   }
}

int convertBinaryToDecimal(long long n) {
    int decimalNumber = 0, i = 0, remainder;
    while (n!=0)    {
        remainder = n%10;
        n /= 10;
        decimalNumber += remainder*pow(2,i);
        ++i;
    }
    return decimalNumber;
}

long long convertDecimalToBinary(int n){
   long long binaryNumber = 0;
   int remainder, i=1;

   while(n != 0) {
     remainder = n%2;
     n = n / 2;
     binaryNumber += remainder * i;
     i = i * 10;
   }

   return binaryNumber;

}

void testBinaryConverstions() {
   printf("Convery binary to decimal\n");
   long long n = 101;
   int nn = 34;
   printf("%lld in binary = %d in decimal\n", n, convertBinaryToDecimal(n));
   printf("%d in decimal = %lld to binary\n", nn , convertDecimalToBinary(n)); 
}
