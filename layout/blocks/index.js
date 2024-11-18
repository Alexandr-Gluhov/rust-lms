import windowsCreator from "./js/windowsCreator.js";

let element = document.getElementsByClassName("courses")[0];

async function start() {
    await windowsCreator.createWindows(element);
    await windowsCreator.createWindows(element);
    windowsCreator.register[0].querySelector('.name').click();
}

start();
