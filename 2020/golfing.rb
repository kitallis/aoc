# day 1
def d1(c);File.read("i").split.map(&:to_i).combination(c).to_a.select{|p|p.sum==2020}[0].inject(:*);end
d1(2) # part 1
d1(3) # part 2

# day 2
# part 1
File.read("i").split.map{|l|l.split(" ").then{|d|Range.new(*d[0].split("-").map(&:to_i)).member?(d[2].scan(d[1].chop).count)}}.count(&:itself)

# day 2
# part 2
# attempt 1
File.read("i").split.map{|l|l.split(" ").then{|d|c=d[1].chop;r=d[0].split("-").map{|i|i.to_i-1};[d[2][r[0]],d[2][r[1]]].one?{|_c|_c==c}}}.count(&:itself)
# attempt 2
File.read("i").split.map{|l|l.split(" ").then{|d|c=d[1].chop;r=d[0].split("-").map{|i|i.to_i-1};(d[2][r[0]]==c)^(d[2][r[1]]==c)}}.count(&:itself)

# day 3
# part 1
File.read("i").split.drop(1).reduce([3,0]){|xt,p|p[xt[0]]=="#"?xt[1]=xt[1]+1:0;xt[0]=(xt[0]+3)%31;xt}[1]

# day 3
# part 2
def d3(r,d);File.read("i").split.each_with_index.reduce([r,0]){|xt,(p,i)|next(xt)if i%d==1||i==0;p[xt[0]]=="#"?xt[1]=xt[1]+1:0;xt[0]=(xt[0]+r)%31;xt}[1];end;[[1,1],[3,1],[5,1],[7,1],[1,2]].map{|e|d3(*e)}.inject(:*)
