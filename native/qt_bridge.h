#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// Launches a minimal Qt window. Returns the Qt event loop exit code.
int qt_show_window(void);
char *kscreen_list_screens(void);
void kscreen_free_string(char *ptr);

#ifdef __cplusplus
}
#endif
