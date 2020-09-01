# 项目介绍
本项目通过node.js调用rust函数实现了一个简单的加密程序，跑起来之后向本机器的3000端口发送文本信息即可自动加密，同时支持解密。

# 使用方法

首先按照[这篇文章](https://www.secondstate.io/articles/get-started-with-rust-functions-in-node-zh/)介绍的方法搭建好环境并run起来。

然后就可以使用：

加密请使用 curl "http://0.0.0.0:3000/?operate=encrypt&text=hello"

解密请使用 curl "http://0.0.0.0:3000/?operate=encrypt&decode=YOURCODE&key=YOURKEY"

可以将hello替换成任意文本，在加密后会返回密文和密码，解密的时候可以使用。

最终会实现类似下面的效果
```
$ curl "http://0.0.0.0:3000/?operate=encrypt&text=hello"
密文是："ifmmp"，密码是："1"。

$ curl "http://0.0.0.0:3000/?operate=decode&decode=ifmmp&key=1"
解密结果："hello"。

$ curl "http://0.0.0.0:3000/?"
加密请使用 curl "http://0.0.0.0:3000/?operate=encrypt&text=hello"
解密请使用 curl "http://0.0.0.0:3000/?operate=encrypt&decode=YOURCODE&key=YOURKEY"

```
