import Window from "./window.js";

class WindowManager {

  constructor(nest, iconsPull) {
    this.register = [];
    this.opened = null;
    this.nest = nest;
    this.iconsPull = iconsPull;
  }

  changeOpened = (event) => {
    this.opened.toggle();
    this.iconsPull.prepend(this.opened.state);
    let target = event.target;
    if (!target.classList.contains('window')) {
      target = target.parentNode;
    }
    let el = this.register.find(element => element.tiny === target);
    el.toggle();
    this.nest.prepend(el.state);
    this.opened = el;
  }

  async createWindow(block) {

    let element = new Window(this.changeOpened);
    this.register.push(element);
    await element.init(block);

    if (this.opened === null) {

      element.toggle();
      this.opened = element;
      console.log(this.opened);
      this.nest.prepend(element.state);
      console.log("Window");
      console.log(this.opened.state);

    } else {
      console.log("Icon");
      console.log(element.state);
      this.iconsPull.append(element.state);

    }
    
  }

  async createWindows() {

    let response = await fetch("/get_plugins");
    let plugins = await response.json();

    for (let plugin of plugins) {
      this.createWindow(plugin);
    }

  }

}

export default WindowManager;
