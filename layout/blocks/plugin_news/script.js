const container = document.querySelector("#news-container");
async function showNews() {
    let newsRequest = await fetch("/get_news");
    let news = await newsRequest.json();
    for (let message of news) {
        let element = document.createElement("div");
        element.innerHTML = message.text + '<hr>';
        container.appendChild(element);
    }
}

showNews()