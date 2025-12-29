#include "qt_bridge.h"

#include <QApplication>
#include <QLabel>

int qt_show_window(void) {
    int argc = 0;
    char** argv = nullptr;
    QApplication app(argc, argv);

    QLabel label("Hello from Qt6 + Rust");
    label.resize(320, 120);
    label.show();

    return app.exec();
}
