using DotEnv
using HTTP

DotEnv.config( path = "../.env" )
day =  Base.Filesystem.basename( pwd() )
res = HTTP.get( "http://adventofcode.com/" * ENV["year"] * "/day/" * day * "/input",
               cookies=Dict("session"=>ENV["session"]) )
out = open( "input.txt", "w" )
println( out, String(res.body) )
