#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
uint64_t timestamp_now(){
	return time(NULL);
}
uint64_t timestamp_strftime(uint64_t timestamp, const char *format, char *buffer, uint64_t len){
	struct tm time_info;
	localtime_r((time_t *)&timestamp,&time_info);
	return strftime(buffer,len,format,&time_info);
}
