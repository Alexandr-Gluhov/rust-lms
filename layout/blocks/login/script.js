import Notifier from "/js/notifier.js";

const form = document.getElementById("login-from");
async function sendData(event) {
    event.preventDefault();
    const formData = new FormData(form);


    const formDataParams = new URLSearchParams();
    for (const [key, value] of formData.entries()) {
        formDataParams.append(key, value);
    }

    const response = await fetch("/login", {
        headers: { "Content-Type": "application/x-www-form-urlencoded" },
        method: "POST",
        body: formDataParams.toString(),
    });

    const notifier = new Notifier();
    notifier.displayMessage(await response.text());
}

form.addEventListener("submit", sendData);