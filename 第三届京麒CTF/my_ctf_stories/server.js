const express = require('express');
const path = require('path');
const app = express();

app.use('/', express.static(path.join(__dirname, 'storybook-static')));

app.listen(80, () => {
  console.log('CTF server listening on port 80');
});
