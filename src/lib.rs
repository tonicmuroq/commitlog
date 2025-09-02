use askama::Template;
use worker::*;

static MESSAGES: &[&str] = &[
    "修了修了别再催我了",
    "诶我去, 上一个提交好像不太对",
    "打错字了我擦",
    "这样应该就可以了吧?",
    "那可能是需要这么搞一下",
    "啊啊啊啊啊终于好了",
    "我艹好像还是没好, 那这么改一下试试",
    "你妈啊这样总可以了吧",
    "谁写的sb代码啊改了改了",
    "先这样吧, 明天再说吧",
    "快到点了, 准备闪人了",
    "写的什么鬼, 不过能跑, 就酱吧",
    "啊啊啊啊写错了, 还好这次改对了",
    "这次的提交能值多少钱呢?",
    "我是sb啊啊啊...",
    "这回真的可以了, 我人格担保!",
    "原来没挂啊, 改这个就好了",
    "好像是可以了诶, 不过一会儿可能会挂, 先实验下",
    "我其实就是手贱提交了一下",
    "还是自己写靠谱啊",
    "被别人骗了, 这东西要这么写才对 T^T",
    "吔屎啦,产品狗！",
    "续一秒",
    "方便测试先改成这样，上线前再改回来",
    "线上Bug紧急修复",
    "产品经理说需求又改了，删掉删掉",
    "祈祷这次的CI能过",
    "晚上还没吃饭呢，有点饿了先这样吧",
    "怎么这么多会要开，下班才有时间干活啊",
    "什么鬼啊都没有新的message了",
];

const DUCK_GIF: &[u8] = include_bytes!("../templates/what-the-duck.gif");

const ALIAS_SNIPPET: &str = r#"[alias]
wtf = "!git commit -m \"$(curl -L -s https://commitlog.muroq.app)\"""#;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    Router::new()
        .get("/", |req, _ctx| {
            let accept = req.headers().get("Accept")?.unwrap_or_default();
            let message = pick_random_message();

            if accepts_html(&accept) {
                let page = IndexTemplate {
                    message,
                    alias_snippet: ALIAS_SNIPPET,
                };
                Response::from_html(page.render().map_err(|e| Error::RustError(e.to_string()))?)
            } else {
                let mut resp = Response::ok(format!("{}\n", message))?;
                resp.headers_mut()
                    .set("Content-Type", "text/plain; charset=utf-8")?;
                Ok(resp)
            }
        })
        .get("/_all", |_req, _ctx| Response::from_json(&MESSAGES))
        .get("/what-the-duck.gif", |_req, _ctx| {
            let mut resp = Response::from_bytes(DUCK_GIF.to_vec())?;
            resp.headers_mut().set("Content-Type", "image/gif")?;
            Ok(resp)
        })
        .run(req, env)
        .await
}

fn pick_random_message() -> &'static str {
    let n = MESSAGES.len() as f64;
    let idx = (js_sys::Math::random() * n) as usize;
    MESSAGES[idx]
}

fn accepts_html(accept: &str) -> bool {
    accept.to_ascii_lowercase().contains("text/html")
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    message: &'a str,
    alias_snippet: &'a str,
}
