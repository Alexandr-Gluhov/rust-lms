import deserializer from "./deserializer.js";

let windowsCreator = {
  register: [],

  opened: null,

  async createWindows(parent) {
    let response = await fetch("/get_plugins");
    let plugins = await response.json();
    for (let plugin of plugins) {
      await this.createWindow(parent, plugin);
    }
  },

  async createWindow(parent, block) {
    let response = await fetch(`/${block}/closed.html`).catch();

    if (response.status === 404) {
      this.displayMessage("Запрашиваемый ресурс не найден, или не сущесвует");

      return;
    }

    let responseText = await response.text();

    let windowTemplate = this.getWindowTemplate(block, responseText);
    windowTemplate.type = block;

    parent.append(windowTemplate);
    this.register.push(windowTemplate);
  },

  getWindowTemplate(block, innerHTML) {
    let windowTemplate = document.createElement("div");
    windowTemplate.classList.add("window");

    windowTemplate.innerHTML = innerHTML;

    windowTemplate
      .getElementsByClassName("name")[0]
      .addEventListener("click", this.getWindowSwitchHandler(block));

    return windowTemplate;
  },

  getWindowSwitchHandler(block) {
    return async (event) => {
      let windowElement = event.target.parentElement;

      if (windowElement.classList.toggle("opened")) {
        if (!this.opened) {
          this.openWindow(windowElement);

          this.makeIcons();
          this.opened = windowElement;
        } else {
          this.closeWindow(this.opened);
          this.makeIcon(this.opened);
          this.openWindow(windowElement);
          this.opened = windowElement;
        }
      } else {
        this.opened = null;

        this.closeWindow(windowElement);
        this.makeWindowsFromIcons();
      }
    };
  },

  addLoadingAnimation(element) {
    let div = document.createElement("div");
    div.classList.add("loading");
    element.append(div);
    let spinner = document.createElement("div");
    spinner.classList.add("spinner");
    let block = document.createElement("div");
    block.classList.add("block");

    setTimeout(() => {
      if (element.lastChild.hidden !== true) {
        div.append(spinner, block);
      }
    });
  },

  async displayMessage(message) {
    let div = document.createElement("div");
    div.classList.add("message");

    let response = await fetch("/files/blocks/404.html");

    div.innerHTML = await response.text();
    div.lastElementChild.innerText = message;

    document.body.append(div);
    setTimeout(() => (div.style.opacity = "0"));
    setTimeout(() => div.remove(), 5000);
  },

  async makeIcons() {
    for (let i = this.register.length - 1; i >= 0; i--) {
      if (!this.register[i].classList.contains("opened")) {
        await this.makeIcon(this.register[i]);
      }
    }
  },

  async makeIcon(windowElement) {
    windowElement.classList.add("tiny");
    let icon = await fetch(`/${windowElement.type}/tiny.html`);
    let div = document.createElement("div");
    div.innerHTML = await icon.text();
    windowElement.firstElementChild.hidden = true;
    windowElement.lastElementChild.hidden = true;
    windowElement.append(div);
    document.querySelector(".aside").prepend(windowElement);

    windowElement.lastElementChild.addEventListener("click", () => {
      this.makeWindowFromIcon(windowElement);
      windowElement.firstElementChild.click();
    });
  },

  makeWindowsFromIcons() {
    this.register.forEach((icon) => {
      if (icon.classList.contains("tiny")) {
        this.makeWindowFromIcon(icon);
      }
    });
  },

  makeWindowFromIcon(icon) {
    icon.classList.remove("tiny");
    icon.lastElementChild.remove();
    icon.firstElementChild.hidden = false;
    icon.lastElementChild.hidden = false;
    document.querySelector(".courses").append(icon);
  },

  closeWindow(windowElement) {
    windowElement.classList.remove("opened");
    windowElement.lastElementChild.remove();
    windowElement.lastElementChild.hidden = false;
  },

  async openWindow(windowElement) {
    windowElement.classList.add("opened");
    windowElement.lastElementChild.hidden = true;
    console.log(windowElement.lastElementChild);
    this.addLoadingAnimation(windowElement);
    let response = await fetch(`/${windowElement.type}/content.html`);
    let innerHTML = await response.text();
    let content = document.createElement("div");
    content.innerHTML = innerHTML;
    windowElement.lastElementChild.remove();
    windowElement.append(content);
  },
};

export default windowsCreator;
