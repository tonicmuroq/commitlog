use askama::Template;
use worker::*;

static MESSAGES_ZH: &[&str] = &[
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

static MESSAGES_EN: &[&str] = &[
    "Fixed. Stop rushing me.",
    "Whoa, the last commit seems off.",
    "Damn, typo fixed.",
    "This should do it, right?",
    "Maybe it needs this tweak.",
    "Aaaaa it finally works!",
    "Crap, still broken; try this change.",
    "For real, this must work now.",
    "Who wrote this crap? Fixed.",
    "Good enough for now; revisit tomorrow.",
    "Time's up; I'm out.",
    "Messy, but it runs; ship it.",
    "Argh, wrong earlier; fixed it now.",
    "How much is this commit worth?",
    "I'm an idiot...",
    "It truly works now, I swear!",
    "Not down after all; this fix did it.",
    "Seems ok; might break later; testing now.",
    "Couldn't resist; just committed.",
    "Doing it myself is more reliable.",
    "Misled; this is the correct way T^T",
    "Product, go eat dirt!",
    "One more second.",
    "Temp change for testing; revert before release.",
    "Hotfix for production bug.",
    "PM changed scope again; delete it.",
    "Praying the CI passes.",
    "Hungry; leaving it like this for now.",
    "Too many meetings; real work after hours.",
    "What the heck, no new messages.",
];

const DUCK_GIF: &[u8] = include_bytes!("../templates/what-the-duck.gif");

const ALIAS_SNIPPET: &str = r#"[alias]
wtf = "!git commit -m \"$(curl -L -s https://commitlog.muroq.app)\"""#;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    Router::new()
        .get("/", |req, _ctx| {
            let accept = req.headers().get("Accept")?.unwrap_or_default();
            let lang = detect_lang(req.headers().get("Accept-Language").ok().flatten());
            let message = pick_random_message(lang);

            if accepts_html(&accept) {
                let t = texts(lang);
                let page = IndexTemplate {
                    message,
                    alias_snippet: ALIAS_SNIPPET,
                    t_title: t.title,
                    t_copy_btn: t.copy_btn,
                    t_copy_done: t.copy_done,
                    t_intro: t.intro,
                    t_hint_prefix: t.hint_prefix,
                    t_hint_suffix: t.hint_suffix,
                    t_boss_label: t.boss_label,
                    t_refresh_btn: t.refresh_btn,
                    t_code_btn: t.code_btn,
                };
                Response::from_html(page.render().map_err(|e| Error::RustError(e.to_string()))?)
            } else {
                let mut resp = Response::ok(format!("{}\n", message))?;
                resp.headers_mut()
                    .set("Content-Type", "text/plain; charset=utf-8")?;
                Ok(resp)
            }
        })
        .get("/_all", |req, _ctx| {
            let lang = detect_lang(req.headers().get("Accept-Language").ok().flatten());
            match lang {
                Lang::Zh => Response::from_json(&MESSAGES_ZH),
                Lang::En => Response::from_json(&MESSAGES_EN),
            }
        })
        .get("/what-the-duck.gif", |_req, _ctx| {
            let mut resp = Response::from_bytes(DUCK_GIF.to_vec())?;
            resp.headers_mut().set("Content-Type", "image/gif")?;
            Ok(resp)
        })
        .get("/favicon.ico", |_req, _ctx| {
            let mut resp = Response::from_bytes(DUCK_GIF.to_vec())?;
            resp.headers_mut().set("Content-Type", "image/gif")?;
            Ok(resp)
        })
        .run(req, env)
        .await
}

#[derive(Copy, Clone)]
enum Lang {
    Zh,
    En,
}

fn pick_random_message(lang: Lang) -> &'static str {
    let list = match lang {
        Lang::Zh => MESSAGES_ZH,
        Lang::En => MESSAGES_EN,
    };
    let n = list.len() as f64;
    let idx = (js_sys::Math::random() * n) as usize;
    list[idx]
}

fn accepts_html(accept: &str) -> bool {
    accept.to_ascii_lowercase().contains("text/html")
}

fn detect_lang(header: Option<String>) -> Lang {
    let s = header.unwrap_or_default().to_ascii_lowercase();
    for part in s.split(',') {
        let tag = part.trim().split(';').next().unwrap_or("").trim();
        if tag.starts_with("zh") {
            return Lang::Zh;
        }
    }
    Lang::En
}

struct Texts {
    title: &'static str,
    copy_btn: &'static str,
    copy_done: &'static str,
    intro: &'static str,
    hint_prefix: &'static str,
    hint_suffix: &'static str,
    boss_label: &'static str,
    refresh_btn: &'static str,
    code_btn: &'static str,
}

fn texts(lang: Lang) -> Texts {
    match lang {
        Lang::Zh => Texts {
            title: "Commit Log as a Service",
            copy_btn: "点一下, 复制这条",
            copy_done: " (嗯… 复制好了",
            intro: "打开你的",
            hint_prefix: "记得转义双引号一定不能去掉, 因为这坑货自己会去掉双引号… 下次提交就敲",
            hint_suffix: "看看你的老板多久会打死你。",
            boss_label: "你的老板:",
            refresh_btn: "刷新下, 换一条",
            code_btn: "看下代码",
        },
        Lang::En => Texts {
            title: "Commit Log as a Service",
            copy_btn: "Copy",
            copy_done: " (Copied)",
            intro: "Open your",
            hint_prefix:
                "Make sure to keep the escaped quotes, or they get stripped. Next time just run",
            hint_suffix: "and see how long until your boss gets mad.",
            boss_label: "Your boss:",
            refresh_btn: "Refresh for another",
            code_btn: "View Code",
        },
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    message: &'a str,
    alias_snippet: &'a str,
    t_title: &'a str,
    t_copy_btn: &'a str,
    t_copy_done: &'a str,
    t_intro: &'a str,
    t_hint_prefix: &'a str,
    t_hint_suffix: &'a str,
    t_boss_label: &'a str,
    t_refresh_btn: &'a str,
    t_code_btn: &'a str,
}
