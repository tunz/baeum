
$(document).ready(function(){

    var infos = { data: [{t: 0, total_node: 0}],
                  max: 0,
                  t: 0 };

    infos.update = function (d) {
        if (d['total_node'] > this.max)
            this.max = d['total_node'];
        this.t = d['t'];
        this.data.push(d);
    };

    var svg = d3.select("svg"),
        margin = {top: 20, right: 20, bottom: 20, left: 40},
        width = +svg.attr("width") - margin.left - margin.right,
        height = +svg.attr("height") - margin.top - margin.bottom,
        g = svg.append("g").attr("transform", "translate(" + margin.left + "," + margin.top + ")");

    var x = d3.scaleLinear()
        .range([0, width]);

    var y = d3.scaleLinear()
        .range([height, 0]);

    var line = d3.line()
        .x(function(d) { return x(d['t']); })
        .y(function(d) { return y(d['total_node']); });

    g.append("defs").append("clipPath")
        .attr("id", "clip")
      .append("rect")
        .attr("width", width)
        .attr("height", height);

    g.append("g")
        .attr("class", "axis axis--x")
        .attr("transform", "translate(0," + y(0) + ")")
        .call(d3.axisBottom(x));

    g.append("g")
        .attr("class", "axis axis--y")
        .call(d3.axisLeft(y));

    g.append("g")
        .attr("clip-path", "url(#clip)")
      .append("path")
        .datum(infos.data)
        .attr("class", "line")
      .transition()
        .duration(1000)
        .ease(d3.easeLinear)
        .on("start", tick);

    function tick() {
      y = y.domain([0, infos.max*4/3]);
      svg.selectAll("g .axis--y").call(d3.axisLeft(y));
      x = x.domain([0, infos.t]);
      svg.selectAll("g .axis--x").call(d3.axisBottom(x));

      // Redraw the line.
      d3.select(this)
          .attr("d", line)
          .attr("transform", null);

      // Slide it to the left.
      d3.active(this)
        .transition()
          .on("start", tick);
    }

    function load_infos() {
      $.ajax({
        url: '/plot/' + infos.data.length,
        dataType: 'json',
        success:function(data){
          for(var i in data){
            infos.update(data[i]);
          }
        }
      });
    }

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

    load_infos();
    setInterval(load_infos, 60000);

});
