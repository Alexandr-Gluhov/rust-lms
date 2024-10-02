import deserializer from './deserializer.js';

let windowsCreator = {

  async createWindow(parent, plugin, query) {

    let response = await fetch(`/api/${plugin}/${query}`);

    let responseText = await response.text()

    let content = JSON.parse(responseText);

    if (!deserializer.map.has(content.type)) {
      deserializer.map.set(content.type, null);
      deserializer.load(content.type);
    }

    let info = {
      type: content.type,
      plugin,
      query,
    }

    parent.append(this.getWindowTemplate(info, content.name, content.discription));

  },

  getWindowTemplate(windowInfo, nameContent, discriptionContent) {

    let windowTemplate = document.createElement('div');
    windowTemplate.classList.add('window');

    windowTemplate.innerHTML =`<div class="name">${nameContent}</div><div class="discription">${discriptionContent}</div>`

    windowTemplate.getElementsByClassName('name')[0].addEventListener('click', this.getWindowSwitchHandler(windowInfo));

    return windowTemplate;
    
  },

  getWindowSwitchHandler(windowInfo) {
    return async function(event) {

      let windowElement = event.target.parentElement;

      if (!windowElement.classList.toggle('opened')) {

        windowElement.lastChild.remove();
        windowElement.lastChild.hidden = false;

      }
      else {

        windowElement.lastChild.hidden = true;

        let response = await fetch(`/api/${windowInfo.plugin}/open/${windowInfo.query}`);

        let responseText = await response.text();

        windowElement.append(deserializer.deserialize(windowInfo.type, responseText));

      }
    }
  },

}

export default windowsCreator;