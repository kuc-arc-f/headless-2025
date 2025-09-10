import ReactDOM from 'react-dom/client'
import React from 'react'
import App from './App';
import { BrowserRouter } from 'react-router-dom'
/*
function App(){
  return(
  <div className="App">
    home
  </div>
  )
}
*/

ReactDOM.createRoot(document.getElementById('app')).render(
    <BrowserRouter>
        <App />
    </BrowserRouter>
)
console.log('createRoot')
