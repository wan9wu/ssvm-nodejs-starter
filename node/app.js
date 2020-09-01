const { say, jiami, jiemi } = require('../pkg/ssvm_nodejs_starter_lib.js');

const http = require('http');
const url = require('url');
const hostname = '0.0.0.0';
const port = 3000;

const server = http.createServer((req, res) => {
  const queryObject = url.parse(req.url,true).query;
  console.log(queryObject);
  if (!queryObject['name']) {
    if(queryObject['operate']){
      if(queryObject['operate']=="encrypt" && queryObject['text']){
        res.end(jiami(queryObject['text']) + '\n');
      };
      if(queryObject['operate']=="decode" && queryObject['decode']&&queryObject['key']){
        res.end(jiemi(queryObject['decode'],queryObject['key']) + '\n');
      };
      res.end(`加密请使用 curl  \"http://${hostname}:${port}/?operate=encrypt&text=hello\" \n \
      解密请使用 curl \"http://${hostname}:${port}/?operate=encrypt&decode=YOURCODE&key=YOURKEY\" \n`);
    }else{
      res.end(`加密请使用 curl  \"http://${hostname}:${port}/?operate=encrypt&text=hello\" \n \
      解密请使用 curl \"http://${hostname}:${port}/?operate=encrypt&decode=YOURCODE&key=YOURKEY\" \n`);
    }
  } else {
    res.end(say(queryObject['name']) + '\n');
  }
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
