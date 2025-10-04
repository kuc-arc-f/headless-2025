# API example

***
### List

* curl sample
* your-key: API_KEY 
* content: data type

```
curl -H "Authorization: your-key" http://localhost:3000/api/data/list?content=todo
```
***
* order (option)
* curl sample
* your-key: API_KEY 
* content: data type
* order: asc (created_at ASC ) desc (created_at DESC)

```
curl -H "Authorization: your-key" "http://localhost:3000/api/data/list?content=test1&order=asc"
```

***
* node.js: List

```js
const start = async function() {
  try{
    const response = await fetch("http://localhost:3000/api/data/list?content=test1", {
      method: 'GET',
      headers: {
        'Authorization': 'your-key',
      }
    });
    if (!response.ok) {
      const text = await response.text();
      console.log(text)
      throw new Error('Failed to create item');
    }
    const json = await response.json();
    console.log(json)
  }catch(e){console.log(e)}
}
start();
```
***
### GetOne

* curl sample
* your-key: API_KEY 
* id: id data
* content: data type

```js
const start = async function() {
  try{
    const response = await fetch("http://localhost:3000/api/data/getone?content=todo&id=3", {
      method: 'GET',
      headers: {
        'Authorization': 'your-key',
      }
    });
    if (!response.ok) {
      const text = await response.text();
      console.log(text)
      throw new Error('Failed to create item');
    }
    const json = await response.json();
    console.log(json)
  }catch(e){console.log(e)}
}
start();
```

***
### Create

* curl sample
* your-key: API_KEY 
* content: data type
* data: json data

***
* node.js: create

```js

const start = async function() {
  try{
      const item = {
        content: "test2",
        data: JSON.stringify({
          "title": "tit-22",
          "body": "body-22",
        })
      }
      const response = await fetch("http://localhost:3000/api/data/create", {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'your-key',
      },
      body: JSON.stringify(item),
    });
    if (!response.ok) {
      const text = await response.text();
      console.log(text);
      throw new Error('Failed to create item');
    }
    return response.json();
  }catch(e){console.log(e)}
}
start();

```
***
### Delete

* curl sample
* your-key: API_KEY 
* id: id data
* content: data type

***
* node.js: delete

```js
const start = async function() {
  try{
      const item = {
        content: "todo",
        id: 2
      }
      const response = await fetch("http://localhost:3000/api/data/delete", {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'your-key',
      },
      body: JSON.stringify(item),
    });
    if (!response.ok) {
      const text = await response.text();
      console.log(text);
      throw new Error('Failed to create item');
    }
    return response.json();
  }catch(e){console.log(e)}
}
start();

```

***
### Update

* curl sample
* your-key: API_KEY 
* id: id data
* content: data type
* data: json data

* node.js: update

```js

const start = async function() {
  try{
      const item = {
        id: 14,
        content: "test2",
        data: JSON.stringify({
          "title": "tit-update-14",
          "body": "body-update-14",
        })
      }
      const response = await fetch("http://localhost:3000/api/data/update", {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'your-key',
      },
      body: JSON.stringify(item),
    });
    if (!response.ok) {
      const text = await response.text();
      console.log(text);
      throw new Error('Failed to item');
    }
    return response.json();
  }catch(e){console.log(e)}
}
start();

```
***