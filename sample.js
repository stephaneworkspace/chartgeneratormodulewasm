const path = require('path');
const fs = require('fs');

export function read_file() {
		console.log('2');
		console.log(path.resolve(__dirname));
		return fs.readFileSync(path.resolve(__dirname, 'test.txt'), null).toString(); // for test
    //return fs.readFileSync(path.resolve(__dirname, 'test.txt'), null).buffer;
}
