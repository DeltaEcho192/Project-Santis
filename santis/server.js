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

app.get('/', async (req, res) => {
	values = ["KEEP - Store", "KEEP - Take", "SELL", "DONATE"]
	static = ["SMALL", "MEDIUM", "LARGE", "EXTRA LARGE"]
	sql = "SELECT item_id, item_name FROM items;"
	db.all(sql, [], (err, rows) => {
		if (err)
			throw err
		res.render('index', {cats: values, items: rows, sizes: static})
	})
});

app.get('/list', async (req, res) => {
	sql = "SELECT item_id, item_name, category FROM items;"
	db.all(sql, [], (err, rows) => {
		if (err)
			throw err
		res.render('list', {items: rows})
	})
});

app.post('/items', (req, res) => {
	let id = uuid();
	console.log(id)
	let r = req.body;
	if (r.packed == "on") {
		r.packed = true;
	} else {
		r.packed = false;
	}
	console.log(r.packed)
	let status_message = "Success"
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
		status_message = "Error"
	}

	let template = pug.compileFile('views/includes/enter_message.pug');
	let markup = template({status_message: status_message})
	res.send(markup)
})

app.get("/item/:id/edit", (req, res) => {
	let sql = `SELECT item_id, item_name, category FROM items WHERE item_id=?;`
	console.log(sql)
	db.get(sql, [req.params.id], (err,rows) => {
		if (err)
			throw err
		console.log("Rows: ", rows)
		let template = pug.compileFile('views/includes/table_edit.pug')
		let markup = template({item: rows})
		res.send(markup)
	})
})

app.put('/item/:id', (req,res) => {
	console.log(req.body)
	let sql = `UPDATE items set item_name=?, category=? WHERE item_id=?`
	db.run(sql, [req.body.item_name, req.body.category, req.params.id], (err, rows) => {
		if (err)
			throw err
	});
	let rt = {
		item_id: req.params.id,
		item_name: req.body.item_name,
		category: req.body.category
	}
	let template = pug.compileFile('views/includes/table_row.pug')
	let markup = template({item: rt})
	res.send(markup)
	//res.sendStatus(200)
})

app.get('/item/:id', (req,res) => {
	console.log(req.body)
	let sql = `SELECT item_id, item_name, category FROM items WHERE item_id=?;`
	db.get(sql, [req.params.id], (err,rows) => {
		if (err)
			throw err
		console.log("Rows: ", rows)
		let template = pug.compileFile('views/includes/table_row.pug')
		let markup = template({item: rows})
		res.send(markup)
	});
})

app.listen(PORT);
console.log("Santis is listening on port: " + PORT);
