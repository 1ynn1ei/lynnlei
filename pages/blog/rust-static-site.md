# Static Site Generation in Rust

## Header 2

We used some code to make this work:
```
html! {
    (DOCTYPE)
    meta charset="utf-8";
    link rel="stylesheet" href="/styles.css";
    title {
        @if let Some(htitle) = htitle {
            "lynnlei"
                "::"
            (htitle)
        }
    }
    main {
        (content)
    }
    footer {
        "Built by rust, Powered by maud, Cleaned by ammonia"
    }
}
```
and then we did some other things

