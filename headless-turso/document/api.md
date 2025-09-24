# API example

***
### List

* your-key: API_KEY 
* content: data type

***
* javascript: List

```js
const start = async function() {
  try{
    const response = await fetch("http://localhost:3000/api/data/list?content=todo&order=asc", {
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

* your-key: API_KEY 
* content: data type
* id: id data

```


const start = async function() {
  try{
    const response = await fetch("http://localhost:3000/api/data/getone?content=todo&id=5", {
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

* your-key: API_KEY 
* content: data type
* data: json data

***
* javascript: create

```js

const start = async function() {
  try{
      const item = {
        content: "todo",
        data: JSON.stringify({
          "title": "tit24",
          "body": "body24",
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
    }else{
      console.log("OK");
      const json = await response.json();
      console.log(json);
    }
  }catch(e){console.log(e)}
}
start();


```
***
### Delete

* your-key: API_KEY 
* id: id data
* content: data type

***
* javascript: delete

```js

const start = async function() {
  try{
      const item = {
        content: "todo",
        id: 3
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
    }else{
      console.log("OK");
      const json = await response.json();
      console.log(json);
    }
  }catch(e){console.log(e)}
}
start();

```

***
### Update

* your-key: API_KEY 
* id: id data
* content: data type
* data: json data

* javascript: update

```js

const start = async function() {
  try{
      const item = {
        id: 4,
        content: "todo",
        data: JSON.stringify({
          "title": "tit-up-24",
          "body": "body-up-24",
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