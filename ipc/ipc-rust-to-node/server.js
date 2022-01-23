const net = require('net');

net.createServer((events)=>{
    events.on('data',(data)=>{
        console.log('data ', data.toString());
    });

    events.on('error',(err)=>{
        console.log('error ', err);
    });
}).listen('/tmp/ipc.sock', ()=>{
    console.log('listen on ');
});

