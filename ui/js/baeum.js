setInterval(function(){
  $.ajax({
    url: '/info/all',
    dataType: 'json',
    success:function(data){
      for(var i in data){
        let id = '#' + data[i].id;
        let value = data[i].value;

        $(id).text(value);
      }
    }
  })
},1000);
