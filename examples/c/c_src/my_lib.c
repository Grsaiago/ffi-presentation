#include "my_lib.h"

int get_c_pid(void) {
	return getpid();
}

void calling_another_function(void (*func)(void)) {
	printf("omg C hiiiiiiiiiiiiiiiiii\n");
	func();
}
