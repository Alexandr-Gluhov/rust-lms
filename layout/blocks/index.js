import WindowManager from "./js/windowManager.js";
import Window from "./js/window.js";

let centralWindow = new Window();
centralWindow.toggle();
document.getElementsByClassName("courses")[0].append(centralWindow.state);

async function start() {
    await centralWindow.init("main_struct");
    let element = centralWindow.state;
    element.classList.add('outer');
    let pull = element.querySelector('.aside');
    let manager = new WindowManager(element.querySelector('.content'), pull);
    await manager.createWindows();

    console.log(manager.register);

}

start();