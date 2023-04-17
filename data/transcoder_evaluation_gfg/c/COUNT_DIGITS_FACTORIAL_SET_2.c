// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//


#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int min(int x, int y) { return (x < y)? x: y; }
int max(int x, int y) { return (x > y)? x: y; }
int cmpfunc (const void * a, const void * b) {return ( *(int*)a - *(int*)b );}
int len (int arr [ ]) {return ((int) (sizeof (arr) / sizeof (arr)[0]));}
void sort (int arr [ ], int n) {qsort (arr, n, sizeof(int), cmpfunc);}

long long f_gold ( int n ) {
  if ( n < 0 ) return 0;
  if ( n <= 1 ) return 1;
  double x = ( ( n * log10 ( n / M_E ) + log10 ( 2 * M_PI * n ) / 2.0 ) );
  return floor ( x ) + 1;
}


long long f_filled ( int n ) {}

int main(void) {
    int n_success = 0;
    int param0[] = {56,92,52,96,96,63,51,79,70,9};
    for(int i = 0; i < len(param0); ++i)
    {
        if(f_filled(param0[i]) == f_gold(param0[i]))
        {
            n_success+=1;
        }
    }
    printf("#Results:", " ", n_success, ", ", len(param0));
    return 0;
}