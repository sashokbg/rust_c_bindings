#pragma once
#ifdef __cplusplus
extern "C" {
#endif

// Returns malloc-allocated '\n'-separated screen names; free with kscreen_free_string.
char *kscreen_list_screens(void);
void kscreen_free_string(char *ptr);

#ifdef __cplusplus
}
#endif
