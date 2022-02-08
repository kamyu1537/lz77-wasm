const fs = require('fs');

fs.rmSync('./dist/.gitignore');
fs.rmSync('./dist/package.json');
fs.rmSync('./dist/LICENSE');
fs.rmSync('./dist/README.md');
