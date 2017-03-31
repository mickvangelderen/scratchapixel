#include <X11/Xlib.h>
#include <iostream>
#include <thread>
#include <chrono>

int main(void) {
    Display * display = XOpenDisplay(nullptr);
    if (display == nullptr) {
        std::cerr << "Failed to open display" << std::endl;
    }

    int screen = DefaultScreen(display);
    int black_pixel = BlackPixel(display, screen);
    int white_pixel = WhitePixel(display, screen);

    int width = XDisplayWidth(display, screen);
    int height = XDisplayHeight(display, screen);

    std::cout << width << std::endl;
    std::cout << height << std::endl;

    Window window = XCreateSimpleWindow(
        display,
        DefaultRootWindow(display),
        // x, probably ignored
        0,
        // y, probably ignored
        0,
        // width
        width,
        // height
        height,
        // border width, useless
        5,
        // border color,
        black_pixel,
        // background color,
        white_pixel
    );

    // Register for events.
    XSelectInput(display, window,
        StructureNotifyMask | KeyPressMask
    );

    // Display the window.
    XMapWindow(display, window);

    // Create a _Graphics Context_.
    GC graphics_context = XCreateGC(
        display,
        window,
        0,
        nullptr
    );

    // Draw with black.
    XSetForeground(display, graphics_context, black_pixel);

    int count = 0;

    // Wait for the MapNotify event.
    bool running = true;

    while (running) {
        XEvent event;
        XNextEvent(display, &event);
        switch (event.type) {
            case KeyPress: {
                std::cout << "key press:" << event.xkey.keycode << " modifiers " << event.xkey.state << std::endl;
                switch (event.xkey.keycode) {
                    case 9: // ESCAPE
                        running = false;
                        break;
                }
                break;
            }
            case MapNotify: {
                XDrawLine(display, window, graphics_context, 10, 10, 200, 200);
                break;
            }
            default: {
                std::cout << event.type << std::endl;
                break;
            }
        }
    }

    XCloseDisplay(display);

    return 0;
}
