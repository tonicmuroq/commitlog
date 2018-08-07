# coding: utf-8

import random

from flask import Flask, request, jsonify, render_template

messages = [
    u'修了修了别再催我了',
    u'诶我去, 上一个提交好像不太对',
    u'打错字了我擦',
    u'这样应该就可以了吧?',
    u'那可能是需要这么搞一下',
    u'啊啊啊啊啊终于好了',
    u'我艹好像还是没好, 那这么改一下试试',
    u'你妈啊这样总可以了吧',
    u'谁写的sb代码啊改了改了',
    u'先这样吧, 明天再说吧',
    u'快到点了, 准备闪人了',
    u'写的什么鬼, 不过能跑, 就酱吧',
    u'啊啊啊啊写错了, 还好这次改对了',
    u'这次的提交能值多少钱呢?',
    u'我是sb啊啊啊...',
    u'这回真的可以了, 我人格担保!',
    u'原来没挂啊, 改这个就好了',
    u'好像是可以了诶, 不过一会儿可能会挂, 先实验下',
    u'我其实就是手贱提交了一下',
    u'还是自己写靠谱啊',
    u'被别人骗了, 这东西要这么写才对 T^T',
    u'吔屎啦,产品狗！',
    u'续一秒',
    u'方便测试先改成这样，上线前再改回来',
    u'线上Bug紧急修复',
    u'产品经理说需求又改了，删掉删掉',
    u'祈祷这次的CI能过',
    u'晚上还没吃饭呢，有点饿了先这样吧',
    u'什么鬼啊都没有新的message了',
]

app = Flask(__name__)


@app.route('/')
def index():
    message = random.choice(messages)
    if not request.user_agent.browser:
        return u'%s\n' % message
    return render_template('landing.html', message=message)


@app.route('/_all')
def list_all():
    return jsonify(messages)
