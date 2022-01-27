import * as glfw from "./bindings/bindings.ts";

glfw.init();
glfw.createWindow(640,480,"owo");
glfw.clearColor(
    {
        r: 255,
        g: 128,
        b: 128,
        a: 255
    }
);
while (!glfw.shouldClose()) {
    glfw.pollEvents();

    glfw.swapBuffers();

    glfw.clear();
    if (glfw.keyPressed(57)) {
        glfw.clearColor(
            {
                r: 0,
                g: 0,
                b: 0,
                a: 255
            }
        );
    } else {
        glfw.clearColor(
            {
                r: 255,
                g: 255,
                b: 255,
                a: 255
            }
        );
    }
}