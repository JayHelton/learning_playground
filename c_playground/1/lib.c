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

void testBitwise() {
   
    int num1 = 7, num2 = 700;
    long long b_num1 = 0, b_num2 = 0;

    int d_num1LeftShiftResult = 0;

    long long b_num1ComplementResult = 0;
    long long b_num2ComplementResult = 0;
    long long b_num1ANDnum2Result = 0;
    long long b_num1ORnum2Result = 0;
    long long b_num1ExORnum2Result = 0;
    long long b_num1LeftShiftResult = 0;
    b_num1 = convertDecimalToBinary(num1);
    b_num2 = convertDecimalToBinary(num2);

    d_num1LeftShiftResult = num1 << 2;
    b_num1ComplementResult = convertDecimalToBinary(~num1);
    b_num2ComplementResult = convertDecimalToBinary(~num2);
    b_num1ANDnum2Result = convertDecimalToBinary(num1 & num2);
    b_num1ORnum2Result = convertDecimalToBinary(num1 | num2);
    b_num1ExORnum2Result = convertDecimalToBinary(num1 ^ num2);
    b_num1LeftShiftResult = convertDecimalToBinary(num1 << 2);


    printf("The result of applying the ~ operator on number %d (%lld) is: %lld \n", num1, b_num1, b_num1ComplementResult);
    printf("The result of applying the ~ operator on number %d (%lld) is: %lld \n", num2, b_num2, b_num2ComplementResult);
    printf("The result of applying the & operator on number %d (%lld) and number %d (%lld) is: %lld \n", num1, b_num1, num2, b_num2, b_num1ANDnum2Result);
    printf("The result of applying the | operator on number %d (%lld) and number %d (%lld) is: %lld \n", num1, b_num1, num2, b_num2, b_num1ORnum2Result);
    printf("The result of applying the ^ operator on number %d (%lld) and number %d (%lld) is: %lld \n", num1, b_num1, num2, b_num2, b_num1ExORnum2Result);
    printf("The result of applying the left shift operator << on number %d (%lld) by 2 places is number %d (%lld)\n", num1, b_num1, d_num1LeftShiftResult, b_num1LeftShiftResult);
}

void testLinkedList() {

}
