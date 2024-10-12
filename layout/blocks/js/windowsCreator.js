import deserializer from "./deserializer.js";

let windowsCreator = {
  register: [],

  openedCount: 0,

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

      if (!windowElement.classList.toggle("opened")) {
        windowElement.lastElementChild.remove();
        windowElement.lastElementChild.hidden = false;
        if (!--this.openedCount) {
          this.register.forEach((wind) => {
            wind.classList.remove("tiny");
            document.querySelector(".courses").append(wind);
            if (wind.lastElementChild.className === "icon") {
              wind.lastElementChild.remove();
              wind.lastElementChild.hidden = false;
              wind.firstElementChild.hidden = false;
            }
          });
        } else {
          windowElement.classList.add("tiny");
          document.querySelector(".aside").prepend(windowElement);
          let icon = await fetch(`/${block}/tiny.html`);
          let result = await icon.text();
        }
      } else {
        windowElement.lastElementChild.hidden = true;
        this.addLoadingAnimation(windowElement);
        let response = await fetch(`/${block}/content.html`);
        let innerHTML = await response.text();
        let content = document.createElement("div");
        content.innerHTML = innerHTML;
        windowElement.lastElementChild.remove();
        windowElement.append(content);
        if (this.openedCount) {
          windowElement.classList.remove("tiny");
          document.querySelector(".courses").append(windowElement);
        } else {
          this.register.forEach(async (wind) => {
            if (wind !== windowElement) {
              if (wind.classList.contains("opened")) {
                wind.firstElementChild.click();
              }
              wind.classList.add("tiny");
              document.querySelector(".aside").append(wind);
            }

            let icon = await fetch(`/${wind.type}/tiny.html`);
            if (icon.status !== 404 && wind !== windowElement) {
              wind.lastElementChild.hidden = true;
              wind.firstElementChild.hidden = true;
              let div = document.createElement("div");
              div.addEventListener("click", () => {
                wind.firstElementChild.click();
                wind.firstElementChild.hidden = false;
              });
              div.innerHTML = await icon.text();
              div.className = "icon";
              wind.append(div);
            }
          });
        }
        this.openedCount++;
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
