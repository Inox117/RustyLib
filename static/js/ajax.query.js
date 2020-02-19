 function ajaxCallRequest(f_method, f_url, f_data) {
    console.log(f_data);
    $.ajax({
      url: f_url,
      type: f_method,
      contentType: 'application/json; charset=UTF-8',
      dataType: 'json',
      data: f_data,
      async: false,
      success: function(msg) {
        alert(msg)
      }
    });
 }

 function test() {
    alert("HALT ! HAMMER ZEIT !");
 }