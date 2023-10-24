const express = require('express');
const pug = require('pug');
const bodyParser = require('body-parser');
const path = require('path');
const { v4 : uuid } = require('uuid');

const PORT = process.env.PORT || 2502

const app = express();
app.set('view engine', 'pug');
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({extended: true}));
app.use(express.static(path.join(__dirname, 'assets')))

app.get('/', (req, res) => {
	res.render('index')
});

app.post('/test', (req, res) => {
	const { test } = req.body;
	console.log(test)
	res.send(`<p>${test}</p>`)
});

app.listen(PORT);
console.log("Santis is listening on port: " + PORT);
