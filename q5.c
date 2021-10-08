#include <stdio.h>


int main() {

    int A= 0,B = 0,X;
    
 //   X = !(!A);
 //   printf("X = !(!A) = %d \n",X);


    X = !(!(A && B));
    printf("X = (!(!(a & b)) = %d\n",X);

    
//    X = !(!(A || B));
//    printf("X = !(!(a | b)) = %d\n",X);


//    X = !(A && B);
//    printf("X = !(a & b) = %d\n",X);

//    X = !(A || B);
//    printf("X = !(a | b) = %d\n",X);
    
    
}