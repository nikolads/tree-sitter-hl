const impl = require("./index.node");

class Highlighter {
    constructor() {
        this.boxed = impl.constructor();
    }

    highlight(lang, code) {
        return impl.highlight(this.boxed, lang, code);
    }

    supportedLanguages() {
        return impl.supported_languages(this.boxed);
    }
}

module.exports = {
    Highlighter,
};
