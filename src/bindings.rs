use std::iter;

use neon::prelude::*;
use neon::result::Throw;

use crate::Highlighter;

impl Finalize for Highlighter {}

fn constructor(mut cx: FunctionContext) -> JsResult<JsBox<Highlighter>> {
    Ok(cx.boxed(Highlighter::new()))
}

fn highlight(mut cx: FunctionContext) -> JsResult<JsString> {
    let this = cx.argument::<JsBox<Highlighter>>(0)?;
    let lang = cx.argument::<JsString>(1)?.to_string(&mut cx)?.value(&mut cx);
    let code = cx.argument::<JsString>(2)?.to_string(&mut cx)?.value(&mut cx);

    let highlighted = { this.highlight(&lang, &code) };

    match highlighted {
        Ok(highlighted) => Ok(cx.string(highlighted)),
        Err(crate::HighlightError::UnknownLanguage) => Err(Throw),
        Err(crate::HighlightError::TreeSitterError) => Err(Throw),
    }
}

fn supported_languages(mut cx: FunctionContext) -> JsResult<JsArray> {
    let this = cx.argument::<JsBox<Highlighter>>(0)?;

    let languages: Vec<_> = { this.supported_languages().map(String::from).collect() };

    let array = cx.empty_array();
    let push = array
        .get(&mut cx, "push")?
        .downcast::<JsFunction, _>(&mut cx)
        .map_err(|_| Throw)?;

    for lang in languages {
        let lang = cx.string(lang);
        push.call(&mut cx, array, iter::once(lang))?;
    }

    Ok(array)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("constructor", constructor)?;
    cx.export_function("highlight", highlight)?;
    cx.export_function("supported_languages", supported_languages)?;

    Ok(())
}
