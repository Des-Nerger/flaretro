#include <stdio.h>

int add_one(int i);

int main() {
	printf("3 + 1 = %d\n", add_one(3));
	printf("-7 + 1 = %d\n", add_one(-7));
	return 0;
}
