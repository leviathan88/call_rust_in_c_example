#include <stdio.h>
#include <stdbool.h>

int add(int *a, int *b, int *res);
int solve_interleave(char *a, char *b, char *c, bool *result);

int main() {
    int f = 3;
    int d = 3;
    int k;
    add(&f, &d, &k);
    
    printf("The result for add() is %d", k);
    printf("\n");

    char a[] = "aabcc";
    char b[] = "dbbca";
    char c[] = "aadbbcbcac";
    bool result = false;

    solve_interleave(a,b,c,&result);
    printf("The result for solve_interleave() is %d", result);
    printf("\n");
    
    return 0;
}