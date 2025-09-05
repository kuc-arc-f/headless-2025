
const dataUtil = {

  hello:function(){
    console.log("#hello");
  },

  getContentArray: function(items){
    const ret = [];

    items.forEach((element) => {
      let row = {id: element}
      //console.log(element);
      ret.push(row);
    });
    return ret;
  },

  getDataItems: function(items: any[]){
    try{
      const ret = [];
      let target = "";
      items.forEach((element) => {
        //console.log("id=", element.id);
        //console.log(element.data);
        target = element.data;
        try{
          target = JSON.parse(element.data);
          element.data = target;
        }catch(e){
          console.error(e);
        }
        ret.push(element);
      });
      return ret;
    }catch(e){
      console.error(e);
      throw new Error("error, getJsonData");
    }
  }

}
export default dataUtil;
