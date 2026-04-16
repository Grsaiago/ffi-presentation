#ifndef MY_LIB
#define MY_LIB
#include <stdio.h>
#include <sys/types.h>
#include <unistd.h>

int get_c_pid(void);
void calling_another_function(void (*func)(void));
#endif // MY_LIB
