#include "lib.h" 
#include <stdio.h>
#include <stdlib.h>

struct flexibles {
   int arraySize;
   int array[];
};

void testFlexibleArrayMembers() {
   int desiredSize = 5;
   struct flexibles *ptr;
   ptr = malloc( sizeof( struct flexibles ) + desiredSize + sizeof( int ) );
   printf("%d\n", ptr);
}
void testVariableLengthArrays() {
   size_t size = 4;
   int x = 0;
   char values[size];
   while (x < size) {
      values[x] = 'A' + x;
      printf("%c\n", values[x]);
      x++;
   }
}
