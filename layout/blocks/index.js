import windowsCreator from "./js/windowsCreator.js";

let element = document.getElementsByClassName("courses")[0];

windowsCreator.createWindow(element, "sleeppy", "sth");
windowsCreator.createWindow(element, "sth", "sth");

windowsCreator.createWindow(element, "common_image");

for (let i = 0; i < 10; i++) {
  windowsCreator.createWindow(element, "common_message");
  windowsCreator.createWindow(element, "hello", "somebody_else");
}
