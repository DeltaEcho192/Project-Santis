const express = require('express');
const pug = require('pug');
const bodyParser = require('body-parser');
const path = require('path');
const { v4 : uuid } = require('uuid');
const sqlite3 = require('sqlite3');

const PORT = process.env.PORT || 2502

const app = express();
app.set('view engine', 'pug');
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({extended: true}));
app.use(express.static(path.join(__dirname, 'assets')))

let db = new sqlite3.Database("santis.db" , (err) => { 
    if (err) { 
        console.log("Error Occurred - " + err.message); 
    } else { 
        console.log("DataBase Connected"); 
    } 
})

app.get('/', (req, res) => {
	values = ["test1", "test2"]
	res.render('index', {cats: values})
});

app.post('/test', (req, res) => {
	console.log(req.body.item_name)
	let id = uuid();
	console.log(id)
	let r = req.body;
	if (r.packed == "on") {
		r.packed = true;
	} else {
		r.packed = false;
	}
	console.log(r.packed)
	try {
	db.run("INSERT INTO items ('item_id', 'item_name', 'size', 'weight', 'value', 'packed', 'category', 'sub_category') VALUES ($id, $item_name, $size, $weight, $value, $packed, $category, $sub_category);", 
		{$id:id,
			$item_name: r.item_name,
			$size: r.size,
			$weight: r.weight,
			$value: r.value,
			$packed: r.packed,
			$category: r.category,
			$sub_category: r.sub_category});

	} catch (e) {
		res.send("<p>Error</p>")
	}
	res.send("<p>Success</p>")
});

app.post('/save', (req, res) => {
	
})

app.listen(PORT);
console.log("Santis is listening on port: " + PORT);
