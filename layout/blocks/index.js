import windowsCreator from "./js/windowsCreator.js";

let element = document.getElementsByClassName("courses")[0];

windowsCreator.createWindow(element, "common_form");
windowsCreator.createWindow(element, "common_image");
windowsCreator.createWindow(element, "common_video");

windowsCreator.createWindow(element, "common_table");
for (let i = 0; i < 10; i++) {
  windowsCreator.createWindow(element, "common_message");
}
