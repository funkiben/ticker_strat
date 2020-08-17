const fs = require('fs');
const uglifycss = require('uglifycss');

const SRC = './src/';
const BUILD = '../../web/styles/'

let oldFiles = fs.readdirSync(BUILD);
oldFiles.forEach(file => {
    fs.unlink(BUILD + file, (err1) => {
        if (err1) {
            console.error(err1);
        } else {
            console.log("Removing " + BUILD + file);
        }
    });
});

fs.readdir(SRC, (err, files) => {
    if (err) {
        console.error(err);
    } else {
        files.forEach(file => {
            let minified = uglifycss.processFiles([SRC + file]);
            fs.writeFile(BUILD + file, minified, function (err) {
                if (err) {
                    console.error(err)
                } else {
                    console.log("Writing " + BUILD + file);
                }
            });
        });
    }
});
