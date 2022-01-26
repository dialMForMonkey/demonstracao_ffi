const 
  r
 = require('./index');
const fs = require('fs');

const files = [
    "./frase1.txt",
    "./frase2.txt",
    "./frase3.txt",
    "./frase4.txt"
];

/**
    readFiles(files)
**/


function ErroReadFileMoreOneGB() {
  fs.readFile("./frase1.txt", (err, data) => {
    if (err) {
      console.log(err);
    }
    console.log('acabou de ler arquivo 1');
  });
}

function readFile(path, finishFun){
    const streamFile = fs.createReadStream("./frase1.txt");
    streamFile.on('error', (err) => {
        console.log('err', err);
    })
    streamFile.on('readable', () => {
      const read = streamFile.read({size:512});
      if(read == null) {
        finishFun()
      } 
    })
    streamFile.on('ready', () => {
        console.log('ready for read file ',path);
    })
}


function readFilesNode(files){
    for (let index = 0, max= files.length; index < max; index++) {
      const file = files[index];
      const start = Date.now();
      readFile(file,()=>{

        const end = Date.now();
        const elapsed = end - start;

        console.log(`file ${file} time elapsed ${elapsed} ms`)
      })
    }
}

//r.readFiles(files);

readFilesNode(files);


