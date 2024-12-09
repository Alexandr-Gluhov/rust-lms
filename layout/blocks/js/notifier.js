class Notifier {
    async displayMessage(message) {
        let div = document.createElement("div");
        div.classList.add("message");
    
        div.innerText = message;
    
        document.body.append(div);
        setTimeout(() => (div.style.opacity = "0"));
        setTimeout(() => div.remove(), 5000);
      }
}

export default Notifier;