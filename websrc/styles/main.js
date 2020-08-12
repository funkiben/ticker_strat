const fs = require('fs');
const uglifycss = require('uglifycss');

const SRC = './src/';
const BUILD = '../../web/styles/'

fs.readdir(SRC, (err, files) => {
    if (err) {
        console.error(err);
    } else {
        files.forEach(file => {
            let minified = uglifycss.processFiles([SRC + file], { convertUrls: BUILD, expandVars: true });
            fs.writeFile(BUILD + file, minified, function(err) {
                if (err) {
                    console.error(err)
                } else {
                    console.log("Writing " + BUILD + file);
                }
            });
        });
    }
});
