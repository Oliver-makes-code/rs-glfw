import * as glfw from "./bindings/bindings.ts";

glfw.init();
glfw.createWindow(640,480,"Haha");
while (!glfw.shouldClose()) {
    glfw.pollEvents();
}