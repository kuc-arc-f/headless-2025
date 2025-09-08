import ReactDOM from 'react-dom/client'
import React from 'react'
//import App from './App';

function App(){
  return(
  <div className="App">
    home
  </div>
  )
}

ReactDOM.createRoot(document.getElementById('app')).render(
    <App />
)
console.log('createRoot')
