# day 1
def d1(c);File.read("i").split("\n").map(&:to_i).combination(c).to_a.select{|p|p.sum==2020}[0].inject(:*);end
d1(2)
d1(3)

# day 2, part 1
File.read("i").split("\n").map{|l|l.split(" ").then{|d|Range.new(*d[0].split("-").map(&:to_i)).member?(d[2].scan(d[1].chop).count)}}.count(&:itself)

# day 2, part 2
# attempt 1
File.read("i").split("\n").map{|l|l.split(" ").then{|d|c=d[1].chop;r=d[0].split("-").map{|i|i.to_i-1};[d[2][r[0]],d[2][r[1]]].one?{|_c|_c==c}}}.count(&:itself)
# attempt 2
File.read("i").split("\n").map{|l|l.split(" ").then{|d|c=d[1].chop;r=d[0].split("-").map{|i|i.to_i-1};(d[2][r[0]]==c)^(d[2][r[1]]==c)}}.count(&:itself)

# day 3, part 1
File.read("i").split("\n").drop(1).reduce([3,0]){|xt,p|p[xt[0]]=="#"?xt[1]=xt[1]+1:0;xt[0]=(xt[0]+3)%31;xt}[1]
