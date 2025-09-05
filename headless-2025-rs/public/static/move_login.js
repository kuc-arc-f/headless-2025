
const start = function(){
  const path = window.location.pathname;
  //console.log(path);
  if(path !== '/login'){
    location.href = '/login';
  }
}
start();