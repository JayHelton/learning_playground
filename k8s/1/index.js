const http = require("http");
const os = require("os");

console.log("Kubia starting");
const handler = (req, res) => {
  console.log(`Received request from ${req.connection.remoteAddress}`);
  res.writeHead(200);
  res.end(`Youve hit ${os.hostname()} \n`);
};

http.createServer(handler).listen(8080);
