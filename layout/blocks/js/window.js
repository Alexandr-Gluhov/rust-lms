import Notifier from "./notifier.js";

class Window {
    constructor(onClick) {
        this.window = document.createElement('div');
        this.window.classList.add('window', 'opened');

        this.name = document.createElement('div');
        this.name.classList.add('name');

        this.content = document.createElement('div');
        this.content.classList.add('content');

        this.window.append(this.name, this.content);

        this.tiny = document.createElement('div');
        this.tiny.classList.add('window', 'tiny');

        this.state = this.tiny;
        this.tiny.addEventListener('click', onClick);
    }

    async init(plug) {
        let nameResponse = await fetch(`/${plug}/name.html`).catch();
        let contentResponse = await fetch(`/${plug}/content.html`).catch();
        let tinyResponse = await fetch(`/${plug}/tiny.html`).catch();

        if (nameResponse.status === 404 || contentResponse.status === 404 || tinyResponse.status === 404) {
            Notifier.displayMessage(`Запрашиваемый ресурс ${plug} не найден, или не сущесвует`);
            return;
        }

        let nameResponseText = await nameResponse.text();
        this.name.innerHTML = nameResponseText;

        let contentResponseText = await contentResponse.text();
        this.content.innerHTML = contentResponseText;

        let tinyResponseText = await tinyResponse.text();
        this.tiny.innerHTML = tinyResponseText;
    }

    toggle() {
        if (this.state === this.tiny) {
            this.tiny.replaceWith(this.window);
            this.state = this.window;
        } else {
            this.window.replaceWith(this.tiny);
            this.state = this.tiny;
        }
    }
}

export default Window;