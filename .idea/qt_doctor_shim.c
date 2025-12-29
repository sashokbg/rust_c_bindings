#include "qt_bridge.h"

  #include <QCoreApplication>
  #include <QGuiApplication>
  #include <QScreen>
  #include <cstdlib>
  #include <cstring>

  // Returns a malloc-allocated, '\n'-separated list of screen names.
  // Caller must free with kscreen_free_string.
  extern "C" char *kscreen_list_screens(void) {
      int argc = 0;
      char **argv = nullptr;

      // Reuse an existing app if Rust already spun one up.
      QGuiApplication *app = qobject_cast<QGuiApplication *>(QCoreApplication::instance());
      bool owns_app = false;
      if (!app) {
          app = new QGuiApplication(argc, argv);
          owns_app = true;
      }

      QStringList names;
      for (QScreen *screen : app->screens()) {
          names << screen->name();
      }

      QByteArray bytes = names.join('\n').toUtf8();
      char *out = static_cast<char *>(std::malloc(bytes.size() + 1));
      if (!out) {
          if (owns_app) delete app;
          return nullptr;
      }
      std::memcpy(out, bytes.constData(), bytes.size());
      out[bytes.size()] = '\0';

      if (owns_app) {
          app->quit();
          delete app;
      }
      return out;
  }

  extern "C" void kscreen_free_string(char *ptr) {
      std::free(ptr);
  }


  unsafe fn print_screens() {
      let ptr = kscreen_list_screens();
      if ptr.is_null() {
          eprintln!("kscreen_list_screens returned null");
          return;
      }
      let names = CStr::from_ptr(ptr).to_string_lossy();
      println!("Screens:\n{names}");
      kscreen_free_string(ptr);
  }
