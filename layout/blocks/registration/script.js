import Group from "/js/group.js";

let groups = new Group();

async function initSelect() {
    let select = document.getElementById("groups-select");
    for (let group of await groups.getAll()) {
        let option = document.createElement("option");
        option.value = group.id;
        option.innerText = group.name;
        select.appendChild(option);
    }
}

initSelect();

import Notifier from "/js/notifier.js";

const form = document.getElementById("registration-form");

async function sendData(event) {
    event.preventDefault();
    const formData = new FormData(form);


    const formDataParams = new URLSearchParams();
    for (const [key, value] of formData.entries()) {
        formDataParams.append(key, value);
    }

    const response = await fetch("/register", {
        headers: {"Content-Type": "application/x-www-form-urlencoded"},
        method: "POST",
        body: formDataParams.toString(),
    });

    const notifier = new Notifier();
    notifier.displayMessage(await response.text());
}

form.addEventListener("submit", sendData);