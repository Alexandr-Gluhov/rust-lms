import windowsCreator from "./js/windowsCreator.js"

let element = document.getElementsByClassName('courses')[0];

windowsCreator.createWindow(element, 'sleeppy', 'sth');
windowsCreator.createWindow(element, 'sth', 'sth');

for (let i = 0; i < 10; i++) {
    windowsCreator.createWindow(element, 'hello', 'somebody');
    windowsCreator.createWindow(element, 'hello', 'somebody_else');
}