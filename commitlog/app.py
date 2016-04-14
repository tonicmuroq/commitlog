# coding: utf-8

import json
import random
from flask import Flask
from flask import Response

messages = [
    '修了修了别再催我了',
    '诶我去, 上一个提交好像不太对',
    '打错字了我擦',
    '这样应该就可以了吧?',
    '那可能是需要这么搞一下',
    '啊啊啊啊啊终于好了',
    '我艹好像还是没好, 那这么改一下试试',
    '你妈啊这样总可以了吧',
    '谁写的sb代码啊改了改了',
    '先这样吧, 明天再说吧',
    '快到点了, 准备闪人了',
    '写的什么鬼, 不过能跑, 就酱吧',
    '啊啊啊啊写错了, 还好这次改对了',
    '这次的提交能值多少钱呢?',
    '我是sb啊啊啊...',
    '这回真的可以了, 我人格担保!',
    '原来没挂啊, 改这个就好了',
    '好像是可以了诶, 不过一会儿可能会挂, 先实验下',
    '我其实就是手贱提交了一下',
    '还是自己写靠谱啊',
    '被别人骗了, 这东西要这么写才对 T^T',
    '吔屎啦,产品狗！',
    '续一秒',
    '方便测试先改成这样，上线前再改回来',
    '线上Bug紧急修复',
    '产品经理说需求又改了，删掉删掉',
    '祈祷这次的CI能过',
]

app = Flask(__name__)


@app.route('/')
def index():
    return '%s\n' % random.choice(messages)


@app.route('/_all')
def list_all():
    return Response(json.dumps(messages), mimetype='application/json')
