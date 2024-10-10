import deserializer from "./deserializer.js";

let windowsCreator = {
  async createWindow(parent, block) {
    let response = await fetch(`/${block}/closed.html`);

    if (response.status === 404) {
      this.displayMessage("Запрашиваемый ресурс не найден, или не сущесвует");

      return;
    }

    let responseText = await response.text();

    parent.append(this.getWindowTemplate(block, responseText));
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

      if (!windowElement.classList.toggle("opened")) {
        let response = await fetch(`/${block}/closed.html`);
        let innerHTML = response.text();
        windowElement.innerHTML = innerHTML;
      } else {
        let response = await fetch(`/${block}/openned.html`);
        let innerHTML = await response.text();
        windowElement.innerHTML = innerHTML;
        console.log("sth");
        //this.addLoadingAnimation(windowElement);
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
};

export default windowsCreator;
