use std::iter;

use neon::borrow::Borrow;
use neon::context::Context;
use neon::object::Object;
use neon::result::Throw;
use neon::types::{JsFunction, JsString, Value};

use crate::Highlighter;

neon::declare_types! {
    pub class JsHighlighter for Highlighter {
        init(_cx) {
            Ok(Highlighter::new())
        }

        method highlight(mut cx) {
            let this = cx.this();
            let lang = cx.argument::<JsString>(0)?.to_string(&mut cx)?.value();
            let code = cx.argument::<JsString>(1)?.to_string(&mut cx)?.value();

            let highlighted = {
                let guard = cx.lock();
                let highlighter = this.borrow(&guard);
                highlighter.highlight(&lang, &code)
            };

            match highlighted {
                Ok(highlighted) => Ok(cx.string(highlighted).upcast()),
                Err(crate::HighlightError::UnknownLanguage) => Err(Throw),
                Err(crate::HighlightError::TreeSitterError) => Err(Throw),
            }
        }

        method supportedLanguages(mut cx) {
            let this = cx.this();

            let languages: Vec<_> = {
                let guard = cx.lock();
                let highlighter = this.borrow(&guard);
                highlighter.supported_languages().map(String::from).collect()
            };

            let array = cx.empty_array();
            let push = array.get(&mut cx, "push")?.downcast::<JsFunction>().map_err(|_| Throw)?;

            for lang in languages {
                let lang = cx.string(lang);
                push.call(&mut cx, array, iter::once(lang))?;
            }

            Ok(array.upcast())
        }
    }
}

neon::register_module!(mut cx, {
    cx.export_class::<JsHighlighter>("Highlighter")?;
    Ok(())
});
