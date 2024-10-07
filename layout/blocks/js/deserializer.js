let deserializer = {

    map: new Map(),

    async load(contentType) {
        let response = await fetch(`/js/modules/${contentType}.js`);

        if (response.ok) {
            let script = document.createElement("script");
            script.type = "module";
            script.src = `/js/modules/${contentType}.js`;
            document.body.append(script);
        }
    },

    deserialize(contentType, response) {

        let responseData = JSON.parse(response);

        if (this.map.has(contentType) && this.map.get(contentType)) {

            return this.map.get(contentType)(responseData);

        }

    },
}

export default deserializer;