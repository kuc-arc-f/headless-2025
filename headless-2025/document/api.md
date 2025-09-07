# API example

***
### List

* curl sample
* your-key: API_KEY 
* content: data type

```
curl -H "Authorization: your-key" http://localhost:8787/api/data/list?content=todo
```

***
* order (option)
* curl sample
* your-key: API_KEY 
* content: data type
* order: asc (created_at ASC ) desc (created_at DESC)

```
curl -H "Authorization: your-key" "http://localhost:8787/api/data/list?content=test1&order=asc"
```

***
* node.js: List

```js
const start = async function() {
  try{
    const response = await fetch("http://localhost:8787/api/data/list?content=todo", {
      method: 'GET',
      headers: {
        'Authorization': 'your-key',
      }
    });
    if (!response.ok) {
      const text = await response.text();
      console.log(text)
      throw new Error('Failed to item');
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

```
curl -H "Authorization: your-key" "http://localhost:8787/api/data/getone?id=33
```

***
### Create

* curl sample
* your-key: API_KEY 
* content: data type
* data: json data

* data.json
```
{
  "content":"test1", 
  "data": "{\"title\": \"test-1\"}"
}
```
* curl
```
curl -X POST -H "Content-Type: application/json" -H "Authorization: your-key" -d @data.json http://localhost:8787/api/data/create 
```
***
* node.js: create

```js

const start = async function() {
  try{
      const item = {
        content: "test1",
        data: JSON.stringify({
          "title": "tit-1",
          "body": "body-1",
        })
      }
      const response = await fetch("http://localhost:8787/api/data/create", {
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

* data.json
```
{
  "id": 1 
}
```
* curl
```
curl -X POST -H "Content-Type: application/json" -H "Authorization: your-key" -d @data.json http://localhost:8787/api/data/delete
```
***
* node.js: delete

```js

const start = async function() {
  try{
      const item = {
        id: 1
      }
      const response = await fetch("http://localhost:8787/api/data/delete", {
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
### Update

* curl sample
* your-key: API_KEY 
* id: id data
* content: data type
* data: json data

* data.json
```
{
  "id": 1,
  "content":"test1", 
  "data": "{\"title\": \"test-update\"}"
}
```
* curl
```
curl -X POST -H "Content-Type: application/json" -H "Authorization: your-key" -d @data.json http://localhost:8787/api/data/update 
```
* node.js: update

```js
const start = async function() {
  try{
      const item = {
        id: 1,
        content: "test1",
        data: JSON.stringify({
          "title": "tit-update-1",
          "body": "body-update-1",
        })
      }
      const response = await fetch("http://localhost:8787/api/data/update", {
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