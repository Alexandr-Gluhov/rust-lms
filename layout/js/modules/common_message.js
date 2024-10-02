import deserializer from '../deserializer.js'

deserializer.map.set("common_message", function(responseData) {
    let content = document.createElement('div');

    content.innerText = responseData.message;

    return content;
})